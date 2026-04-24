use crate::ffi::unix::linux::uapi;

pub mod abi;
pub mod epoll;
pub mod error;
pub mod inputs;
pub mod nr;
pub mod syscall;

pub const STDIN_FILENAME: &str = "/dev/stdin\0";

pub const STDOUT_FILENAME: &str = "/dev/stdout\0";

pub use epoll::Dispatcher;

pub use uapi::time::timespec as TimeSpec;
