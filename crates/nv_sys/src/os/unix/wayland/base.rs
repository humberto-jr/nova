use ::core;

use crate::ffi::unix::wayland::client::xdg_shell_protocol as ffi;

use super::window;

pub(crate) static CALLBACKS: ffi::xdg_wm_base_listener = ffi::xdg_wm_base_listener {
	ping,
};

unsafe extern "C" fn ping(data: *mut (), manager: *mut ffi::xdg_wm_base, serial: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.manager == manager);

	ffi::xdg_wm_base_pong(backend.manager, serial);
}
