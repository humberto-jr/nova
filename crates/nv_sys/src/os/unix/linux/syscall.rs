use crate::{
	ffi, //
	ffi::unix::linux::uapi::asm_generic::fcntl,
	mem,
	os::unix,
	spec,
};

use super::{
	abi, //
	nr,
};

const NULL: usize = 0;

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
