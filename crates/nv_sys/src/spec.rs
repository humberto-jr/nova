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
	mem,
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

	AllocatorNotReady,
	AllocatorAlignmentInvalid,
	AllocatorSizeLimitExceeded,
	AllocatorOutOfMemory,
	AllocatorBlockUnknown,

	// NOTE: File access-related errors in a semantic precedence of
	// failures from top to bottom. Where, a FileAccessLost implies
	// access revoked after being granted (e.g., errors during IO).
	FilePathInvalid,
	FileAccessDenied,
	FileAlreadyExists,
	FileNotFound,
	FileNotReady,
	FileAccessLost,
	FileUnknown,

	SymbolNameInvalid,
	SymbolAddressNotFound,

	DispatcherNotReady,
	DispatcherHandleUnknown,
}

//
// Memory allocation:
//

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlockProtection {
	None,
	ReadOnly,
	ReadAndWrite,
	ReadAndExecute,
	All,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BlockSharing {
	Public,
	Private,
}

pub trait AllocatedBlock<T: ?marker::Sized> {
	unsafe fn from_raw(raw: *mut T, count: usize) -> Self;

	unsafe fn into_raw(self) -> (*mut T, usize);

	unsafe fn overwrite(&mut self, index: usize, val: T);
}

pub trait Allocator {
	fn allocate<T>(&mut self, count: usize) -> Result<mem::UninitBlock<T>>
	where
		T: marker::Sized;

	fn reallocate<T, B>(&mut self, count: usize, block: B) -> Result<mem::UninitBlock<T>>
	where
		T: marker::Sized,
		B: AllocatedBlock<T>;

	fn deallocate<T, B>(&mut self, block: B) -> Result<()>
	where
		T: marker::Sized,
		B: AllocatedBlock<T>;
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

	fn set_dispatcher(&mut self, dispatcher: &mut crate::Dispatcher);

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

	WindowRestored {
		x: i32,
		y: i32,
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
		delta_x: f32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerVerticalScroll {
		x: i32,
		y: i32,
		delta_y: f32,
		index: u64,
		timestamp: time::Instant,
	},

	PointerDiagonalScroll {
		x: i32,
		y: i32,
		delta_x: f32,
		delta_y: f32,
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JoystickButton {
	Unknown,

	Up,
	Left,
	Right,
	Down,

	Start,
	Select,

	LeftTrigger,
	RightTrigger,

	Button,
}

//
// OS abstractions:
//

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DispatchEvent {
	None,
	Error,

	Readable,
	Writable,
	Closed,

	TimerExpired,

	WindowInput,

	UnixSignal(i32),
	WindowsCtrlSignal(u32),
}

pub trait Dispatcher {
	// NOTE: The default dispatcher behavior is for one-shot firings of handlers when events occur, where
	// the Rust waker mechanism is used as handler here. This reflects modern aspects of async design in
	// which persistent reaction to events is an emergent behavior driven by tasks and runtime executors.
	// It greatly reduces complications of handler ownership and lifetime management within the dispatcher.
	// The consequence is that tasks are now responsible for persistent handling, resubmitting a new waker
	// for previously registered events, whereas the dispatcher's responsibility is to make the resubmission
	// procedure as efficient as possible.
	type Event;
	type Handle;
	type Descriptor;

	fn register(&mut self, list: &[Self::Event], source: &Self::Descriptor, waker: task::Waker) -> Result<Self::Handle>;

	fn update_waker(&mut self, handle: &Self::Handle, waker: task::Waker) -> Result<()>;

	fn unregister(&mut self, handle: &Self::Handle) -> Result<()>;

	fn wait_and_dispatch(&mut self, ms_timeout: host::Time) -> u32;
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
	Temporary,
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

	fn resize(&mut self, len: usize) -> Result<()>;

	fn write(&mut self, buf: &[host::Byte]) -> Result<usize>;

	fn read(&mut self, buf: &mut [host::Byte]) -> Result<usize>;

	fn seek(&mut self, pos: SeekFrom) -> Result<usize>;

	fn flush(&mut self) -> Result<()>;

	fn map(&self, offset: usize, size: usize, prot: BlockProtection, vis: BlockSharing) -> Result<mem::Block<host::Byte>>;

	fn unmap(&self, block: mem::Block<host::Byte>) -> Result<()>;

	fn close(&mut self) -> Result<()>;
}

pub trait DynamicLibrary {
	type Address;

	fn load(&mut self, filename: &str) -> Result<()>;

	fn is_loaded(&self) -> bool;

	fn find_symbol(&self, name: &str) -> Result<Self::Address>;
}
