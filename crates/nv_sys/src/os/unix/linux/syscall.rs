#[cfg(target_arch = "x86")]
use crate::ffi::unix::linux::arch::x86_uapi_asm_unistd_32 as ffi;

#[cfg(target_arch = "x86_64")]
use crate::ffi::unix::linux::arch::x86_uapi_asm_unistd_64 as ffi;

pub const READ: usize = ffi::__NR_read;
pub const WRITE: usize = ffi::__NR_write;
pub const OPEN: usize = ffi::__NR_openat;
pub const CLOSE: usize = ffi::__NR_close;
pub const DUP: usize = ffi::__NR_dup;
