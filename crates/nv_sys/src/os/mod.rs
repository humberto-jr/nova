mod impls;

pub mod unix;

//
// EventBuffer and WindowEventList:
//
// NOTE: The usage of certain methods implemented here may depend on
// conditional compilation, and some may become unused in another
// configuration. Thus, we allow dead code, if any.
#[allow(dead_code)]
mod impl_window_event_list;

pub type EventBuffer = [spec::WindowEvent; MAX_WINDOW_EVENT_COUNT];

pub struct WindowEventList {
	back_buffer: u8,
	front_count: u16,
	next_event: sync::SpinLock<u64>,
	event_count: sync::SpinLock<u16>,
	event_list: [EventBuffer; 2],
}

//
// BaseCStr:
//

pub mod impl_base_cstr;

#[derive(Debug, Clone, PartialEq)]
pub struct BaseCStr<const MAX_LEN: usize> {
	len: u16,
	buf: [u8; MAX_LEN],
}

pub type CStr8 = BaseCStr<8>;
pub type CStr16 = BaseCStr<16>;
pub type CStr64 = BaseCStr<64>;
pub type CStr128 = BaseCStr<128>;
pub type CStr256 = BaseCStr<256>;
pub type CStr512 = BaseCStr<512>;
pub type CStr1024 = BaseCStr<1024>;
