pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGBUS: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGUSR1: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGUSR2: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGSTKFLT: u32 = 16;
pub const SIGCHLD: u32 = 17;
pub const SIGCONT: u32 = 18;
pub const SIGSTOP: u32 = 19;
pub const SIGTSTP: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGURG: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGIO: u32 = 29;
pub const SIGPOLL: u32 = 29;
pub const SIGPWR: u32 = 30;
pub const SIGSYS: u32 = 31;
pub const SIGUNUSED: u32 = 31;
pub const SIGRTMIN: u32 = 32;
pub const SIGRTMAX: u32 = 64;

pub const MINSIGSTKSZ: u32 = 2048;
pub const SIGSTKSZ: u32 = 8192;

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct sigset_t {
	pub sig: [u64; 1],
}

#[cfg(target_arch = "x86")]
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct sigset_t {
	pub sig: [u32; 2],
}

pub type old_sigset_t = u64;

pub type __signalfn_t = unsafe extern "C" fn(arg1: i32);

pub type __sighandler_t = __signalfn_t;

pub type __restorefn_t = unsafe extern "C" fn();

pub type __sigrestore_t = __restorefn_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigaction {
	pub sa_handler: __sighandler_t,
	pub sa_flags: u64,
	pub sa_mask: sigset_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct sigaltstack {
	pub ss_sp: *mut (),
	pub ss_flags: i32,
	pub ss_size: usize,
}

pub type stack_t = sigaltstack;
