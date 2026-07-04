use ::core::slice;

use crate::{
	ffi::unix::xkbcommon::x11 as ffi, //
	mem,
	spec,
};

// NOTE: These are the bindings (prefixed xcb_xkb_) from the XCB extension for XKB (libxcb-xkb) that are
// required to implement the X11-only code path. Note that the xkbcommon::x11 submodule imported above has
// additional symbols (prefixed xkb_x11_) for the X11 implementation but also re-exports XKB symbols used
// to implement the Wayland side, which is why it is renamed as ffi.
use crate::ffi::unix::x11::xkb as x11;

const INVALID_KEY: Key = Key {
	keycode: 0,
	symbol: spec::LogicalKey::Unknown,
};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Key {
	keycode: u32,
	symbol: spec::LogicalKey,
}

pub type Keysym = ffi::xkb_keysym_t;

#[derive(Debug, PartialEq)]
pub struct KeyboardMetadata {
	context: *mut ffi::xkb_context,
	keymap: *mut ffi::xkb_keymap,
	state: *mut ffi::xkb_state,

	event_offset: u8,
	keycode_offset: u8,
	pressed_key_count: u8,
	pressed_key_list: [Key; spec::MAX_PRESSED_KEYCODE_COUNT],
}

impl KeyboardMetadata {
	pub fn for_wayland(fd: i32, format: u32, size: u32) -> Self {
		unsafe {
			let file = super::File::from(fd);

			let block = file.map(0, size as _, spec::BlockProtection::ReadOnly, spec::BlockSharing::Public).unwrap();

			let c_str = block.as_ptr() as *const i8;

			let context = ffi::xkb_context_new(ffi::XKB_CONTEXT_NO_FLAGS);

			crate::panic_if!(context.is_null());

			let keymap = ffi::xkb_keymap_new_from_string(context, c_str, format, ffi::XKB_KEYMAP_COMPILE_NO_FLAGS);

			crate::panic_if!(keymap.is_null());

			let state = ffi::xkb_state_new(keymap);

			crate::panic_if!(state.is_null());

			let _ = file.unmap(block);

			Self {
				context,
				keymap,
				state,

				event_offset: 0,
				keycode_offset: 8,
				pressed_key_count: 0,
				pressed_key_list: [INVALID_KEY; spec::MAX_PRESSED_KEYCODE_COUNT],
			}
		}
	}

	pub fn for_xcb(connection: *mut x11::xcb_connection_t) -> Self {
		unsafe {
			let context = ffi::xkb_context_new(ffi::XKB_CONTEXT_NO_FLAGS);

			crate::panic_if!(context.is_null());

			let mut major: u16 = ffi::XKB_X11_MIN_MAJOR_XKB_VERSION;

			let mut minor: u16 = ffi::XKB_X11_MIN_MINOR_XKB_VERSION;

			let mut event_offset: u8 = 0;

			let mut error_offset: u8 = 0;

			let info =
				ffi::xkb_x11_setup_xkb_extension(connection, ffi::XKB_X11_MIN_MAJOR_XKB_VERSION, ffi::XKB_X11_MIN_MINOR_XKB_VERSION, 0, &mut major, &mut minor, &mut event_offset, &mut error_offset);

			crate::panic_if!(info == 0);

			let device_id = ffi::xkb_x11_get_core_keyboard_device_id(connection);

			crate::panic_if!(device_id == -1);

			let keymap = ffi::xkb_x11_keymap_new_from_device(context, connection, device_id, ffi::XKB_KEYMAP_COMPILE_NO_FLAGS);

			crate::panic_if!(keymap.is_null());

			let state = ffi::xkb_x11_state_new_from_device(keymap, connection, device_id);

			crate::panic_if!(state.is_null());

			let mask = (x11::XCB_XKB_EVENT_TYPE_STATE_NOTIFY | x11::XCB_XKB_EVENT_TYPE_MAP_NOTIFY) as u16;

			let _ = x11::xcb_xkb_select_events(connection, device_id as _, mask, 0, mask, 0, 0, mem::null());

			Self {
				context,
				keymap,
				state,

				event_offset,
				keycode_offset: 0,
				pressed_key_count: 0,
				pressed_key_list: [INVALID_KEY; spec::MAX_PRESSED_KEYCODE_COUNT],
			}
		}
	}

