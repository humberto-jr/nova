use crate::ffi::unix::linux::arch::x86_uapi_asm_unistd_64 as ffi;

pub const READ: usize = ffi::__NR_read;

pub const WRITE: usize = ffi::__NR_write;

pub const OPEN: usize = ffi::__NR_openat;

pub const CLOSE: usize = ffi::__NR_close;

pub const SEEK: usize = ffi::__NR_lseek;

pub const FILE_STATUS: usize = ffi::__NR_statx;

pub const FILE_SYNC: usize = ffi::__NR_fsync;

pub const GET_TIME: usize = ffi::__NR_clock_gettime;

pub const MEMORY_MAP: usize = ffi::__NR_mmap;

pub const MEMORY_UNMAP: usize = ffi::__NR_munmap;

pub const EPOLL_CREATE: usize = ffi::__NR_epoll_create1;

pub const EPOLL_CONTROL: usize = ffi::__NR_epoll_ctl;

pub const EPOLL_WAIT: usize = ffi::__NR_epoll_wait;
