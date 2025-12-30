#![no_implicit_prelude]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub extern crate libc;

pub mod khronos;
pub mod stb;
pub mod unix;

pub use ::core::ffi::CStr;

#[inline(always)]
pub const fn c_str(rs_str: &str) -> *const i8 {
	rs_str.as_ptr() as *const i8
}

#[inline(always)]
pub const fn c_null<T>() -> *mut T {
	0 as *mut T
}

pub const NULL_VOID: *mut () = c_null();
