pub type __s8 = i8;
pub type __u8 = u8;
pub type __s16 = i16;
pub type __u16 = u16;
pub type __s32 = i32;
pub type __u32 = u32;
pub type __s64 = i64;
pub type __u64 = u64;

#[cfg(target_pointer_width = "64")]
pub type __kernel_long_t = i64;

#[cfg(target_pointer_width = "32")]
pub type __kernel_long_t = i32;

pub type __kernel_time64_t = i64;

#[cfg(target_pointer_width = "64")]
pub type __kernel_old_time_t = i64;

#[cfg(target_pointer_width = "32")]
pub type __kernel_old_time_t = i32;

#[cfg(target_pointer_width = "64")]
pub type __kernel_suseconds_t = i64;

#[cfg(target_pointer_width = "32")]
pub type __kernel_suseconds_t = i32;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_timespec {
	pub tv_sec: __kernel_time64_t,
	pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_itimerspec {
	pub it_interval: __kernel_timespec,
	pub it_value: __kernel_timespec,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_old_timeval {
	pub tv_sec: __kernel_long_t,
	pub tv_usec: __kernel_long_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_old_timespec {
	pub tv_sec: __kernel_old_time_t,

	// NOTE: The type of tv_nsec is a platform-dependent long in
	// the original source. Thus, we use __kernel_long_t here.
	pub tv_nsec: __kernel_long_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_old_itimerval {
	pub it_interval: __kernel_old_timeval,
	pub it_value: __kernel_old_timeval,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct __kernel_sock_timeval {
	pub tv_sec: __s64,
	pub tv_usec: __s64,
}
