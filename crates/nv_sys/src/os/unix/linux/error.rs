use crate::{
	ffi::unix::linux::uapi::asm_generic::errno as ffi, //
	spec,
};

impl spec::Error {
	pub const fn from_mapping_errors(val: isize) -> Self {
		match (-val) as u32 {
			ffi::EINVAL => Self::Unknown,

			ffi::EACCES | ffi::ENODEV => Self::AllocatorNotReady,

			ffi::EOVERFLOW => Self::AllocatorSizeLimitExceeded,

			ffi::ENOMEM => Self::AllocatorOutOfMemory,

			ffi::EBADF => Self::AllocatorBlockUnknown,

			_ => Self::Unknown,
		}
	}

	pub const fn from_unmapping_errors(val: isize) -> Self {
		match (-val) as u32 {
			ffi::EINVAL | ffi::ENOMEM => Self::AllocatorBlockUnknown,

			_ => Self::Unknown,
		}
	}

	pub const fn from_file_errors(val: isize) -> Self {
		match (-val) as u32 {
			ffi::ENOTDIR | ffi::EISDIR | ffi::EINVAL | ffi::ENAMETOOLONG => Self::FilePathInvalid,

			ffi::EACCES | ffi::EPERM => Self::FileAccessDenied,

			ffi::EEXIST => Self::FileAlreadyExists,

			ffi::ENOENT => Self::FileNotFound,

			ffi::EAGAIN | ffi::EWOULDBLOCK => Self::FileNotReady,

			ffi::EIO | ffi::ENODEV | ffi::EPIPE | ffi::ESTALE => Self::FileAccessLost,

			ffi::EBADF => Self::FileUnknown,

			_ => Self::Unknown,
		}
	}
}
