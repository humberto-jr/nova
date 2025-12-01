use crate::libc;

pub const STBI_default: i32 = 0;
pub const STBI_grey: i32 = 1;
pub const STBI_grey_alpha: i32 = 2;
pub const STBI_rgb: i32 = 3;
pub const STBI_rgb_alpha: i32 = 4;

pub type stbi_uc = u8;
pub type stbi_us = u16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stbi_io_callbacks {
	pub read: unsafe extern "C" fn(user: *mut (), data: *mut i8, size: i32) -> i32,
	pub skip: unsafe extern "C" fn(user: *mut (), n: i32),
	pub eof: unsafe extern "C" fn(user: *mut ()) -> i32,
}

#[link(name = "stb")]
unsafe extern "C" {
	pub fn stbi_load_from_memory(buffer: *const stbi_uc, len: i32, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_uc;

	pub fn stbi_load_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut (), x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_uc;

	pub fn stbi_load(filename: *const i8, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_uc;

	pub fn stbi_load_from_file(f: *mut libc::FILE, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_uc;

	pub fn stbi_load_gif_from_memory(buffer: *const stbi_uc, len: i32, delays: *mut *mut i32, x: *mut i32, y: *mut i32, z: *mut i32, comp: *mut i32, req_comp: i32) -> *mut stbi_uc;

	pub fn stbi_load_16_from_memory(buffer: *const stbi_uc, len: i32, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_us;

	pub fn stbi_load_16_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut (), x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_us;

	pub fn stbi_load_16(filename: *const i8, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_us;

	pub fn stbi_load_from_file_16(f: *mut libc::FILE, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut stbi_us;

	pub fn stbi_loadf_from_memory(buffer: *const stbi_uc, len: i32, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut f32;

	pub fn stbi_loadf_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut (), x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut f32;

	pub fn stbi_loadf(filename: *const i8, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut f32;

	pub fn stbi_loadf_from_file(f: *mut libc::FILE, x: *mut i32, y: *mut i32, channels_in_file: *mut i32, desired_channels: i32) -> *mut f32;

	pub fn stbi_hdr_to_ldr_gamma(gamma: f32);

	pub fn stbi_hdr_to_ldr_scale(scale: f32);

	pub fn stbi_ldr_to_hdr_gamma(gamma: f32);

	pub fn stbi_ldr_to_hdr_scale(scale: f32);

	pub fn stbi_is_hdr_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut ()) -> i32;

	pub fn stbi_is_hdr_from_memory(buffer: *const stbi_uc, len: i32) -> i32;

	pub fn stbi_is_hdr(filename: *const i8) -> i32;

	pub fn stbi_is_hdr_from_file(f: *mut libc::FILE) -> i32;

	pub fn stbi_failure_reason() -> *const i8;

	pub fn stbi_image_free(retval_from_stbi_load: *mut ());

	pub fn stbi_info_from_memory(buffer: *const stbi_uc, len: i32, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32;

	pub fn stbi_info_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut (), x: *mut i32, y: *mut i32, comp: *mut i32) -> i32;

	pub fn stbi_is_16_bit_from_memory(buffer: *const stbi_uc, len: i32) -> i32;

	pub fn stbi_is_16_bit_from_callbacks(clbk: *const stbi_io_callbacks, user: *mut ()) -> i32;

	pub fn stbi_info(filename: *const i8, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32;

	pub fn stbi_info_from_file(f: *mut libc::FILE, x: *mut i32, y: *mut i32, comp: *mut i32) -> i32;

	pub fn stbi_is_16_bit(filename: *const i8) -> i32;

	pub fn stbi_is_16_bit_from_file(f: *mut libc::FILE) -> i32;

	pub fn stbi_set_unpremultiply_on_load(flag_true_if_should_unpremultiply: i32);

	pub fn stbi_convert_iphone_png_to_rgb(flag_true_if_should_convert: i32);

	pub fn stbi_set_flip_vertically_on_load(flag_true_if_should_flip: i32);

	//	pub fn stbi_set_unpremultiply_on_load_thread(flag_true_if_should_unpremultiply: i32);

	pub fn stbi_convert_iphone_png_to_rgb_thread(flag_true_if_should_convert: i32);

	pub fn stbi_set_flip_vertically_on_load_thread(flag_true_if_should_flip: i32);

	pub fn stbi_zlib_decode_malloc_guesssize(buffer: *const i8, len: i32, initial_size: i32, outlen: *mut i32) -> *mut i8;

	pub fn stbi_zlib_decode_malloc_guesssize_headerflag(buffer: *const i8, len: i32, initial_size: i32, outlen: *mut i32, parse_header: i32) -> *mut i8;

	pub fn stbi_zlib_decode_malloc(buffer: *const i8, len: i32, outlen: *mut i32) -> *mut i8;

	pub fn stbi_zlib_decode_buffer(obuffer: *mut i8, olen: i32, ibuffer: *const i8, ilen: i32) -> i32;

	pub fn stbi_zlib_decode_noheader_malloc(buffer: *const i8, len: i32, outlen: *mut i32) -> *mut i8;

	pub fn stbi_zlib_decode_noheader_buffer(obuffer: *mut i8, olen: i32, ibuffer: *const i8, ilen: i32) -> i32;
}
