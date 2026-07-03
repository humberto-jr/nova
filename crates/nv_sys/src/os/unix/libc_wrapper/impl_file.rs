use ::core::ops;

use crate::{
	mem, //
	spec,
	spec::AllocatedBlock,
};

use super::ffi;

macro_rules! libc_result {
	($info:expr, $result:expr) => {{
		if $info != (super::POSIX_FAILURE_FLAG as _) {
			spec::Result::Ok($result)
		} else {
			spec::Result::Err(from_errno())
		}
	}};
}

impl ops::Drop for super::File {
	#[inline]
	fn drop(&mut self) {
		if self.0 != super::INVALID_DESCRIPTOR {
			let _ = unsafe { ffi::close(self.0) };
		}
	}
}

impl spec::File for super::File {
	fn open(&mut self, filename: &str, access: spec::FileAccess) -> spec::Result<()> {
		let flags = match access {
			spec::FileAccess::Append => ffi::O_APPEND | ffi::O_NONBLOCK,

			spec::FileAccess::ReadOnly => ffi::O_RDONLY | ffi::O_NONBLOCK,

			spec::FileAccess::WriteOnly => ffi::O_WRONLY | ffi::O_CREAT | ffi::O_NONBLOCK,

			spec::FileAccess::ReadAndWrite => ffi::O_RDWR | ffi::O_CREAT | ffi::O_NONBLOCK,
		};

		self.0 = unsafe { ffi::open(crate::ffi::c_str!(filename), flags) };

		libc_result!(self.0, ())
	}

	#[inline]
	fn is_open(&self) -> bool {
		self.0 != super::INVALID_DESCRIPTOR
	}

	fn size(&self) -> spec::Result<usize> {
		let mut stat = mem::MaybeUninit::<ffi::stat>::uninit();

		unsafe {
			let info = ffi::fstat(self.0, stat.as_mut_ptr());

			libc_result!(info, stat.assume_init_ref().st_size as usize)
		}
	}

	fn write(&mut self, buf: &[super::Byte]) -> spec::Result<usize> {
		let info = unsafe { ffi::write(self.0, buf.as_ptr() as _, buf.len() as _) };

		libc_result!(info, info as usize)
	}

	fn read(&mut self, buf: &mut [super::Byte]) -> spec::Result<usize> {
		let info = unsafe { ffi::read(self.0, buf.as_mut_ptr() as _, buf.len() as _) };

		libc_result!(info, info as usize)
	}

	fn seek(&mut self, pos: spec::SeekFrom) -> spec::Result<usize> {
		let (offset, whence) = match pos {
			spec::SeekFrom::Start(offset) => (offset as ffi::off_t, ffi::SEEK_SET),

			spec::SeekFrom::End(offset) => (offset as ffi::off_t, ffi::SEEK_END),

			spec::SeekFrom::Current(offset) => (offset as ffi::off_t, ffi::SEEK_CUR),
		};

		let info = unsafe { ffi::lseek(self.0, offset, whence) };

		libc_result!(info, info as usize)
	}

	fn flush(&mut self) -> spec::Result<()> {
		let info = unsafe { ffi::fsync(self.0) };

		libc_result!(info, ())
	}

	fn map(&self, offset: usize, size: usize, prot: spec::BlockProtection, vis: spec::BlockSharing) -> spec::Result<mem::Block<super::Byte>> {
		let prot = match prot {
			spec::BlockProtection::None => ffi::PROT_NONE,

			spec::BlockProtection::ReadOnly => ffi::PROT_READ,

			spec::BlockProtection::ReadAndWrite => ffi::PROT_READ | ffi::PROT_WRITE,

			spec::BlockProtection::ReadAndExecute => ffi::PROT_READ | ffi::PROT_EXEC,

			spec::BlockProtection::All => ffi::PROT_READ | ffi::PROT_WRITE | ffi::PROT_EXEC,
		};

		let flags = match vis {
			spec::BlockSharing::Public => ffi::MAP_SHARED,

			spec::BlockSharing::Private => ffi::MAP_PRIVATE,
		};

		unsafe {
			let raw = ffi::mmap(mem::null(), size, prot, flags, self.0, offset as _);

			libc_result!(raw, mem::Block::from_raw(raw as *mut super::Byte, size))
		}
	}

	fn unmap(&self, block: mem::Block<super::Byte>) -> spec::Result<()> {
		let (raw, size) = unsafe { block.into_raw() };

		let info = unsafe { ffi::munmap(raw as *mut ffi::c_void, size) };

		libc_result!(info, ())
	}

	fn close(&mut self) -> spec::Result<()> {
		let info = unsafe { ffi::close(self.0) };

		self.0 = super::INVALID_DESCRIPTOR;

		libc_result!(info, ())
	}
}

impl super::File {
	#[inline]
	pub const fn new() -> Self {
		Self(super::INVALID_DESCRIPTOR)
	}
}

//
// Internals:
//

fn from_errno() -> spec::Error {
	let errno = unsafe { *ffi::__errno_location() };

	match errno {
		ffi::ENOTDIR | ffi::EISDIR | ffi::EINVAL | ffi::ENAMETOOLONG => spec::Error::FilePathInvalid,

		ffi::EACCES | ffi::EPERM => spec::Error::FileAccessDenied,

		ffi::EEXIST => spec::Error::FileAlreadyExists,

		ffi::ENOENT => spec::Error::FileNotFound,

		#[cfg(target_os = "linux")]
		ffi::EAGAIN => spec::Error::FileNotReady,

		#[cfg(not(target_os = "linux"))]
		ffi::EAGAIN | ffi::EWOULDBLOCK => spec::Error::FileNotReady,

		ffi::EIO | ffi::ENODEV | ffi::EPIPE | ffi::ESTALE => spec::Error::FileAccessLost,

		ffi::EBADF => spec::Error::FileUnknown,

		ffi::ENOMEM => spec::Error::AllocatorOutOfMemory,

		_ => spec::Error::Unknown,
	}
}
