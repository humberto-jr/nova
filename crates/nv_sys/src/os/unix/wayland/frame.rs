use ::core;

use crate::{
	ffi::unix::wayland::client::protocol as ffi, //
	mem,
};

use super::window;

pub(crate) static CALLBACKS: ffi::wl_callback_listener = ffi::wl_callback_listener {
	done,
};

unsafe extern "C" fn done(data: *mut (), frame: *mut ffi::wl_callback, _callback_data: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.window_frame == frame);

	ffi::wl_callback_destroy(backend.window_frame);

	// NOTE: From this point on, the compositor is ready
	// to accept another frame for this window.
	backend.window_frame = mem::null();
}
