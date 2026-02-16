pub const O_ACCMODE: u32 = 3;
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 64;
pub const O_EXCL: u32 = 128;
pub const O_NOCTTY: u32 = 256;
pub const O_TRUNC: u32 = 512;
pub const O_APPEND: u32 = 1024;
pub const O_NONBLOCK: u32 = 2048;
pub const O_DSYNC: u32 = 4096;

pub const FASYNC: u32 = 8192;

pub const O_DIRECT: u32 = 16384;
pub const O_LARGEFILE: u32 = 32768;
pub const O_DIRECTORY: u32 = 65536;
pub const O_NOFOLLOW: u32 = 131072;
pub const O_NOATIME: u32 = 262144;
pub const O_CLOEXEC: u32 = 524288;
pub const O_SYNC: u32 = 1052672;
pub const O_PATH: u32 = 2097152;
pub const O_TMPFILE: u32 = 4259840;
pub const O_NDELAY: u32 = 2048;

pub const F_DUPFD: u32 = 0;
pub const F_GETFD: u32 = 1;
pub const F_SETFD: u32 = 2;
pub const F_GETFL: u32 = 3;
pub const F_SETFL: u32 = 4;
pub const F_GETLK: u32 = 5;
pub const F_SETLK: u32 = 6;
pub const F_SETLKW: u32 = 7;
pub const F_SETOWN: u32 = 8;
pub const F_GETOWN: u32 = 9;
pub const F_SETSIG: u32 = 10;
pub const F_GETSIG: u32 = 11;
pub const F_SETOWN_EX: u32 = 15;
pub const F_GETOWN_EX: u32 = 16;
pub const F_GETOWNER_UIDS: u32 = 17;

pub const F_OFD_GETLK: u32 = 36;
pub const F_OFD_SETLK: u32 = 37;
pub const F_OFD_SETLKW: u32 = 38;
pub const F_OWNER_TID: u32 = 0;
pub const F_OWNER_PID: u32 = 1;
pub const F_OWNER_PGRP: u32 = 2;

pub const FD_CLOEXEC: u32 = 1;

pub const F_RDLCK: u32 = 0;
pub const F_WRLCK: u32 = 1;
pub const F_UNLCK: u32 = 2;
pub const F_EXLCK: u32 = 4;
pub const F_SHLCK: u32 = 8;

pub const LOCK_SH: u32 = 1;
pub const LOCK_EX: u32 = 2;
pub const LOCK_NB: u32 = 4;
pub const LOCK_UN: u32 = 8;
pub const LOCK_MAND: u32 = 32;
pub const LOCK_READ: u32 = 64;
pub const LOCK_WRITE: u32 = 128;
pub const LOCK_RW: u32 = 192;

pub const F_LINUX_SPECIFIC_BASE: u32 = 1024;

pub const F_SETLEASE: u32 = 1024;
pub const F_GETLEASE: u32 = 1025;
pub const F_NOTIFY: u32 = 1026;
pub const F_DUPFD_QUERY: u32 = 1027;
pub const F_CREATED_QUERY: u32 = 1028;
pub const F_CANCELLK: u32 = 1029;
pub const F_DUPFD_CLOEXEC: u32 = 1030;
pub const F_SETPIPE_SZ: u32 = 1031;
pub const F_GETPIPE_SZ: u32 = 1032;
pub const F_ADD_SEALS: u32 = 1033;
pub const F_GET_SEALS: u32 = 1034;
pub const F_SEAL_SEAL: u32 = 1;
pub const F_SEAL_SHRINK: u32 = 2;
pub const F_SEAL_GROW: u32 = 4;
pub const F_SEAL_WRITE: u32 = 8;
pub const F_SEAL_FUTURE_WRITE: u32 = 16;
pub const F_SEAL_EXEC: u32 = 32;
pub const F_GET_RW_HINT: u32 = 1035;
pub const F_SET_RW_HINT: u32 = 1036;
pub const F_GET_FILE_RW_HINT: u32 = 1037;
pub const F_SET_FILE_RW_HINT: u32 = 1038;

pub const RWH_WRITE_LIFE_NOT_SET: u32 = 0;
pub const RWH_WRITE_LIFE_NONE: u32 = 1;
pub const RWH_WRITE_LIFE_SHORT: u32 = 2;
pub const RWH_WRITE_LIFE_MEDIUM: u32 = 3;
pub const RWH_WRITE_LIFE_LONG: u32 = 4;
pub const RWH_WRITE_LIFE_EXTREME: u32 = 5;
pub const RWF_WRITE_LIFE_NOT_SET: u32 = 0;

pub const F_GETDELEG: u32 = 1039;
pub const F_SETDELEG: u32 = 1040;

pub const DN_ACCESS: u32 = 1;
pub const DN_MODIFY: u32 = 2;
pub const DN_CREATE: u32 = 4;
pub const DN_DELETE: u32 = 8;
pub const DN_RENAME: u32 = 16;
pub const DN_ATTRIB: u32 = 32;
pub const DN_MULTISHOT: u32 = 2147483648;

pub const AT_FDCWD: i32 = -100;

pub const PIDFD_SELF_THREAD: i32 = -10000;
pub const PIDFD_SELF_THREAD_GROUP: i32 = -10001;

pub const FD_PIDFS_ROOT: i32 = -10002;
pub const FD_NSFS_ROOT: i32 = -10003;
pub const FD_INVALID: i32 = -10009;

pub const AT_SYMLINK_NOFOLLOW: u32 = 256;
pub const AT_SYMLINK_FOLLOW: u32 = 1024;
pub const AT_NO_AUTOMOUNT: u32 = 2048;
pub const AT_EMPTY_PATH: u32 = 4096;
pub const AT_STATX_SYNC_TYPE: u32 = 24576;
pub const AT_STATX_SYNC_AS_STAT: u32 = 0;
pub const AT_STATX_FORCE_SYNC: u32 = 8192;
pub const AT_STATX_DONT_SYNC: u32 = 16384;
pub const AT_RECURSIVE: u32 = 32768;
pub const AT_RENAME_NOREPLACE: u32 = 1;
pub const AT_RENAME_EXCHANGE: u32 = 2;
pub const AT_RENAME_WHITEOUT: u32 = 4;
pub const AT_EACCESS: u32 = 512;
pub const AT_REMOVEDIR: u32 = 512;
pub const AT_HANDLE_FID: u32 = 512;
pub const AT_HANDLE_MNT_ID_UNIQUE: u32 = 1;
pub const AT_HANDLE_CONNECTABLE: u32 = 2;
pub const AT_EXECVE_CHECK: u32 = 65536;
