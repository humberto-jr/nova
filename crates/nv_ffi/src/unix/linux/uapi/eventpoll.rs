pub use super::asm_generic::fcntl::*;

pub const EPOLL_CLOEXEC: u32 = O_CLOEXEC;

pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLL_CTL_DEL: u32 = 2;
pub const EPOLL_CTL_MOD: u32 = 3;
pub const EPOLL_IOC_TYPE: u32 = 138;

pub const EPOLLIN: u32 = 0x00000001;
pub const EPOLLPRI: u32 = 0x00000002;
pub const EPOLLOUT: u32 = 0x00000004;
pub const EPOLLERR: u32 = 0x00000008;
pub const EPOLLHUP: u32 = 0x00000010;
pub const EPOLLNVAL: u32 = 0x00000020;
pub const EPOLLRDNORM: u32 = 0x00000040;
pub const EPOLLRDBAND: u32 = 0x00000080;
pub const EPOLLWRNORM: u32 = 0x00000100;
pub const EPOLLWRBAND: u32 = 0x00000200;
pub const EPOLLMSG: u32 = 0x00000400;
pub const EPOLLRDHUP: u32 = 0x00002000;

pub const EPOLLEXCLUSIVE: u32 = 1 << 28;
pub const EPOLLWAKEUP: u32 = 1 << 29;
pub const EPOLLONESHOT: u32 = 1 << 30;
pub const EPOLLET: u32 = 1 << 31;

pub type __poll_t = u32;

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct epoll_event {
	pub events: __poll_t,
	pub data: u64,
}

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct epoll_event {
	pub events: __poll_t,
	pub data: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct epoll_params {
	pub busy_poll_usecs: u32,
	pub busy_poll_budget: u16,
	pub prefer_busy_poll: u8,
	pub __pad: u8,
}
