use ::core::{
	marker, //
	ops,
	slice,
};

use crate::{
	ffi::CStr, //
	ffi::khronos::spirv_reflect as ffi,
};

use crate::mem;

impl ops::Drop for super::Module<'_> {
	#[inline]
	fn drop(&mut self) {
		unsafe {
			ffi::spvReflectDestroyShaderModule(&mut self.0);
		}
	}
}

impl<'c> super::Module<'c> {
	pub fn new(bytecode: &'c [super::Word]) -> Self {
		unsafe {
			let raw = bytecode.as_ptr() as *const ();

			let len = bytecode.len() * mem::size_of::<super::Word>();

			let mut module: ffi::SpvReflectShaderModule = mem::zeroed();

			let info = ffi::spvReflectCreateShaderModule2(ffi::SPV_REFLECT_MODULE_FLAG_NO_COPY, len, raw, &mut module);

			if info != ffi::SPV_REFLECT_RESULT_SUCCESS {
				crate::panic!("spvReflectCreateShaderModule2() failed with error code {info}");
			}

			Self(module, marker::PhantomData::<&()>)
		}
	}

	pub fn filename(&self) -> &str {
		let name = self.0.source_file;

		if name.is_null() {
			"[unknown]"
		} else {
			unsafe { CStr::from_ptr(name).to_str().unwrap_or("[unknown]") }
		}
	}

	pub const fn entry_points(&self) -> &[super::EntryPoint] {
		if self.0.entry_point_count > 0 {
			unsafe {
				// SAFETY: EntryPoint has the same ABI and layout as SpvReflectEntryPoint
				// due to the transparent representation.
				slice::from_raw_parts(self.0.entry_points as *const super::EntryPoint, self.0.entry_point_count as usize)
			}
		} else {
			&[]
		}
	}

	pub const fn push_constants(&self) -> &[super::PushConstant] {
		if self.0.push_constant_block_count > 0 {
			unsafe {
				// SAFETY: PushConstant has the same ABI and layout as SpvReflectBlockVariable
				// due to the transparent representation.
				slice::from_raw_parts(self.0.push_constant_blocks as *const super::PushConstant, self.0.push_constant_block_count as usize)
			}
		} else {
			&[]
		}
	}
}
