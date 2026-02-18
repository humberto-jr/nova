pub const PROT_READ: u32 = 1;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC: u32 = 4;
pub const PROT_SEM: u32 = 8;
pub const PROT_NONE: u32 = 0;
pub const PROT_GROWSDOWN: u32 = 16777216;
pub const PROT_GROWSUP: u32 = 33554432;

pub const MAP_FILE: u32 = 0;
pub const MAP_SHARED: u32 = 1;
pub const MAP_PRIVATE: u32 = 2;
pub const MAP_SHARED_VALIDATE: u32 = 3;
pub const MAP_DROPPABLE: u32 = 8;
pub const MAP_GROWSDOWN: u32 = 256;
pub const MAP_DENYWRITE: u32 = 2048;
pub const MAP_EXECUTABLE: u32 = 4096;
pub const MAP_LOCKED: u32 = 8192;
pub const MAP_NORESERVE: u32 = 16384;
pub const MAP_TYPE: u32 = 15;
pub const MAP_FIXED: u32 = 16;
pub const MAP_ANONYMOUS: u32 = 32;
pub const MAP_POPULATE: u32 = 32768;
pub const MAP_NONBLOCK: u32 = 65536;
pub const MAP_STACK: u32 = 131072;
pub const MAP_HUGETLB: u32 = 262144;
pub const MAP_SYNC: u32 = 524288;
pub const MAP_FIXED_NOREPLACE: u32 = 1048576;
pub const MAP_UNINITIALIZED: u32 = 67108864;

pub const MLOCK_ONFAULT: u32 = 1;

pub const MS_ASYNC: u32 = 1;
pub const MS_INVALIDATE: u32 = 2;
pub const MS_SYNC: u32 = 4;

pub const MADV_NORMAL: u32 = 0;
pub const MADV_RANDOM: u32 = 1;
pub const MADV_SEQUENTIAL: u32 = 2;
pub const MADV_WILLNEED: u32 = 3;
pub const MADV_DONTNEED: u32 = 4;
pub const MADV_FREE: u32 = 8;
pub const MADV_REMOVE: u32 = 9;
pub const MADV_DONTFORK: u32 = 10;
pub const MADV_DOFORK: u32 = 11;
pub const MADV_HWPOISON: u32 = 100;
pub const MADV_SOFT_OFFLINE: u32 = 101;
pub const MADV_MERGEABLE: u32 = 12;
pub const MADV_UNMERGEABLE: u32 = 13;
pub const MADV_HUGEPAGE: u32 = 14;
pub const MADV_NOHUGEPAGE: u32 = 15;
pub const MADV_DONTDUMP: u32 = 16;
pub const MADV_DODUMP: u32 = 17;
pub const MADV_WIPEONFORK: u32 = 18;
pub const MADV_KEEPONFORK: u32 = 19;
pub const MADV_COLD: u32 = 20;
pub const MADV_PAGEOUT: u32 = 21;
pub const MADV_POPULATE_READ: u32 = 22;
pub const MADV_POPULATE_WRITE: u32 = 23;
pub const MADV_DONTNEED_LOCKED: u32 = 24;
pub const MADV_COLLAPSE: u32 = 25;
pub const MADV_GUARD_INSTALL: u32 = 102;
pub const MADV_GUARD_REMOVE: u32 = 103;

pub const PKEY_UNRESTRICTED: u32 = 0;
pub const PKEY_DISABLE_ACCESS: u32 = 1;
pub const PKEY_DISABLE_WRITE: u32 = 2;
pub const PKEY_ACCESS_MASK: u32 = 3;

pub const MCL_CURRENT: u32 = 1;
pub const MCL_FUTURE: u32 = 2;
pub const MCL_ONFAULT: u32 = 4;

pub const SHADOW_STACK_SET_TOKEN: u32 = 1;
pub const SHADOW_STACK_SET_MARKER: u32 = 2;
