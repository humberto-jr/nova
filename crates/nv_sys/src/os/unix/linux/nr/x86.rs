use crate::ffi::unix::linux::arch::x86_uapi_asm_unistd_32 as ffi;

pub const READ: usize = ffi::__NR_read;

pub const WRITE: usize = ffi::__NR_write;

pub const OPEN: usize = ffi::__NR_openat;

pub const CLOSE: usize = ffi::__NR_close;

pub const SEEK: usize = ffi::__NR_lseek;

pub const FILE_STATUS: usize = ffi::__NR_statx;

pub const FILE_SYNC: usize = ffi::__NR_fsync;

pub const GET_TIME: usize = ffi::__NR_clock_gettime64;

pub const MEMORY_MAP: usize = ffi::__NR_mmap2;

pub const MEMORY_UNMAP: usize = ffi::__NR_munmap;
