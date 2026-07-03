use ::core::ops;

use crate::spec;

use super::ffi;

const INVALID_HANDLE: *mut ffi::c_void = 0 as _;

impl ops::Drop for super::DynamicLibrary {
	#[inline]
	fn drop(&mut self) {
		if self.0 != INVALID_HANDLE {
			let _ = unsafe { ffi::dlclose(self.0) };
		}
	}
}

impl spec::DynamicLibrary for super::DynamicLibrary {
	type Address = *mut ();

	fn load(&mut self, filename: &str) -> spec::Result<()> {
		let handle = unsafe { ffi::dlopen(crate::ffi::c_str!(filename), ffi::RTLD_LAZY | ffi::RTLD_LOCAL) };

		if handle.is_null() {
			spec::Result::Err(spec::Error::Unknown)
		} else {
			self.0 = handle;
			spec::Result::Ok(())
		}
	}

	#[inline]
	fn is_loaded(&self) -> bool {
		self.0 != INVALID_HANDLE
	}

	fn find_symbol(&self, name: &str) -> spec::Result<Self::Address> {
		let symbol_addr = unsafe { ffi::dlsym(self.0, crate::ffi::c_str!(name)) };

		if symbol_addr.is_null() {
			spec::Result::Err(spec::Error::SymbolAddressNotFound)
		} else {
			spec::Result::Ok(symbol_addr as _)
		}
	}
}

impl super::DynamicLibrary {
	#[inline]
	pub const fn new() -> Self {
		Self(INVALID_HANDLE)
	}
}
