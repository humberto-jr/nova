use ::core;
use core::{
	marker, //
	ops,
	task,
};

use crate::{
	ffi::unix::wayland::client::protocol as wl, //
	ffi::unix::x11,
};

//
// Windowing system:
//

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

pub trait File: marker::Sized {
	fn open(filename: &str, access: FileAccess) -> FileAccessResult<Self>;

	fn clone(&self) -> FileAccessResult<Self>;

	fn size(&self) -> usize;

	fn write<T: marker::Sized>(&mut self, buf: &[T]) -> u32;

	fn read<T: marker::Sized>(&mut self, buf: &mut [T]) -> u32;

	fn seek(&mut self, from: SeekFrom);

	fn flush(&mut self);

	fn close(self);
}

pub trait DynamicLibrary: marker::Sized {
	fn load(filename: &str) -> FileAccessResult<Self>;

	fn symbol(&self, name: &str) -> *const ();
}
