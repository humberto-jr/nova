use super::super::x11::xcb_connection_t;

pub use super::*;

pub type xkb_x11_setup_xkb_extension_flags = u32;
pub const XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS: xkb_x11_setup_xkb_extension_flags = 0;

#[link(name = "xkbcommon")]
unsafe extern "C" {
	pub fn xkb_x11_setup_xkb_extension(
		connection: *mut xcb_connection_t,
		major_xkb_version: u16,
		minor_xkb_version: u16,
		flags: xkb_x11_setup_xkb_extension_flags,
		major_xkb_version_out: *mut u16,
		minor_xkb_version_out: *mut u16,
		base_event_out: *mut u8,
		base_error_out: *mut u8,
	) -> i32;

	pub fn xkb_x11_get_core_keyboard_device_id(connection: *mut xcb_connection_t) -> i32;

	pub fn xkb_x11_keymap_new_from_device(context: *mut xkb_context, connection: *mut xcb_connection_t, device_id: i32, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_x11_state_new_from_device(keymap: *mut xkb_keymap, connection: *mut xcb_connection_t, device_id: i32) -> *mut xkb_state;
}
