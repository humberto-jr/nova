#![no_implicit_prelude]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use ::core::ffi;

pub extern crate libc;

pub mod khronos;
pub mod stb;

#[cfg(unix)]
pub mod unix;

//
// C helpers:
//

pub use ffi::CStr;
pub use ffi::c_char;
pub use ffi::c_double;
pub use ffi::c_float;
pub use ffi::c_int;
pub use ffi::c_long;
pub use ffi::c_longlong;
pub use ffi::c_schar;
pub use ffi::c_short;
pub use ffi::c_uchar;
pub use ffi::c_uint;
pub use ffi::c_ulong;
pub use ffi::c_ulonglong;
pub use ffi::c_ushort;
pub use ffi::c_void;

pub type c_size_t = usize;
pub type c_ptrdiff_t = isize;

#[inline(always)]
pub const fn c_null<T>() -> *mut T {
	0 as *mut T
}

#[inline(always)]
pub const fn is_nul_terminated(rs_str: &str) -> bool {
	(rs_str.len() > 0) && (*rs_str.as_bytes().last().unwrap() == b'\0')
}

#[inline(always)]
pub const fn c_str(rs_str: &str) -> *const c_char {
	// NOTE: Originally this Rust-to-C string conversion with a compile-time check of
	// nul-termination was implemented as a function. However, it ended up being more
	// helpful to have it as a macro to catch the source location of ill-formed strings.
	// The function version is kept for compatibility and can be used for literals where
	// nul-termination is guaranteed.
	crate::c_str!(rs_str)
}

#[macro_export]
macro_rules! c_str {
	($rs_str:expr) => {{
		::core::debug_assert!($crate::is_nul_terminated($rs_str));

		$rs_str.as_ptr() as *const $crate::c_char
	}};
}

pub const NULL_VOID: *mut () = c_null();
