use ::core;

use crate::{
	ffi::unix::wayland::client::protocol as ffi, //
	mem,
};

use super::window;

pub(crate) static CALLBACKS: ffi::wl_seat_listener = ffi::wl_seat_listener {
	capabilities,
	name,
};

unsafe extern "C" fn capabilities(data: *mut (), seat: *mut ffi::wl_seat, capabilities: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.inputs == seat);

	// NOTE: Apparently the Wayland logic is that if the bit for the corresponding device is set,
	// we should have a corresponding resource pointer for it, then create it if not. If the bit
	// is not set, the device is lost, and we should not have a resource pointer; then destroy it
	// if we still have one.

	if (capabilities & ffi::WL_SEAT_CAPABILITY_POINTER) != 0 {
		if backend.pointer == mem::null() {
			backend.pointer = ffi::wl_seat_get_pointer(backend.inputs);

			backend.register_pointer_listeners();
		}
	} else {
		if backend.pointer != mem::null() {
			ffi::wl_pointer_destroy(backend.pointer);

			backend.pointer = mem::null();
		}
	}

	if (capabilities & ffi::WL_SEAT_CAPABILITY_KEYBOARD) != 0 {
		if backend.keyboard == mem::null() {
			backend.keyboard = ffi::wl_seat_get_keyboard(backend.inputs);

			// NOTE: Unlike pointers, the keyboard callbacks must be set immediately after the
			// keyboard object has been created, because if the display is dispatched in between,
			// then we will miss the keyboard::keymap() callback and the keyboard will remain unusable.
			backend.register_keyboard_listeners();
		}
	} else {
		if backend.keyboard != mem::null() {
			ffi::wl_keyboard_destroy(backend.keyboard);

			backend.keyboard = mem::null();
		}
	}
}

unsafe extern "C" fn name(_data: *mut (), _seat: *mut ffi::wl_seat, _name: *const i8) {}
