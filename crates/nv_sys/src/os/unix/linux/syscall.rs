use ::core::{
	cmp, //
	marker,
};

use crate::{
	ffi, //
	mem,
	os::unix,
	spec,
};

use super::{
	abi, //
	nr,
};

use super::uapi::{
	asm_generic::fcntl, //
	asm_generic::mman,
	eventpoll,
	stat,
	time,
};

const _NULL: usize = 0;

macro_rules! kernel_result {
	($info:expr) => {{
		if $info > -1 {
			spec::Result::Ok(())
		} else {
			spec::Result::Err(spec::Error::Unknown)
		}
	}};

	($info:expr, $error_map_fn:path) => {{
		if $info > -1 {
			spec::Result::Ok(())
		} else {
			spec::Result::Err($error_map_fn($info))
		}
	}};

	($info:expr, $result:expr) => {{
		if $info > -1 {
			spec::Result::Ok($result)
		} else {
			spec::Result::Err(spec::Error::Unknown)
		}
	}};

	($info:expr, $error_map_fn:path, $result:expr) => {{
		if $info > -1 {
			spec::Result::Ok($result)
		} else {
			spec::Result::Err($error_map_fn($info))
		}
	}};
}

#[inline]
pub fn open(filename: &str, access: spec::FileAccess) -> spec::Result<unix::Descriptor> {
	// NOTE: Requests read and write permissions for user, group, and others. The kernel
	// applies "mode & ~umask" to determine the final permissions, so the actual result
	// depends on the process umask. With the typical default umask of 0o022, the final
	// permissions will be 0o644 (read and write for user, read-only for all else). A
	// umask of 0o000 would grant the full 0o666 to all.
	const RW_MODE: u32 = 0o666;

	if ffi::is_nul_terminated(filename) {
		let (flags, mode) = match access {
			spec::FileAccess::Append => (fcntl::O_APPEND | fcntl::O_NONBLOCK, 0),

			spec::FileAccess::ReadOnly => (fcntl::O_RDONLY | fcntl::O_NONBLOCK, 0),

			spec::FileAccess::WriteOnly => (fcntl::O_WRONLY | fcntl::O_CREAT | fcntl::O_NONBLOCK, RW_MODE),

			spec::FileAccess::ReadAndWrite => (fcntl::O_RDWR | fcntl::O_CREAT | fcntl::O_NONBLOCK, RW_MODE),
		};

		let info = unsafe { abi::syscall4(nr::OPEN, fcntl::AT_FDCWD as _, filename.as_ptr() as _, flags as _, mode as _) };

		kernel_result!(info, spec::Error::from_file_errors, info as _)
	} else {
		spec::Result::Err(spec::Error::FilePathInvalid)
	}
}

#[inline]
pub fn close(fd: unix::Descriptor) -> spec::Result<()> {
	let info = unsafe { abi::syscall1(nr::CLOSE, fd as _) };

	kernel_result!(info, spec::Error::from_file_errors)
}

#[inline]
pub fn write(fd: unix::Descriptor, buf: &[unix::Byte]) -> spec::Result<usize> {
	let info = unsafe { abi::syscall3(nr::WRITE, fd as _, buf.as_ptr() as _, buf.len()) };

	kernel_result!(info, spec::Error::from_file_errors, info as _)
}

#[inline]
pub fn read(fd: unix::Descriptor, buf: &mut [unix::Byte]) -> spec::Result<usize> {
	let info = unsafe { abi::syscall3(nr::READ, fd as _, buf.as_mut_ptr() as _, buf.len()) };

	kernel_result!(info, spec::Error::from_file_errors, info as _)
}

#[inline]
pub fn seek(fd: unix::Descriptor, pos: spec::SeekFrom) -> spec::Result<usize> {
	const SEEK_SET: usize = 0;
	const SEEK_CUR: usize = 1;
	const SEEK_END: usize = 2;

	let (offset, whence) = match pos {
		spec::SeekFrom::Start(offset) => (offset, SEEK_SET),

		spec::SeekFrom::End(offset) => (offset as usize, SEEK_END),

		spec::SeekFrom::Current(offset) => (offset as usize, SEEK_CUR),
	};

	let info = unsafe { abi::syscall3(nr::SEEK, fd as _, offset, whence) };

	kernel_result!(info, spec::Error::from_file_errors, info as _)
}

#[inline]
pub fn flush(fd: unix::Descriptor) -> spec::Result<()> {
	let info = unsafe { abi::syscall1(nr::FILE_SYNC, fd as _) };

	kernel_result!(info, spec::Error::from_file_errors)
}

