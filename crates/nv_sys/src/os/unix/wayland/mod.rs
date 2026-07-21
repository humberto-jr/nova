use ::core::{
	ops, //
	task,
};

use crate::{
	mem, //
	spec,
};

mod base;
mod buffer;
mod frame;
mod keyboard;
mod pointer;
mod registry;
mod seat;
mod surface;
mod toplevel;
mod utils;
mod window;

const MAX_BUFFER_COUNT: usize = 1;

const FORMAT_CHANNEL_COUNT: usize = 4;

// NOTE: Wayland does rely on the compositor owning raw pointers of the window Backend.
// We need to make sure Rust won't move the Backend in memory and, therefore, it is kept
// in the heap while the outer wrapper can be moved freely. If creating many windows
// becomes a bottleneck later, we can implement a pool of backend windows with a single
// allocation at startup. A static list was discarded to avoid an upper limit of windows
// that could be opened.
#[repr(transparent)]
pub struct Window(mem::Block<window::Backend>);

impl Window {
	pub fn new() -> Self {
		let result = mem::map(1, spec::BlockProtection::ReadAndWrite, spec::BlockSharing::Private);

		if let spec::Result::Ok(uninit_block) = result {
			Self(uninit_block.clear())
		} else {
			let size = mem::size_of::<Self>();

			crate::panic!("Unable to allocate {size} bytes during Wayland-window creation");
		}
	}
}

impl ops::Drop for Window {
	fn drop(&mut self) {
		spec::Window::close(self);

		let self_ptr = self.0.as_mut_ptr();

		let _ = mem::unmap((self_ptr, 1));
	}
}

impl spec::Window for Window {
	type Event = spec::WindowEvent;

	fn name(&self) -> &'static str {
		"Wayland-window"
	}

	fn open(&mut self, _x: i32, _y: i32, width: u32, height: u32) {
		if self.is_open() {
			return;
		}

		let backend = self.deref_backend_mut();

		// NOTE: In case this window has been closed, we must clean any leftovers before reopening.
		*backend = window::Backend::new();

		backend.pending_extent = utils::Extent {
			x: 0,
			y: 0,
			width: width as i32,
			height: height as i32,
		};

		backend.connect_display();

		backend.get_display_registry();

		backend.register_global_listeners();

		backend.create_window();

		backend.register_window_listeners();

		backend.first_commit();

		// NOTE: Apparently, Wayland only maps the window if actual content is drawn into it,
		// whether the initial setup is correct or not. However, it may take some time between
		// opening the window here and the first GPU-driven frame while the window would remain
		// invisible. Thus, mostly for debugging purposes, we draw a CPU-driven test pattern
		// just to force the window to be mapped. This is only used here at startup.
		backend.display_test_pattern();
	}

	fn is_open(&self) -> bool {
		let backend = self.deref_backend();

		(backend.display != mem::null()) && (backend.surface != mem::null()) && (backend.window != mem::null())
	}

	fn extent(&self) -> (i32, i32, u32, u32) {
		let backend = self.deref_backend();

		(0, 0, backend.applied_extent.width as _, backend.applied_extent.height as _)
	}

	fn resize(&mut self, _x: i32, _y: i32, _width: u32, _height: u32) {
		// NOTE: Apparently Wayland does not support the usual move and resize operations from the window
		// itself. This is now under the responsibility of the compositor. Thus, the Wayland version of
		// the Window::resize() method is a no-op.
	}

	fn poll_input_events(&mut self) -> u32 {
		let backend = self.deref_backend_mut();

		backend.flush_and_dispatch();

		backend.event_count() as _
	}

	fn flush_queued_events(&mut self) -> &[Self::Event] {
		let backend = self.deref_backend_mut();

		backend.event_list.swap_back_buffer();

		backend.event_list.as_slice()
	}

	fn set_dispatcher(&mut self, dispatcher: &mut crate::Dispatcher) {
		let waker = self.waker();

		let backend = self.deref_backend_mut();

		if backend.dispatcher_handle.is_valid() {
			let _ = dispatcher.update_waker(&backend.dispatcher_handle, waker);
		} else {
			let list = [spec::DispatchEvent::Readable];

			let result = dispatcher.register(&list[..], &backend.display_fd, waker);

			if let spec::Result::Ok(handle) = result {
				backend.dispatcher_handle = handle;
			}
		}
	}

	fn raw_window_handle(&self) -> spec::WindowBackend {
		let backend = self.deref_backend();

		spec::WindowBackend::Wayland(backend.display, backend.surface)
	}

	fn close(&mut self) {
		if self.is_open() {
			let backend = self.deref_backend_mut();

			backend.stop_input_events();

			backend.destroy_buffer_list();

			backend.destroy_window();

			backend.destroy_registry();

			backend.disconnect_display();
		}
	}
}

impl Window {
	#[inline(always)]
	fn deref_backend(&self) -> &window::Backend {
		&self.0[0u8]
	}

	#[inline(always)]
	fn deref_backend_mut(&mut self) -> &mut window::Backend {
		&mut self.0[0u8]
	}

	fn waker(&self) -> task::Waker {
		// NOTE: We have attempted to implement the prepared-read mechanism of Wayland.
		// The approach was to begin preparing the read during the Dispatcher setup (and
		// update) and to end when the Dispatcher would have fired the waker. Thus, the
		// event list could have been populated asynchronously in the background. However,
		// even making sure the beginning and ending would happen in the same thread,
		// various issues and apparently deadlocks happened. Therefore, we now wait for
		// the Dispatcher to fire and use the blocking flush_and_dispatch() method, as
		// seen below. It still happens in the background and should block for the least
		// amount of time since the display file descriptor is already ready for readings.
		let self_ptr = (self as *const _) as *const ();

		fn clone(self_ptr: *const ()) -> task::RawWaker {
			task::RawWaker::new(self_ptr, &VTABLE)
		}

		fn wake(self_ptr: *const ()) {
			let self_ref = unsafe {
				let self_ptr = self_ptr as *mut Window;

				&mut (*self_ptr)
			};

			self_ref.deref_backend().flush_and_dispatch();
		}

		fn wake_by_ref(self_ptr: *const ()) {
			let self_ref = unsafe {
				let self_ptr = self_ptr as *mut Window;

				&mut (*self_ptr)
			};

			self_ref.deref_backend().flush_and_dispatch();
		}

		fn drop(_: *const ()) {}

		const VTABLE: task::RawWakerVTable = task::RawWakerVTable::new(clone, wake, wake_by_ref, drop);

		unsafe { task::Waker::new(self_ptr, &VTABLE) }
	}
}
