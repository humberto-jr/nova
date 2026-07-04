#![no_implicit_prelude]
#![cfg_attr(not(feature = "use_std"), no_std)]

use ::core::ops;

pub extern crate nv_ffi;
pub use nv_ffi as ffi;

pub mod bit;
pub mod gpu;
pub mod macros;
pub mod mem;
pub mod simd;
pub mod spec;
pub mod sync;
pub mod time;

//
// Host platform:
//

mod os;

#[cfg(target_os = "linux")]
use os::unix as host;

//
// Basic types:
//

pub use host::Byte;

pub use host::Time;

pub use host::OpaquePtr;

pub use spec::Error;

pub use spec::Result;

pub use spec::LogicalKey;

pub use spec::PointerButton;

pub use spec::JoystickButton;

//
// Windowing system:
//

pub use spec::WindowBackend;

pub use spec::WindowEvent;

pub struct Window(host::Window);

impl ops::Deref for Window {
	type Target = dyn spec::Window<Event = spec::WindowEvent>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for Window {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Window {
	#[inline(always)]
	pub fn new() -> Self {
		Self(host::Window::new())
	}
}

//
// File:
//

pub use host::stdin;

pub use host::stdout;

pub use spec::FileAccess;

pub use spec::SeekFrom;

pub struct File(host::File);

impl ops::Deref for File {
	type Target = dyn spec::File;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for File {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl File {
	#[inline(always)]
	pub const fn new() -> Self {
		Self(host::File::new())
	}
}

//
// DynamicLibrary:
//

pub struct DynamicLibrary(host::DynamicLibrary);

impl ops::Deref for DynamicLibrary {
	type Target = dyn spec::DynamicLibrary<Address = host::OpaquePtr>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for DynamicLibrary {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl DynamicLibrary {
	#[inline(always)]
	pub const fn new() -> Self {
		Self(host::DynamicLibrary::new())
	}
}

//
// Thread:
//

pub struct Thread<Fn, In, Out>(host::Thread<Fn, In, Out>)
where
	Fn: ops::FnOnce(In) -> Out;

impl<Fn, In, Out> Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	#[inline(always)]
	pub fn start(routine: Fn, args: In) -> Self {
		let inner = <host::Thread<Fn, In, Out> as spec::Thread<Fn, In, Out>>::start(routine, args);

		Self(inner)
	}

	#[inline(always)]
	pub fn stop(self) -> Out {
		spec::Thread::<Fn, In, Out>::stop(self.0)
	}
}

//
// Dispatcher:
//

pub use spec::DispatchEvent;

pub use host::DispatchHandle;

pub struct Dispatcher(host::Dispatcher);

pub type ResourceDescriptor = <host::Dispatcher as spec::Dispatcher>::Descriptor;

impl ops::Deref for Dispatcher {
	type Target = dyn spec::Dispatcher<Event = spec::DispatchEvent, Handle = host::DispatchHandle, Descriptor = ResourceDescriptor>;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl ops::DerefMut for Dispatcher {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl Dispatcher {
	#[inline(always)]
	pub fn new() -> Self {
		Self(host::Dispatcher::new())
	}
}

//
// Termination:
//

pub use host::exit_thread;

pub use host::exit_process;
