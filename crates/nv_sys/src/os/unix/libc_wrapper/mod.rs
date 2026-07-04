use ::core::{
	marker, //
	ops,
};

use crate::{
	ffi::libc as ffi, //
	mem,
	spec,
};

use super::utils;

mod impl_dispatcher;
mod impl_dynamic_library;
mod impl_file;
mod impl_thread;

//
// Definitions:
//

pub type Byte = u8;

pub type Time = u64;

pub struct File(ffi::c_int);

pub struct DynamicLibrary(*mut ffi::c_void);

pub struct Dispatcher {
	handle: ffi::c_int,
	waker_list: utils::DispatchList,
}

pub struct Thread<Fn, In, Out>
where
	Fn: ops::FnOnce(In) -> Out,
{
	handle: ffi::pthread_t,
	task: mem::Block<utils::ThreadTask<Fn, In, Out>>,
}

pub const POSIX_FAILURE_FLAG: ffi::c_int = -1;

pub const INVALID_DESCRIPTOR: ffi::c_int = POSIX_FAILURE_FLAG;

//
// Utils:
//

pub fn monotonic_time() -> super::Time {
	let result = unsafe {
		let mut timespec = mem::MaybeUninit::<ffi::timespec>::uninit();

		let _ = ffi::clock_gettime(ffi::CLOCK_MONOTONIC, timespec.as_mut_ptr());

		timespec.assume_init()
	};

	((result.tv_sec * 1_000_000_000) + result.tv_nsec) as _
}

pub fn memory_map<T: marker::Sized>(count: usize, prot: spec::BlockProtection, vis: spec::BlockSharing) -> spec::Result<*mut T> {
	let size = mem::size_of::<T>() * count;

	let prot = match prot {
		spec::BlockProtection::None => ffi::PROT_NONE,

		spec::BlockProtection::ReadOnly => ffi::PROT_READ,

		spec::BlockProtection::ReadAndWrite => ffi::PROT_READ | ffi::PROT_WRITE,

		spec::BlockProtection::ReadAndExecute => ffi::PROT_READ | ffi::PROT_EXEC,

		spec::BlockProtection::All => ffi::PROT_READ | ffi::PROT_WRITE | ffi::PROT_EXEC,
	};

	let flags = match vis {
		spec::BlockSharing::Public => ffi::MAP_SHARED | ffi::MAP_ANONYMOUS,

		spec::BlockSharing::Private => ffi::MAP_PRIVATE | ffi::MAP_ANONYMOUS,
	};

	unsafe {
		let raw = ffi::mmap(mem::null(), size, prot, flags, INVALID_DESCRIPTOR, 0);

		if raw != ffi::MAP_FAILED {
			spec::Result::Ok(raw as *mut T)
		} else {
			let info = if *ffi::__errno_location() == ffi::ENOMEM {
				spec::Error::AllocatorOutOfMemory
			} else {
				spec::Error::Unknown
			};

			spec::Result::Err(info)
		}
	}
}

pub fn memory_unmap<T: marker::Sized>(raw: *const T, count: usize) -> spec::Result<()> {
	let size = mem::size_of::<T>() * count;

	let info = unsafe { ffi::munmap(raw as _, size) };

	if info != POSIX_FAILURE_FLAG {
		spec::Result::Ok(())
	} else {
		spec::Result::Err(spec::Error::AllocatorBlockUnknown)
	}
}

#[inline]
pub fn exit_thread(_status: i32) -> ! {
	unsafe { ffi::pthread_exit(mem::null()) }
}

#[inline]
pub fn exit_process(status: i32) -> ! {
	unsafe { ffi::exit(status) }
}
