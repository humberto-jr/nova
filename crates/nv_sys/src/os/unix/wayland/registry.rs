use crate::{
	ffi::CStr, //
	ffi::unix::wayland::client::xdg_shell_protocol as ffi,
};

use super::window;

pub(crate) static CALLBACKS: ffi::wl_registry_listener = ffi::wl_registry_listener {
	global,
	global_remove,
};

unsafe extern "C" fn global(data: *mut (), registry: *mut ffi::wl_registry, name: u32, interface: *const i8, version: u32) {
	unsafe {
		let backend = window::Backend::from_opaque_ptr(data);

		let c_str = CStr::from_ptr(interface);

		match c_str.to_bytes() {
			b"wl_compositor" => {
				if version > 4 {
					(*backend).compositor = ffi::wl_registry_bind(registry, name, &ffi::wl_compositor_interface, 4);
				} else {
					(*backend).compositor = ffi::wl_registry_bind(registry, name, &ffi::wl_compositor_interface, version);
				}
			},

			b"xdg_wm_base" => {
				(*backend).manager = ffi::wl_registry_bind(registry, name, &ffi::xdg_wm_base_interface, 1);
			},

			b"wl_seat" => {
				(*backend).inputs = ffi::wl_registry_bind(registry, name, &ffi::wl_seat_interface, version);
			},

			b"wl_shm" => {
				(*backend).shared_memory = ffi::wl_registry_bind(registry, name, &ffi::wl_shm_interface, 1);
			},

			_ => {},
		}
	}
}

unsafe extern "C" fn global_remove(_data: *mut (), _registry: *mut ffi::wl_registry, _name: u32) {}
