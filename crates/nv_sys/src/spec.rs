use ::core;
use core::{
	marker, //
	ops,
	result,
	task,
};

use crate::{
	ffi::unix::wayland::client::protocol as wl, //
	ffi::unix::x11,
    host,
	time,
};

//
// Error handling:
//

pub type Result<T> = result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
	None,
	Unknown,

	// NOTE: File access-related errors in a semantic precedence of
	// failures from top to bottom. Where, a FileAccessLost implies
	// access revoked after being granted (i.e., errors during IO).
	FilePathInvalid,
	FileAccessDenied,
	FileAlreadyExists,
	FileNotFound,
	FileAccessLost,
	FileUnknown,
}

//
// Windowing system:
//

pub const WINDOW_TEXT_INPUT_CAPACITY: usize = 128;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowBackend {
	None,

	X11(*mut x11::xcb_connection_t, x11::xproto::xcb_window_t),

	Wayland(*mut wl::wl_display, *mut wl::wl_surface),
}

pub trait Window {
	type Event;

	fn name(&self) -> &'static str;

	fn open(&mut self, x: i32, y: i32, width: u32, height: u32);

	fn is_open(&self) -> bool;

	fn extent(&self) -> (i32, i32, u32, u32);

	fn resize(&mut self, x: i32, y: i32, width: u32, height: u32);

	fn poll_input_events(&mut self) -> u32;

	fn flush_queued_events(&mut self) -> &[Self::Event];

	fn register_task_waker(&mut self, waker: task::Waker);

	fn set_dispatcher(&self, dispatcher: &mut crate::Dispatcher);

	fn raw_window_handle(&self) -> WindowBackend;

	fn close(&mut self);
}

#[derive(Debug, Clone, PartialEq)]
pub enum WindowEvent {
	None,

	WindowResized {
		x: i32,
		y: i32,
		width: u32,
		height: u32,
		scale: f32,
		index: u64,
		timestamp: time::Instant,
	},

	WindowMinimized {
		x: i32,
		y: i32,
		width: u32,
		height: u32,
		scale: f32,
		index: u64,
		timestamp: time::Instant,
	},

	WindowMaximized {
		x: i32,
		y: i32,
		width: u32,
		height: u32,
		scale: f32,
		index: u64,
		timestamp: time::Instant,
	},

	WindowFullScreen {
		width: u32,
		height: u32,
		scale: f32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerMoved {
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerLeftWindow {
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerEnteredWindow {
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerButtonPressed {
		button: PointerButton,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerButtonReleased {
		button: PointerButton,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerHorizontalScroll {
		x: i32,
		y: i32,
		delta_x: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerVerticalScroll {
		x: i32,
		y: i32,
		delta_y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerDiagonalScroll {
		x: i32,
		y: i32,
		delta_x: i32,
		delta_y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerButtonRepeated {
		button: PointerButton,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	KeyboardKeyPressed {
		key: LogicalKey,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	KeyboardKeyReleased {
		key: LogicalKey,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	KeyboardKeyRepeated {
		key: LogicalKey,
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},

	TextInput {
		length: u16,
		buffer: [u8; WINDOW_TEXT_INPUT_CAPACITY],
		index: u64,
		timestamp: time::Instant,
	},

	ShouldClose {
		x: i32,
		y: i32,
		index: u64,
		timestamp: time::Instant,
	},
}

//
// Input system:
//

pub const MAX_PRESSED_KEYCODE_COUNT: usize = 16;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LogicalKey {
	Unknown,

	Character(char),

	// Control keys
	Backspace,
	Tab,
	Enter,
	Escape,
	Insert,
	Delete,

	// System
	PrintScreen,
	Pause,
	Menu,

	// Navigation
	Home,
	End,
	PageUp,
	PageDown,
	ArrowLeft,
	ArrowRight,
	ArrowUp,
	ArrowDown,

	// Function keys
	F1,
	F2,
	F3,
	F4,
	F5,
	F6,
	F7,
	F8,
	F9,
	F10,
	F11,
	F12,
	F13,
	F14,
	F15,
	F16,
	F17,
	F18,
	F19,
	F20,
	F21,
	F22,
	F23,
	F24,

	// Modifiers
	Shift,
	Control,
	Alt,
	AltGraph,
	Super,
	CapsLock,
	NumLock,
	ScrollLock,

	// Media
	VolumeUp,
	VolumeDown,
	VolumeMute,
	MediaPlayPause,
	MediaStop,
	MediaNext,
	MediaPrevious,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PointerButton {
	Unknown,

	Left,
	Middle,
	Right,

	Wheel,

	Forward,
	Backward,

	Other,
}

//
// OS abstractions:
//

pub trait Dispatcher {
	type Event;
	type Descriptor;

	fn new() -> Self;

	fn register(&mut self, list: &[Self::Event], source: &Self::Descriptor, waker: task::Waker);

	fn unregister(&mut self, list: &[Self::Event], source: &Self::Descriptor);

	fn wait_and_dispatch(&mut self, ms_timeout: u32) -> u32;
}

pub trait Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	fn start(routine: Fn, args: In) -> Self;

	fn stop(self) -> Out;
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FileAccess {
	Append,
	ReadOnly,
	WriteOnly,
	ReadAndWrite,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FileAccessResult<T> {
	None,
	Open(T),
	Closed,
	NotFound,
	AccessDenied,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SeekFrom {
	Start(usize),
	End(isize),
	Current(isize),
}

pub trait File {
	fn open(&mut self, filename: &str, access: FileAccess) -> Result<()>;

	fn is_open(&self) -> bool;

	fn size(&self) -> Result<usize>;

	fn write(&mut self, buf: &[host::Byte]) -> Result<usize>;

	fn read(&mut self, buf: &mut [host::Byte]) -> Result<usize>;

	fn seek(&mut self, pos: SeekFrom) -> Result<usize>;

	fn flush(&mut self) -> Result<()>;

	fn close(&mut self) -> Result<()>;
}

pub trait DynamicLibrary: marker::Sized {
	fn load(filename: &str) -> FileAccessResult<Self>;

	fn symbol(&self, name: &str) -> *const ();
}
