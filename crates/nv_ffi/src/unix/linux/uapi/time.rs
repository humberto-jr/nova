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
