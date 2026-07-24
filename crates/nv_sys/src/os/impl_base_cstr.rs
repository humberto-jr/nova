use ::core::fmt;

use crate::{
	ffi, //
	ops,
};

impl<const MAX_LEN: usize> ops::Deref for super::BaseCStr<MAX_LEN> {
	type Target = str;

	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		self.as_str()
	}
}

impl<const MAX_LEN: usize> fmt::Display for super::BaseCStr<MAX_LEN> {
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		::core::write!(f, "{}", self.as_str())
	}
}

impl<const MAX_LEN: usize> super::BaseCStr<MAX_LEN> {
	pub const fn from_bytes_until_nul(string: &[u8]) -> Self {
		let mut len = 0;

		let mut buf = [0u8; MAX_LEN];

		while (len < string.len()) && (len < (MAX_LEN - 1)) && (string[len] != b'\0') {
			buf[len] = string[len];
			len += 1;
		}

		Self {
			len: len as _,
			buf,
		}
	}

	#[inline]
	pub const fn from_str(string: &str) -> Self {
		Self::from_bytes_until_nul(string.as_bytes())
	}

	#[inline]
	pub const unsafe fn from_cstr(string: *const ffi::c_char) -> Self {
		let slice = unsafe { ffi::CStr::from_ptr(string).to_bytes_with_nul() };

		Self::from_bytes_until_nul(slice)
	}

	#[inline]
	pub const fn len(&self) -> usize {
		self.len as _
	}

	#[inline]
	pub fn as_bytes(&self) -> &[u8] {
		&self.buf[..self.len()]
	}

	#[inline]
	pub fn as_bytes_with_nul(&self) -> &[u8] {
		&self.buf[..self.len() + 1]
	}

	#[inline]
	pub fn as_str(&self) -> &str {
		unsafe { str::from_utf8_unchecked(self.as_bytes()) }
	}

	#[inline]
	pub fn as_str_with_nul(&self) -> &str {
		unsafe { str::from_utf8_unchecked(self.as_bytes_with_nul()) }
	}

	#[inline]
	pub const fn as_cstr(&self) -> *const ffi::c_char {
		self.buf.as_ptr() as _
	}
}
