use ::core;
use core::{
	clone::Clone, //
	default,
};

use crate::{
	ffi::unix::wayland::client::xdg_shell_protocol as ffi, //
	mem,
	os,
	os::unix,
};

use super::{
	super::xkb, //
	base,
	buffer,
	frame,
	keyboard,
	pointer,
	registry,
	seat,
	surface,
	toplevel,
	utils,
};

pub struct Backend {
	pub display: *mut ffi::wl_display,
	pub registry: *mut ffi::wl_registry,

	pub compositor: *mut ffi::wl_compositor,
	pub surface: *mut ffi::wl_surface,
	pub surface_role: *mut ffi::xdg_surface,
	pub manager: *mut ffi::xdg_wm_base,
	pub inputs: *mut ffi::wl_seat,
	pub display_fd: i32,

	// NOTE: At the moment, shared memory is used exclusively to draw a one-time test pattern during startup.
	// This forces the compositor to map the window before any graphics backend submits its first frame, which
	// is useful for debugging. Thus, a single buffer (self.window_buffer_list[0]) is covering the entire pool,
	// and no buffer management is implemented. If software rendering support is added, this code should be
	// extended to manage a pool of buffer objects (i.e., double- or triple-buffered), handle buffer release
	// events, recreate buffers after window resizes, and synchronize CPU rendering with the compositor before
	// reusing buffers.
	pub shared_memory: *mut ffi::wl_shm,
	pub shared_memory_file: crate::File,
	pub shared_memory_pool: *mut ffi::wl_shm_pool,

	// NOTE: Also notice that the single buffer used here is created with WL_SHM_FORMAT_ARGB8888 by default and,
	// apparently, all compositors are required to support it. See Backend::display_test_pattern() for details.
	// In normal conditions, we should query the advertised formats and select one supported by the compositor.
	pub window: *mut ffi::xdg_toplevel,
	pub window_state: utils::WindowState,
	pub window_frame: *mut ffi::wl_callback,
	pub window_buffer_mask: u8,
	pub window_buffer_list: [*mut ffi::wl_buffer; super::MAX_BUFFER_COUNT],

	pub pending_serial: u32,
	pub pending_extent: utils::Extent,
	pub applied_extent: utils::Extent,

	pub pointer: *mut ffi::wl_pointer,
	pub pointer_serial: u32,
	pub pointer_x: ffi::wl_fixed_t,
	pub pointer_y: ffi::wl_fixed_t,

	pub keyboard: *mut ffi::wl_keyboard,
	pub keyboard_serial: u32,
	pub keyboard_metadata: xkb::KeyboardMetadata,

	pub event_list: os::WindowEventList,
	pub dispatcher_handle: unix::DispatchHandle,
}

impl default::Default for Backend {
	#[inline]
	fn default() -> Self {
		Self::new()
	}
}

impl Backend {
	pub const fn new() -> Self {
		Self {
			display: mem::null(),
			registry: mem::null(),

			compositor: mem::null(),
			surface: mem::null(),
			surface_role: mem::null(),
			manager: mem::null(),
			inputs: mem::null(),
			display_fd: -1,

			shared_memory: mem::null(),
			shared_memory_file: crate::File::new(),
			shared_memory_pool: mem::null(),

			window: mem::null(),
			window_state: utils::WindowState::zeroed(),
			window_frame: mem::null(),
			window_buffer_mask: !0,
			window_buffer_list: [mem::null(); super::MAX_BUFFER_COUNT],

			pending_serial: u32::MAX,
			pending_extent: utils::Extent::zeroed(),
			applied_extent: utils::Extent::zeroed(),

			pointer: mem::null(),
			pointer_serial: u32::MAX,
			pointer_x: 0,
			pointer_y: 0,

			keyboard: mem::null(),
			keyboard_serial: u32::MAX,
			keyboard_metadata: xkb::KeyboardMetadata::zeroed(),

			event_list: os::WindowEventList::empty(),
			dispatcher_handle: unix::DispatchHandle::invalid(),
		}
	}

	#[inline]
	pub const fn from_opaque_ptr<'s>(raw: *mut ()) -> &'s mut Self {
		let self_ptr = raw as *mut Self;

		unsafe { &mut (*self_ptr) }
	}

	#[inline]
	pub const fn into_opaque_ptr(&mut self) -> *mut () {
		(self as *mut Self) as *mut ()
	}

	#[inline]
	pub const fn should_close(&self) -> bool {
		self.window_state.check_bit(utils::WINDOW_STATE_SHOULD_CLOSE_BIT)
	}

	#[inline]
	pub const fn cannot_present(&self) -> bool {
		!self.window_frame.is_null() || (self.window_buffer_mask == 0)
	}

