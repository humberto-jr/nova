use ::core::slice;

use crate::ffi::CStr;

impl super::DescriptorBindingList {
	#[inline]
	pub const fn count(&self) -> u32 {
		self.count
	}

	pub const fn as_slice(&self) -> &[super::DescriptorBinding] {
		if self.count > 0 {
			unsafe {
				// SAFETY: DescriptorBinding has the same ABI and layout as SpvReflectDescriptorBinding
				// due to the transparent representation.
				slice::from_raw_parts(self.list as *const super::DescriptorBinding, self.count as usize)
			}
		} else {
			&[]
		}
	}
}

impl super::DescriptorBinding {
	pub fn name(&self) -> &str {
		if self.0.name.is_null() {
			"[unnamed]"
		} else {
			unsafe { CStr::from_ptr(self.0.name).to_str().unwrap_or("[unnamed]") }
		}
	}

	#[inline]
	pub const fn base_type(&self) -> super::Descriptor {
		super::Descriptor::from(&self.0)
	}
}
