use ::core;

use crate::ffi::unix::wayland::client::xdg_shell_protocol as ffi;

use super::window;

pub(crate) static CALLBACKS: ffi::xdg_surface_listener = ffi::xdg_surface_listener {
	configure,
};

unsafe extern "C" fn configure(data: *mut (), surface_role: *mut ffi::xdg_surface, serial: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.surface_role == surface_role);

	backend.pending_serial = serial;
}