	#[inline]
	pub fn event_count(&self) -> usize {
		self.event_list.as_slice().len()
	}

	pub fn connect_display(&mut self) {
		unsafe {
			self.display = ffi::wl_display_connect(mem::null());

			self.display_fd = ffi::wl_display_get_fd(self.display);

			if self.display.is_null() || (self.display_fd == -1) {
				crate::panic!("Unable to connect the Wayland display");
			}
		}
	}

	pub fn get_display_registry(&mut self) {
		self.registry = ffi::wl_display_get_registry(self.display);

		if self.registry.is_null() {
			crate::panic!("Wayland display registry not available");
		}
	}

	pub fn register_global_listeners(&mut self) {
		let self_ptr = self.into_opaque_ptr();

		if ffi::wl_registry_add_listener(self.registry, &registry::CALLBACKS, self_ptr) == -1 {
			crate::panic!("wl_registry_add_listener() failed");
		}

		unsafe {
			if ffi::wl_display_roundtrip(self.display) == -1 {
				crate::panic!("1st wl_display_roundtrip() call failed");
			}
		}

		// NOTE: If any of the errors below are ever caught, that means
		// the compositor did not fire the registry::global() callback.

		if self.compositor.is_null() {
			crate::panic!("Wayland compositor not available");
		}

		if self.manager.is_null() {
			crate::panic!("Wayland window manager not available");
		}

		if self.inputs.is_null() {
			crate::panic!("Wayland input handler not available");
		}

		if self.shared_memory.is_null() {
			crate::panic!("Wayland shared-memory handler not available");
		}
	}

	pub fn create_window(&mut self) {
		self.surface = ffi::wl_compositor_create_surface(self.compositor);

		self.surface_role = ffi::xdg_wm_base_get_xdg_surface(self.manager, self.surface);

		if self.surface.is_null() || self.surface_role.is_null() {
			crate::panic!("Wayland window surface not available");
		}

		self.window = ffi::xdg_surface_get_toplevel(self.surface_role);

		if self.window.is_null() {
			crate::panic!("Wayland window not available");
		}

		ffi::xdg_toplevel_set_title(self.window, "Wayland-window\0".as_ptr() as _);
	}

	pub fn register_window_listeners(&mut self) {
		let self_ptr = self.into_opaque_ptr();

		if ffi::xdg_wm_base_add_listener(self.manager, &base::CALLBACKS, self_ptr) == -1 {
			crate::panic!("xdg_wm_base_add_listener() failed");
		}

		if ffi::xdg_surface_add_listener(self.surface_role, &surface::CALLBACKS, self_ptr) == -1 {
			crate::panic!("xdg_surface_add_listener() failed");
		}

		if ffi::wl_seat_add_listener(self.inputs, &seat::CALLBACKS, self_ptr) == -1 {
			crate::panic!("wl_seat_add_listener() failed");
		}

		if ffi::xdg_toplevel_add_listener(self.window, &toplevel::CALLBACKS, self_ptr) == -1 {
			crate::panic!("xdg_toplevel_add_listener() failed");
		}

		unsafe {
			if ffi::wl_display_roundtrip(self.display) == -1 {
				crate::panic!("2nd wl_display_roundtrip() call failed");
			}
		}
	}

	pub fn first_commit(&mut self) {
		ffi::wl_surface_commit(self.surface);

		unsafe {
			if ffi::wl_display_roundtrip(self.display) == -1 {
				crate::panic!("3rd wl_display_roundtrip() call failed");
			}
		}
	}

	pub fn stop_input_events(&mut self) {
		if self.window != mem::null() {
			ffi::wl_callback_destroy(self.window_frame);
		}

		if self.pointer != mem::null() {
			ffi::wl_pointer_destroy(self.pointer);
		}

		unsafe {
			self.keyboard_metadata.destroy();
		}

		if self.keyboard != mem::null() {
			ffi::wl_keyboard_destroy(self.keyboard);
		}
	}

	pub fn destroy_buffer_list(&mut self) {
		for buffer in &self.window_buffer_list[..] {
			if buffer.is_null() {
				continue;
			}

			ffi::wl_buffer_destroy(*buffer);
		}

		ffi::wl_shm_pool_destroy(self.shared_memory_pool);
	}

	pub fn destroy_window(&mut self) {
		ffi::xdg_toplevel_destroy(self.window);

		ffi::xdg_surface_destroy(self.surface_role);

		ffi::wl_surface_destroy(self.surface);
	}

	pub fn destroy_registry(&mut self) {
		ffi::wl_shm_destroy(self.shared_memory);

		ffi::wl_seat_destroy(self.inputs);

		ffi::xdg_wm_base_destroy(self.manager);

		ffi::wl_compositor_destroy(self.compositor);

		ffi::wl_registry_destroy(self.registry);
	}

