use ::core::option;

use crate::{
	ffi::unix::linux::input_event_codes as evdev, //
	ffi::unix::xkbcommon as x11,
	mem,
	spec,
};

pub const STDIN_FILENAME: &str = "/dev/stdin\0";

pub const STDOUT_FILENAME: &str = "/dev/stdout\0";

//
// Inputs:
//

impl spec::PointerButton {
	#[inline]
	pub const fn take(&mut self) -> Self {
		mem::replace(self, Self::Unknown)
	}

	pub const fn from_wayland_button(button: u32) -> Self {
		match button {
			evdev::BTN_LEFT => Self::Left,
			evdev::BTN_MIDDLE => Self::Middle,
			evdev::BTN_RIGHT => Self::Right,

			evdev::BTN_WHEEL => Self::Wheel,

			evdev::BTN_FORWARD => Self::Forward,
			evdev::BTN_BACK => Self::Backward,

			evdev::BTN_MISC => Self::Other,

			_ => Self::Unknown,
		}
	}

	pub const fn from_xcb_button(button: u8) -> Self {
		match button {
			1 => Self::Left,
			2 => Self::Middle,
			3 => Self::Right,

			4 | 5 => Self::Wheel,

			8 => Self::Forward,
			9 => Self::Backward,

			10 => Self::Other,

			_ => Self::Unknown,
		}
	}
}

impl spec::LogicalKey {
	#[inline]
	pub const fn take(&mut self) -> Self {
		mem::replace(self, Self::Unknown)
	}

	pub fn from_xkb_keysym(keysym: x11::xkb_keysym_t) -> Self {
		use x11::keysyms;

		match keysym {
			keysyms::XKB_KEY_BackSpace => return Self::Backspace,
			keysyms::XKB_KEY_Tab => return Self::Tab,
			keysyms::XKB_KEY_Return => return Self::Enter,
			keysyms::XKB_KEY_Escape => return Self::Escape,
			keysyms::XKB_KEY_Insert => return Self::Insert,
			keysyms::XKB_KEY_Delete => return Self::Delete,

			keysyms::XKB_KEY_Home => return Self::Home,
			keysyms::XKB_KEY_End => return Self::End,
			keysyms::XKB_KEY_Page_Up => return Self::PageUp,
			keysyms::XKB_KEY_Page_Down => return Self::PageDown,
			keysyms::XKB_KEY_Left => return Self::ArrowLeft,
			keysyms::XKB_KEY_Right => return Self::ArrowRight,
			keysyms::XKB_KEY_Up => return Self::ArrowUp,
			keysyms::XKB_KEY_Down => return Self::ArrowDown,

			keysyms::XKB_KEY_Print => return Self::PrintScreen,
			keysyms::XKB_KEY_Pause => return Self::Pause,
			keysyms::XKB_KEY_Menu => return Self::Menu,

			keysyms::XKB_KEY_F1 => return Self::F1,
			keysyms::XKB_KEY_F2 => return Self::F2,
			keysyms::XKB_KEY_F3 => return Self::F3,
			keysyms::XKB_KEY_F4 => return Self::F4,
			keysyms::XKB_KEY_F5 => return Self::F5,
			keysyms::XKB_KEY_F6 => return Self::F6,
			keysyms::XKB_KEY_F7 => return Self::F7,
			keysyms::XKB_KEY_F8 => return Self::F8,
			keysyms::XKB_KEY_F9 => return Self::F9,
			keysyms::XKB_KEY_F10 => return Self::F10,
			keysyms::XKB_KEY_F11 => return Self::F11,
			keysyms::XKB_KEY_F12 => return Self::F12,
			keysyms::XKB_KEY_F13 => return Self::F13,
			keysyms::XKB_KEY_F14 => return Self::F14,
			keysyms::XKB_KEY_F15 => return Self::F15,
			keysyms::XKB_KEY_F16 => return Self::F16,
			keysyms::XKB_KEY_F17 => return Self::F17,
			keysyms::XKB_KEY_F18 => return Self::F18,
			keysyms::XKB_KEY_F19 => return Self::F19,
			keysyms::XKB_KEY_F20 => return Self::F20,
			keysyms::XKB_KEY_F21 => return Self::F21,
			keysyms::XKB_KEY_F22 => return Self::F22,
			keysyms::XKB_KEY_F23 => return Self::F23,
			keysyms::XKB_KEY_F24 => return Self::F24,

			keysyms::XKB_KEY_KP_0 => return Self::Character('0'),
			keysyms::XKB_KEY_KP_1 => return Self::Character('1'),
			keysyms::XKB_KEY_KP_2 => return Self::Character('2'),
			keysyms::XKB_KEY_KP_3 => return Self::Character('3'),
			keysyms::XKB_KEY_KP_4 => return Self::Character('4'),
			keysyms::XKB_KEY_KP_5 => return Self::Character('5'),
			keysyms::XKB_KEY_KP_6 => return Self::Character('6'),
			keysyms::XKB_KEY_KP_7 => return Self::Character('7'),
			keysyms::XKB_KEY_KP_8 => return Self::Character('8'),
			keysyms::XKB_KEY_KP_9 => return Self::Character('9'),
			keysyms::XKB_KEY_KP_Enter => return Self::Enter,

			keysyms::XKB_KEY_Shift_L | keysyms::XKB_KEY_Shift_R => return Self::Shift,

			keysyms::XKB_KEY_Control_L | keysyms::XKB_KEY_Control_R => return Self::Control,

			keysyms::XKB_KEY_Alt_L | keysyms::XKB_KEY_Alt_R => return Self::Alt,

			keysyms::XKB_KEY_ISO_Level3_Shift => return Self::AltGraph,

			keysyms::XKB_KEY_Super_L | keysyms::XKB_KEY_Super_R => return Self::Super,

			keysyms::XKB_KEY_Caps_Lock => return Self::CapsLock,
			keysyms::XKB_KEY_Num_Lock => return Self::NumLock,
			keysyms::XKB_KEY_Scroll_Lock => return Self::ScrollLock,

			_ => {},
		}

		let utf32 = unsafe { x11::xkb_keysym_to_utf32(keysym) };

		if utf32 != 0 {
			let result = char::from_u32(utf32);

			if let option::Option::Some(rs_char) = result {
				Self::Character(rs_char)
			} else {
				Self::Unknown
			}
		} else {
			Self::Unknown
		}
	}
}
