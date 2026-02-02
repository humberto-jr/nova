#![no_implicit_prelude]

use ::core::ops;

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
	#[inline]
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
	#[inline]
	pub const fn new() -> Self {
		Self(host::File::new())
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
	#[inline]
	pub fn start(routine: Fn, args: In) -> Self {
		let inner = <host::Thread<Fn, In, Out> as spec::Thread<Fn, In, Out>>::start(routine, args);

		Self(inner)
	}

	#[inline]
	pub fn stop(self) -> Out {
		spec::Thread::<Fn, In, Out>::stop(self.0)
	}
}
