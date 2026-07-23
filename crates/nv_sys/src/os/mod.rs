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
