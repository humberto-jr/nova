pub const S_IFMT: u32 = 61440;
pub const S_IFSOCK: u32 = 49152;
pub const S_IFLNK: u32 = 40960;
pub const S_IFREG: u32 = 32768;
pub const S_IFBLK: u32 = 24576;
pub const S_IFDIR: u32 = 16384;
pub const S_IFCHR: u32 = 8192;
pub const S_IFIFO: u32 = 4096;
pub const S_ISUID: u32 = 2048;
pub const S_ISGID: u32 = 1024;
pub const S_ISVTX: u32 = 512;
pub const S_IRWXU: u32 = 448;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRWXG: u32 = 56;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IRWXO: u32 = 7;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;

pub const STATX_TYPE: u32 = 1;
pub const STATX_MODE: u32 = 2;
pub const STATX_NLINK: u32 = 4;
pub const STATX_UID: u32 = 8;
pub const STATX_GID: u32 = 16;
pub const STATX_ATIME: u32 = 32;
pub const STATX_MTIME: u32 = 64;
pub const STATX_CTIME: u32 = 128;
pub const STATX_INO: u32 = 256;
pub const STATX_SIZE: u32 = 512;
pub const STATX_BLOCKS: u32 = 1024;
pub const STATX_BASIC_STATS: u32 = 2047;
pub const STATX_BTIME: u32 = 2048;
pub const STATX_MNT_ID: u32 = 4096;
pub const STATX_DIOALIGN: u32 = 8192;
pub const STATX_MNT_ID_UNIQUE: u32 = 16384;
pub const STATX_SUBVOL: u32 = 32768;
pub const STATX_WRITE_ATOMIC: u32 = 65536;
pub const STATX_DIO_READ_ALIGN: u32 = 131072;
pub const STATX__RESERVED: u32 = 2147483648;
pub const STATX_ALL: u32 = 4095;
pub const STATX_ATTR_COMPRESSED: u32 = 4;
pub const STATX_ATTR_IMMUTABLE: u32 = 16;
pub const STATX_ATTR_APPEND: u32 = 32;
pub const STATX_ATTR_NODUMP: u32 = 64;
pub const STATX_ATTR_ENCRYPTED: u32 = 2048;
pub const STATX_ATTR_AUTOMOUNT: u32 = 4096;
pub const STATX_ATTR_MOUNT_ROOT: u32 = 8192;
pub const STATX_ATTR_VERITY: u32 = 1048576;
pub const STATX_ATTR_DAX: u32 = 2097152;
pub const STATX_ATTR_WRITE_ATOMIC: u32 = 4194304;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct statx_timestamp {
	pub tv_sec: i64,
	pub tv_nsec: u32,
	pub __reserved: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct statx {
	pub stx_mask: u32,
	pub stx_blksize: u32,
	pub stx_attributes: u64,
	pub stx_nlink: u32,
	pub stx_uid: u32,
	pub stx_gid: u32,
	pub stx_mode: u16,
	pub __spare0: [u16; 1],
	pub stx_ino: u64,
	pub stx_size: u64,
	pub stx_blocks: u64,
	pub stx_attributes_mask: u64,
	pub stx_atime: statx_timestamp,
	pub stx_btime: statx_timestamp,
	pub stx_ctime: statx_timestamp,
	pub stx_mtime: statx_timestamp,
	pub stx_rdev_major: u32,
	pub stx_rdev_minor: u32,
	pub stx_dev_major: u32,
	pub stx_dev_minor: u32,
	pub stx_mnt_id: u64,
	pub stx_dio_mem_align: u32,
	pub stx_dio_offset_align: u32,
	pub stx_subvol: u64,
	pub stx_atomic_write_unit_min: u32,
	pub stx_atomic_write_unit_max: u32,
	pub stx_atomic_write_segments_max: u32,
	pub stx_dio_read_offset_align: u32,
	pub stx_atomic_write_unit_max_opt: u32,
	pub __spare2: [u32; 1],
	pub __spare3: [u64; 8],
}
