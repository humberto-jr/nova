use ::core;

use crate::{
	ffi::unix::wayland::client::protocol as ffi, //
	spec,
};

use super::{
	super::xkb, //
	utils,
	window,
};

//
// Callbacks:
//

pub(crate) static CALLBACKS: ffi::wl_keyboard_listener = ffi::wl_keyboard_listener {
	keymap,
	enter,
	leave,
	key,
	modifiers,
	repeat_info,
};

unsafe extern "C" fn keymap(data: *mut (), keyboard: *mut ffi::wl_keyboard, format: u32, fd: i32, size: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.keyboard == keyboard);

	backend.keyboard_metadata = xkb::KeyboardMetadata::for_wayland(fd, format, size);
}

unsafe extern "C" fn enter(data: *mut (), keyboard: *mut ffi::wl_keyboard, serial: u32, surface: *mut ffi::wl_surface, keys: *mut ffi::wl_array) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.keyboard == keyboard);
	core::debug_assert!(backend.surface == surface);

	backend.keyboard_serial = serial;
	backend.keyboard_metadata.clear_pressed_keys();

	let keycode_list: &[u32] = utils::wl_array_slice(keys);

	for keycode in keycode_list {
		let keysym_list = unsafe { backend.keyboard_metadata.update_state_pressed(*keycode) };

		for keysym in keysym_list {
			let symbol = spec::LogicalKey::from_xkb_keysym(*keysym);

			backend.keyboard_metadata.insert_pressed_key(*keycode, symbol);
			backend.event_list.push_keyboard_key_pressed(symbol, backend.pointer_x, backend.pointer_y);
		}
	}
}

unsafe extern "C" fn leave(data: *mut (), keyboard: *mut ffi::wl_keyboard, serial: u32, surface: *mut ffi::wl_surface) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.keyboard == keyboard);
	core::debug_assert!(backend.surface == surface);

	backend.keyboard_serial = serial;
	backend.keyboard_metadata.update_state_reset();
	backend.keyboard_metadata.clear_pressed_keys();
}

unsafe extern "C" fn key(data: *mut (), keyboard: *mut ffi::wl_keyboard, serial: u32, _time: u32, key: u32, state: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.keyboard == keyboard);

	// NOTE: A note to my future self on the always confusing handling of keyboards. During the key press event,
	// the keycode represents the physical key being pressed on the device. Here, XKB takes the current keyboard
	// state (layout and modifiers) to map the keycode to an intermediate representation, the keysym. Then we map
	// that keysym to our logical key representation. While the key is pressed, the pair (keycode, logical key) is
	// true regardless of whether the keyboard state has changed or not. Here, our logical keys are named symbols.
	// We need to cache this pair until the key release event, when it can be discarded. This is handled by the
	// xkb::KeyboardMetadata object.
	backend.keyboard_serial = serial;

	match state {
		ffi::WL_KEYBOARD_KEY_STATE_PRESSED => {
			let keysym_list = unsafe { backend.keyboard_metadata.update_state_pressed(key) };

			for keysym in keysym_list {
				let symbol = spec::LogicalKey::from_xkb_keysym(*keysym);

				backend.keyboard_metadata.insert_pressed_key(key, symbol);
				backend.event_list.push_keyboard_key_pressed(symbol, backend.pointer_x, backend.pointer_y);
			}
		},

		ffi::WL_KEYBOARD_KEY_STATE_RELEASED => {
			backend.keyboard_metadata.update_state_released(key);

			if backend.keyboard_metadata.pressed_keys() == 0 {
				return;
			}

			let symbol = backend.keyboard_metadata.remove_pressed_key(key);

			backend.event_list.push_keyboard_key_released(symbol, backend.pointer_x, backend.pointer_y);
		},

		_ => {},
	}
}

unsafe extern "C" fn modifiers(data: *mut (), keyboard: *mut ffi::wl_keyboard, serial: u32, mods_depressed: u32, mods_latched: u32, mods_locked: u32, group: u32) {
	let backend = window::Backend::from_opaque_ptr(data);

	core::debug_assert!(backend.keyboard == keyboard);

	backend.keyboard_serial = serial;
	backend.keyboard_metadata.update_modifiers(mods_depressed, mods_latched, mods_locked, group);
}

unsafe extern "C" fn repeat_info(_data: *mut (), _keyboard: *mut ffi::wl_keyboard, _rate: i32, _delay: i32) {
	// TODO: We are not handling key repeat events just yet.
}