#[inline]
pub fn file_status(fd: unix::Descriptor) -> spec::Result<stat::statx> {
	let mut status = mem::MaybeUninit::<stat::statx>::uninit();

	let flags = (fcntl::AT_EMPTY_PATH | fcntl::AT_STATX_SYNC_AS_STAT) as usize;

	let mask = stat::STATX_BASIC_STATS as usize;

	// NOTE: Apparently the kernel requires a valid pointer for
	// the path argument even when the AT_EMPTY_PATH bit is set.
	const EMPTY_PATH: *const i8 = ffi::c_str("\0");

	unsafe {
		let info = abi::syscall5(nr::FILE_STATUS, fd as _, EMPTY_PATH as _, flags, mask, status.as_mut_ptr() as _);

		kernel_result!(info, spec::Error::from_file_errors, status.assume_init())
	}
}

#[inline]
pub fn clock_get_time(clock_id: unix::Clock) -> spec::Result<super::TimeSpec> {
	let mut time_spec = mem::MaybeUninit::<super::TimeSpec>::uninit();

	let clock_id = match clock_id {
		unix::Clock::Realtime => time::CLOCK_REALTIME,

		unix::Clock::Monotonic => time::CLOCK_MONOTONIC,

		unix::Clock::ProcessCPUTimeID => time::CLOCK_PROCESS_CPUTIME_ID,

		unix::Clock::ThreadCPUTimeID => time::CLOCK_THREAD_CPUTIME_ID,
	};

	unsafe {
		let info = abi::syscall2(nr::GET_TIME, clock_id as _, time_spec.as_mut_ptr() as _);

		kernel_result!(info, time_spec.assume_init())
	}
}

#[inline]
pub fn memory_map<T: marker::Sized>(addr: usize, count: usize, prot: spec::BlockProtection, fd: unix::Descriptor, offset: usize) -> spec::Result<*mut T> {
	let len = mem::size_of::<T>() * count;

	let prot = match prot {
		spec::BlockProtection::None => mman::PROT_NONE,

		spec::BlockProtection::ReadOnly => mman::PROT_READ,

		spec::BlockProtection::ReadAndWrite => mman::PROT_READ | mman::PROT_WRITE,

		spec::BlockProtection::ReadAndExecute => mman::PROT_READ | mman::PROT_EXEC,

		spec::BlockProtection::All => mman::PROT_READ | mman::PROT_WRITE | mman::PROT_EXEC,
	};

	let flags = if fd == unix::INVALID_DESCRIPTOR {
		mman::MAP_PRIVATE | mman::MAP_ANONYMOUS
	} else {
		mman::MAP_PRIVATE
	};

	let info = unsafe { abi::syscall6(nr::MEMORY_MAP, addr, len, prot as _, flags as _, fd as _, offset) };

	kernel_result!(info, spec::Error::from_mapping_errors, info as _)
}

#[inline]
pub fn memory_unmap<T: marker::Sized>(raw: *mut T, count: usize) -> spec::Result<()> {
	let len = mem::size_of::<T>() * count;

	let info = unsafe { abi::syscall2(nr::MEMORY_UNMAP, raw as _, len) };

	kernel_result!(info, spec::Error::from_unmapping_errors)
}

#[inline]
pub fn epoll_create() -> spec::Result<unix::Descriptor> {
	let info = unsafe { abi::syscall1(nr::EPOLL_CREATE, eventpoll::EPOLL_CLOEXEC as _) };

	kernel_result!(info, spec::Error::from_file_errors, info as _)
}

#[inline]
pub fn epoll_control(epfd: unix::Descriptor, op: u32, fd: unix::Descriptor, event: &eventpoll::epoll_event) -> spec::Result<()> {
	let info = unsafe { abi::syscall4(nr::EPOLL_CONTROL, epfd as _, op as _, fd as _, (event as *const _) as _) };

	kernel_result!(info, spec::Error::from_file_errors)
}

#[inline]
pub fn epoll_wait(epfd: unix::Descriptor, events: &mut [eventpoll::epoll_event], timeout: unix::Time) -> spec::Result<u32> {
	let timeout = if timeout == unix::Time::MAX {
		-1
	} else {
		cmp::min(timeout, i32::MAX as _) as i32
	};

	let info = unsafe { abi::syscall4(nr::EPOLL_WAIT, epfd as _, events.as_mut_ptr() as _, events.len(), timeout as _) };

	kernel_result!(info, spec::Error::from_file_errors, info as _)
}