	#[inline]
	pub const fn is_xkb_event(&self, x11_poll_response: u8) -> bool {
		self.event_offset == x11_poll_response
	}

	#[inline]
	pub const fn pressed_keys(&self) -> u8 {
		self.pressed_key_count
	}

	pub fn insert_pressed_key(&mut self, keycode: u32, symbol: spec::LogicalKey) {
		let keycode = keycode + (self.keycode_offset as u32);

		let slot_list = 0..spec::MAX_PRESSED_KEYCODE_COUNT;

		for slot in slot_list {
			if self.pressed_key_list[slot].keycode == 0 {
				self.pressed_key_count += 1;

				self.pressed_key_list[slot] = Key {
					keycode,
					symbol,
				};

				return;
			}
		}
	}

	pub fn remove_pressed_key(&mut self, keycode: u32) -> spec::LogicalKey {
		let keycode = keycode + (self.keycode_offset as u32);

		let slot_list = 0..spec::MAX_PRESSED_KEYCODE_COUNT;

		for slot in slot_list {
			if self.pressed_key_list[slot].keycode == keycode {
				self.pressed_key_count -= 1;
				self.pressed_key_list[slot].keycode = 0;

				return mem::replace(&mut self.pressed_key_list[slot].symbol, spec::LogicalKey::Unknown);
			}
		}

		spec::LogicalKey::Unknown
	}

	pub unsafe fn update_state_pressed<'xkb>(&self, keycode: u32) -> &'xkb [Keysym] {
		unsafe {
			let keycode = keycode + (self.keycode_offset as u32);

			let _ = ffi::xkb_state_update_key(self.state, keycode, ffi::XKB_KEY_DOWN);

			let mut keysym_list: *const ffi::xkb_keysym_t = mem::null();

			let keysym_count = ffi::xkb_state_key_get_syms(self.state, keycode, &mut keysym_list) as usize;

			// SAFETY: The raw pointer keysym_list points to internal data owned by the XKB state.
			// The fact that it goes out of scope while the slice is alive is correct. However, the
			// slice should be consumed immediately, before any state change.
			slice::from_raw_parts(keysym_list, keysym_count)
		}
	}

	#[inline]
	pub fn update_state_released(&self, keycode: u32) {
		unsafe {
			let keycode = keycode + (self.keycode_offset as u32);

			let _ = ffi::xkb_state_update_key(self.state, keycode, ffi::XKB_KEY_UP);
		}
	}

	#[inline]
	pub fn update_state_reset(&mut self) {
		unsafe {
			ffi::xkb_state_unref(self.state);

			self.state = ffi::xkb_state_new(self.keymap);
		}
	}

	#[inline]
	pub fn update_modifiers(&self, mods_depressed: u32, mods_latched: u32, mods_locked: u32, layout_locked: u32) {
		unsafe {
			ffi::xkb_state_update_mask(self.state, mods_depressed, mods_latched, mods_locked, 0, 0, layout_locked);
		}
	}

	#[inline]
	pub const fn clear_pressed_keys(&mut self) {
		self.pressed_key_count = 0;
		self.pressed_key_list = [INVALID_KEY; spec::MAX_PRESSED_KEYCODE_COUNT];
	}

	pub unsafe fn destroy(&mut self) {
		unsafe {
			if self.state != mem::null() {
				ffi::xkb_state_unref(self.state);
			}

			if self.keymap != mem::null() {
				ffi::xkb_keymap_unref(self.keymap);
			}

			if self.context != mem::null() {
				ffi::xkb_context_unref(self.context);
			}
		}
	}
}
