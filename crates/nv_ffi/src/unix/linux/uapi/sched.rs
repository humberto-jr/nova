pub const CSIGNAL: u32 = 255;

pub const CLONE_VM: u32 = 256;
pub const CLONE_FS: u32 = 512;
pub const CLONE_FILES: u32 = 1024;
pub const CLONE_SIGHAND: u32 = 2048;
pub const CLONE_PIDFD: u32 = 4096;
pub const CLONE_PTRACE: u32 = 8192;
pub const CLONE_VFORK: u32 = 16384;
pub const CLONE_PARENT: u32 = 32768;
pub const CLONE_THREAD: u32 = 65536;
pub const CLONE_NEWNS: u32 = 131072;
pub const CLONE_SYSVSEM: u32 = 262144;
pub const CLONE_SETTLS: u32 = 524288;
pub const CLONE_PARENT_SETTID: u32 = 1048576;
pub const CLONE_CHILD_CLEARTID: u32 = 2097152;
pub const CLONE_DETACHED: u32 = 4194304;
pub const CLONE_UNTRACED: u32 = 8388608;
pub const CLONE_CHILD_SETTID: u32 = 16777216;
pub const CLONE_NEWCGROUP: u32 = 33554432;
pub const CLONE_NEWUTS: u32 = 67108864;
pub const CLONE_NEWIPC: u32 = 134217728;
pub const CLONE_NEWUSER: u32 = 268435456;
pub const CLONE_NEWPID: u32 = 536870912;
pub const CLONE_NEWNET: u32 = 1073741824;
pub const CLONE_IO: u32 = 2147483648;
pub const CLONE_CLEAR_SIGHAND: u64 = 4294967296;
pub const CLONE_INTO_CGROUP: u64 = 8589934592;
pub const CLONE_NEWTIME: u32 = 128;
pub const CLONE_ARGS_SIZE_VER0: u32 = 64;
pub const CLONE_ARGS_SIZE_VER1: u32 = 80;
pub const CLONE_ARGS_SIZE_VER2: u32 = 88;

pub const SCHED_NORMAL: u32 = 0;
pub const SCHED_FIFO: u32 = 1;
pub const SCHED_RR: u32 = 2;
pub const SCHED_BATCH: u32 = 3;
pub const SCHED_IDLE: u32 = 5;
pub const SCHED_DEADLINE: u32 = 6;
pub const SCHED_EXT: u32 = 7;
pub const SCHED_RESET_ON_FORK: u32 = 1073741824;
pub const SCHED_FLAG_RESET_ON_FORK: u32 = 1;
pub const SCHED_FLAG_RECLAIM: u32 = 2;
pub const SCHED_FLAG_DL_OVERRUN: u32 = 4;
pub const SCHED_FLAG_KEEP_POLICY: u32 = 8;
pub const SCHED_FLAG_KEEP_PARAMS: u32 = 16;
pub const SCHED_FLAG_UTIL_CLAMP_MIN: u32 = 32;
pub const SCHED_FLAG_UTIL_CLAMP_MAX: u32 = 64;
pub const SCHED_FLAG_KEEP_ALL: u32 = 24;
pub const SCHED_FLAG_UTIL_CLAMP: u32 = 96;
pub const SCHED_FLAG_ALL: u32 = 127;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct clone_args {
	pub flags: u64,
	pub pidfd: u64,
	pub child_tid: u64,
	pub parent_tid: u64,
	pub exit_signal: u64,
	pub stack: u64,
	pub stack_size: u64,
	pub tls: u64,
	pub set_tid: u64,
	pub set_tid_size: u64,
	pub cgroup: u64,
}
