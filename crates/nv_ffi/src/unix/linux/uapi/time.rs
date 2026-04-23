pub use super::time_types::*;

pub const ITIMER_REAL: u32 = 0;
pub const ITIMER_VIRTUAL: u32 = 1;
pub const ITIMER_PROF: u32 = 2;

pub const CLOCK_REALTIME: u32 = 0;
pub const CLOCK_MONOTONIC: u32 = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: u32 = 2;
pub const CLOCK_THREAD_CPUTIME_ID: u32 = 3;
pub const CLOCK_MONOTONIC_RAW: u32 = 4;
pub const CLOCK_REALTIME_COARSE: u32 = 5;
pub const CLOCK_MONOTONIC_COARSE: u32 = 6;
pub const CLOCK_BOOTTIME: u32 = 7;
pub const CLOCK_REALTIME_ALARM: u32 = 8;
pub const CLOCK_BOOTTIME_ALARM: u32 = 9;
pub const CLOCK_SGI_CYCLE: u32 = 10;
pub const CLOCK_TAI: u32 = 11;

pub const MAX_CLOCKS: u32 = 16;

pub const CLOCK_AUX: u32 = 16;
pub const MAX_AUX_CLOCKS: u32 = 8;
pub const CLOCK_AUX_LAST: u32 = 23;

pub const CLOCKS_MASK: u32 = 1;
pub const CLOCKS_MONO: u32 = 1;

pub const TIMER_ABSTIME: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct timespec {
	pub tv_sec: __kernel_old_time_t,

	// NOTE: The type of tv_nsec is a platform-dependent long in
	// the original source. Thus, we use __kernel_long_t here.
	pub tv_nsec: __kernel_long_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct timeval {
	pub tv_sec: __kernel_old_time_t,
	pub tv_usec: __kernel_suseconds_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct itimerspec {
	pub it_interval: timespec,
	pub it_value: timespec,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct itimerval {
	pub it_interval: timeval,
	pub it_value: timeval,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct timezone {
	pub tz_minuteswest: i32,
	pub tz_dsttime: i32,
}
