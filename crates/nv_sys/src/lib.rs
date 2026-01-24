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
