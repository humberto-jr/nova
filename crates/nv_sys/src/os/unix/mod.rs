use ::core::marker;

use crate::spec;

mod utils;
mod xkb;

//
// Base OS:
//

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux as base;

#[cfg(all(target_os = "linux", feature = "use_libc"))]
mod libc_wrapper;

#[cfg(all(target_os = "linux", feature = "use_libc"))]
pub use libc_wrapper::*;

//
// Definitions:
//

pub type Descriptor = i32;

pub type OpaquePtr = *mut ();

pub use utils::DispatchHandle;

pub use base::STDIN_FILENAME;

pub use base::STDOUT_FILENAME;

//
// Windowing system:
//

#[cfg(all(target_os = "linux", feature = "use_x11"))]
mod x11;

#[cfg(all(target_os = "linux", feature = "use_x11"))]
pub use x11::Window;

#[cfg(all(target_os = "linux", not(feature = "use_x11")))]
mod wayland;

#[cfg(all(target_os = "linux", not(feature = "use_x11")))]
pub use wayland::Window;

//
// Utils:
//

pub fn stdin() -> crate::File {
	let mut file = crate::File::new();

	let result = file.open(STDIN_FILENAME, spec::FileAccess::ReadOnly);

	if let spec::Result::Err(info) = result {
		crate::panic!("Failed to open the stdin at \"{STDIN_FILENAME}\" ({:?})", info);
	} else {
		file
	}
}

pub fn stdout() -> crate::File {
	let mut file = crate::File::new();

	let result = file.open(STDOUT_FILENAME, spec::FileAccess::WriteOnly);

	if let spec::Result::Err(info) = result {
		crate::panic!("Failed to open the stdout at \"{STDOUT_FILENAME}\" ({:?})", info);
	} else {
		file
	}
}
