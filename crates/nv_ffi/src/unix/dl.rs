pub const RTLD_LAZY: i32 = 1;
pub const RTLD_NOW: i32 = 2;
pub const RTLD_GLOBAL: i32 = 0x100;
pub const RTLD_LOCAL: i32 = 0x000;
pub const RTLD_TRACE: i32 = 0x200;
pub const RTLD_NODELETE: i32 = 0x400;
pub const RTLD_NOLOAD: i32 = 0x800;

#[link(name = "dl")]
unsafe extern "C" {
	pub fn dlopen(file: *const i8, mode: i32) -> *mut ();

	pub fn dlsym(handle: *mut (), name: *const i8) -> *mut ();

	pub fn dlclose(handle: *mut ()) -> i32;

	pub fn dlerror() -> *const i8;
}
