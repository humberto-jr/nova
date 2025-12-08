#![no_implicit_prelude]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub extern crate libc;

pub mod khronos;
pub mod stb;
pub mod unix;

pub use ::core::ffi::CStr;
