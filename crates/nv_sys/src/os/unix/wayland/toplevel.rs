use ::core;

use crate::ffi::unix::wayland::client::xdg_shell_protocol as ffi;

use super::{
	utils, //
	window,
};

pub(crate) static CALLBACKS: ffi::xdg_toplevel_listener = ffi::xdg_toplevel_listener {
	configure,
	close,
	configure_bounds,
	wm_capabilities,
};

unsafe extern "C" fn configure(data: *mut (), window: *mut ffi::xdg_toplevel, width: i32, height: i32, states: *mut ffi::wl_array) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.window == window);

	// NOTE: A zeroed extent is not an error here. It only means we should keep
	// our current shape. Also, resizing here is allowed, but we are deferring
	// it to the main loop.
	if (width > 0) && (height > 0) {
		backend.pending_extent.width = width;
		backend.pending_extent.height = height;
	}

	if states.is_null() || backend.should_close() {
		return;
	}

	// NOTE: Apparently Wayland sends a complete snapshot of the current state,
	// not an incremental state change. Thus, the old state needs to be reset
	// before parsing the new one.
	backend.window_state.unset_all();

	let state_list: &[ffi::xdg_toplevel_state] = utils::wl_array_slice(states);

	for state in state_list {
		match *state {
			ffi::XDG_TOPLEVEL_STATE_ACTIVATED => {
				backend.window_state.set_bit(utils::WINDOW_STATE_ACTIVATED_BIT);
			},

			ffi::XDG_TOPLEVEL_STATE_MAXIMIZED => {
				backend.event_list.push_window_maximized(backend.pointer_x, backend.pointer_y, backend.applied_extent.width as _, backend.applied_extent.height as _, 1.0);
				backend.window_state.set_bit(utils::WINDOW_STATE_MAXIMIZED_BIT);
			},

			ffi::XDG_TOPLEVEL_STATE_FULLSCREEN => {
				backend.event_list.push_window_full_screen(backend.applied_extent.width as _, backend.applied_extent.height as _, 1.0);
				backend.window_state.set_bit(utils::WINDOW_STATE_FULL_SCREEN_BIT);
			},

			ffi::XDG_TOPLEVEL_STATE_RESIZING => {
				backend.window_state.set_bit(utils::WINDOW_STATE_RESIZING_BIT);
			},

			ffi::XDG_TOPLEVEL_STATE_TILED_LEFT | ffi::XDG_TOPLEVEL_STATE_TILED_RIGHT | ffi::XDG_TOPLEVEL_STATE_TILED_TOP | ffi::XDG_TOPLEVEL_STATE_TILED_BOTTOM => {
				backend.window_state.set_bit(utils::WINDOW_STATE_TILED_BIT);
			},

			_ => {},
		}
	}
}

unsafe extern "C" fn close(data: *mut (), window: *mut ffi::xdg_toplevel) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.window == window);

	backend.event_list.push_should_close(backend.pointer_x, backend.pointer_y);
	backend.window_state.set_bit(utils::WINDOW_STATE_SHOULD_CLOSE_BIT);
}

unsafe extern "C" fn configure_bounds(_data: *mut (), _window: *mut ffi::xdg_toplevel, _width: i32, _height: i32) {}

unsafe extern "C" fn wm_capabilities(_data: *mut (), _window: *mut ffi::xdg_toplevel, _capabilities: *mut ffi::wl_array) {}
