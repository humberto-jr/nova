use ::core::slice;

use crate::ffi::CStr;

impl super::PushConstant {
	pub fn name(&self) -> &str {
		if self.0.name.is_null() {
			"[unnamed]"
		} else {
			unsafe { CStr::from_ptr(self.0.name).to_str().unwrap_or("[unnamed]") }
		}
	}

	#[inline]
	pub const fn layout(&self) -> (u32, u32, u32, u32) {
		(self.0.absolute_offset, self.0.offset, self.0.size, self.0.padded_size)
	}

	pub const fn members(&self) -> &[Self] {
		if self.0.member_count > 0 {
			unsafe {
				// SAFETY: PushConstant has the same ABI and layout as SpvReflectBlockVariable
				// due to the transparent representation.
				slice::from_raw_parts(self.0.members as *const Self, self.0.member_count as usize)
			}
		} else {
			&[]
		}
	}
}
