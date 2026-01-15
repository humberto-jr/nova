use ::core::slice;

impl super::DescriptorSetList {
	#[inline]
	pub const fn count(&self) -> u32 {
		self.count
	}

	pub const fn as_slice(&self) -> &[super::DescriptorSet] {
		if self.count > 0 {
			unsafe {
				// SAFETY: DescriptorSet has the same ABI and layout as SpvReflectDescriptorSet
				// due to the transparent representation.
				slice::from_raw_parts(self.list as *const super::DescriptorSet, self.count as usize)
			}
		} else {
			&[]
		}
	}
}

impl super::DescriptorSet {
	#[inline]
	pub const fn set(&self) -> u32 {
		self.0.set
	}

	#[inline]
	pub const fn bindings(&self) -> super::DescriptorBindingList {
		unsafe {
			super::DescriptorBindingList {
				count: self.0.binding_count,
				list: *self.0.bindings,
			}
		}
	}
}