	pub fn disconnect_display(&mut self) {
		unsafe {
			ffi::wl_display_disconnect(self.display);

			let _ = self.shared_memory_file.close();
		}
	}

	pub fn flush_and_dispatch(&self) {
		unsafe {
			let _ = ffi::wl_display_flush(self.display);

			let _ = ffi::wl_display_dispatch(self.display);
		}
	}

	pub fn register_pointer_listeners(&mut self) {
		let self_ptr = self.into_opaque_ptr();

		core::debug_assert!(self.pointer != mem::null());

		if ffi::wl_pointer_add_listener(self.pointer, &pointer::CALLBACKS, self_ptr) == -1 {
			crate::panic!("wl_pointer_add_listener() failed");
		}

		self.flush_and_dispatch();
	}

	pub fn register_keyboard_listeners(&mut self) {
		let self_ptr = self.into_opaque_ptr();

		core::debug_assert!(self.keyboard != mem::null());

		if ffi::wl_keyboard_add_listener(self.keyboard, &keyboard::CALLBACKS, self_ptr) == -1 {
			crate::panic!("wl_keyboard_add_listener() failed");
		}

		self.flush_and_dispatch();
	}

	pub fn register_frame_listeners(&mut self) {
		let self_ptr = self.into_opaque_ptr();

		// NOTE: From this point on, the window cannot present another frame until
		// self.window_frame is destroyed and set back to NULL in the frame::done()
		// callback.
		self.window_frame = ffi::wl_surface_frame(self.surface);

		if ffi::wl_callback_add_listener(self.window_frame, &frame::CALLBACKS, self_ptr) == -1 {
			crate::panic!("wl_callback_add_listener() failed");
		}
	}

	pub fn update_memory_pool(&mut self) {
		if self.shared_memory.is_null() || (self.pending_extent == self.applied_extent) {
			return;
		}

		let pending_pool_size = self.pending_extent.size() * super::FORMAT_CHANNEL_COUNT * super::MAX_BUFFER_COUNT;

		let applied_pool_size = self.applied_extent.size() * super::FORMAT_CHANNEL_COUNT * super::MAX_BUFFER_COUNT;

		if pending_pool_size > applied_pool_size {
			if applied_pool_size > 0 {
				let _ = self.shared_memory_file.resize(pending_pool_size);

				ffi::wl_shm_pool_resize(self.shared_memory_pool, pending_pool_size as _);
			} else {
				self.shared_memory_file = utils::allocate_shared_memory(pending_pool_size);

				self.shared_memory_pool = ffi::wl_shm_create_pool(self.shared_memory, self.shared_memory_file.0.0, pending_pool_size as _);
			}
		}

		self.applied_extent = self.pending_extent.clone();
	}

	pub fn check_surface(&mut self) {
		if self.pending_serial == u32::MAX {
			return;
		}

		ffi::xdg_surface_ack_configure(self.surface_role, self.pending_serial);

		if self.pending_extent != self.applied_extent {
			self.event_list.push_window_resized(self.pointer_x, self.pointer_y, self.pending_extent.width as _, self.pending_extent.height as _, 1.0);
		}

		self.update_memory_pool();

		let region = ffi::wl_compositor_create_region(self.compositor);

		ffi::wl_region_add(region, 0, 0, self.applied_extent.width, self.applied_extent.height);

		ffi::wl_surface_set_input_region(self.surface, region);

		ffi::wl_region_destroy(region);

		self.pending_serial = u32::MAX;
	}

	pub fn display_test_pattern(&mut self) {
		if self.should_close() || self.cannot_present() {
			return;
		}

		self.check_surface();

		let width = self.applied_extent.width;

		let height = self.applied_extent.height;

		let stride = width * (super::FORMAT_CHANNEL_COUNT as i32);

		self.window_buffer_list[0] = ffi::wl_shm_pool_create_buffer(self.shared_memory_pool, 0, width, height, stride, ffi::WL_SHM_FORMAT_ARGB8888);

		crate::panic_if!(self.window_buffer_list[0].is_null());

		if ffi::wl_buffer_add_listener(self.window_buffer_list[0], &buffer::CALLBACKS, self.into_opaque_ptr()) == -1 {
			crate::panic!("wl_buffer_add_listener() failed");
		}

		// NOTE: We are writing directly on the file backing the memory pool.
		utils::draw_test_pattern(&self.shared_memory_file, &self.applied_extent);

		ffi::wl_surface_attach(self.surface, self.window_buffer_list[0], 0, 0);

		ffi::wl_surface_damage_buffer(self.surface, 0, 0, width, height);

		ffi::wl_surface_commit(self.surface);

		self.register_frame_listeners();

		self.flush_and_dispatch();
	}
}
