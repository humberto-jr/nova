use ::core::slice;

use crate::{
	ffi::CStr, //
	ffi::khronos::spirv_reflect as ffi,
};

impl super::InterfaceVariableList {
	#[inline]
	pub const fn count(&self) -> u32 {
		self.count
	}

	pub const fn as_slice(&self) -> &[super::InterfaceVariable] {
		if self.count > 0 {
			unsafe {
				// SAFETY: InterfaceVariable has the same ABI and layout as SpvReflectInterfaceVariable
				// due to the transparent representation.
				slice::from_raw_parts(self.list.as_ptr() as *const super::InterfaceVariable, self.count as usize)
			}
		} else {
			&[]
		}
	}

	//
	// Internals:
	//

	pub(crate) fn from_sparse_layout(sparse_count: u32, sparse_list: *const *mut ffi::SpvReflectInterfaceVariable) -> Self {
		let slot_list = 0..(sparse_count as usize);

		crate::panic_if!(sparse_count >= super::MAX_SHADER_LOCATION_COUNT as u32);

		let mut packed_count = 0;
		let mut packed_list = [0 as *const ffi::SpvReflectInterfaceVariable; super::MAX_SHADER_LOCATION_COUNT];

		unsafe {
			for slot in slot_list {
				let raw = sparse_list.add(slot);

				if raw.is_null() {
					continue;
				}

				packed_list[packed_count] = *raw;
				packed_count += 1;
			}

			Self {
				count: packed_count as u32,
				list: packed_list,
			}
		}
	}
}

impl super::InterfaceVariable {
	pub fn name(&self) -> &str {
		let name = self.unwrap().name;

		if name.is_null() {
			"[unnamed]"
		} else {
			unsafe { CStr::from_ptr(name).to_str().unwrap_or("[unnamed]") }
		}
	}

	#[inline]
	pub const fn location(&self) -> u32 {
		self.unwrap().location
	}

	pub const fn members(&self) -> &[Self] {
		let variable = self.unwrap();

		if variable.member_count > 0 {
			unsafe {
				// SAFETY: InterfaceVariable has the same ABI and layout as SpvReflectInterfaceVariable
				// due to the transparent representation.
				slice::from_raw_parts(variable.members as *const Self, variable.member_count as usize)
			}
		} else {
			&[]
		}
	}

	#[inline]
	pub const fn base_type(&self) -> super::Primitive {
		super::Primitive::from(self.unwrap())
	}

	#[inline]
	pub fn typename(&self) -> super::Typename {
		super::Typename::from(self.unwrap())
	}

	//
	// Internals:
	//

	#[inline]
	const fn unwrap(&self) -> &ffi::SpvReflectInterfaceVariable {
		unsafe { &(*self.0) }
	}
}
