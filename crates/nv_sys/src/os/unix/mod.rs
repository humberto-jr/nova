use crate::spec;

mod impl_dynamic_library;
mod impl_file;
mod utils;
mod xkb;

//
// Base OS:
//

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
use linux as base;

pub use base::inputs;

//
// Definitions:
//

pub type Byte = u8;

pub type Time = u64;

pub type Descriptor = i32;

pub type OpaquePtr = *mut ();

pub struct File(pub Descriptor);

pub struct DynamicLibrary(pub OpaquePtr);

pub use base::Dispatcher;

pub use utils::DispatchHandle;

pub const POSIX_FAILURE_FLAG: isize = -1;

pub const INVALID_DESCRIPTOR: Descriptor = POSIX_FAILURE_FLAG as _;

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

	let result = file.open(base::STDIN_FILENAME, spec::FileAccess::ReadOnly);

	if let spec::Result::Err(info) = result {
		crate::panic!("Failed to open the stdin at \"{}\" ({:?})", base::STDIN_FILENAME, info);
	} else {
		file
	}
}

pub fn stdout() -> crate::File {
	let mut file = crate::File::new();

	let result = file.open(base::STDOUT_FILENAME, spec::FileAccess::WriteOnly);

	if let spec::Result::Err(info) = result {
		crate::panic!("Failed to open the stdout at \"{}\" ({:?})", base::STDOUT_FILENAME, info);
	} else {
		file
	}
}
