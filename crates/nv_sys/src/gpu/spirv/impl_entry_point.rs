use crate::ffi::{
	CStr, //
	khronos::spirv_reflect as ffi,
};

impl super::EntryPoint {
	pub fn name(&self) -> &str {
		if self.0.name.is_null() {
			"[unnamed]"
		} else {
			unsafe { CStr::from_ptr(self.0.name).to_str().unwrap_or("[unnamed]") }
		}
	}

	pub const fn execution_model(&self) -> super::ExecModel {
		if (self.0.shader_stage & ffi::SPV_REFLECT_SHADER_STAGE_VERTEX_BIT) > 0 {
			super::ExecModel::Vertex
		}
		//
		else if (self.0.shader_stage & ffi::SPV_REFLECT_SHADER_STAGE_FRAGMENT_BIT) > 0 {
			super::ExecModel::Fragment
		}
		//
		else if (self.0.shader_stage & ffi::SPV_REFLECT_SHADER_STAGE_COMPUTE_BIT) > 0 {
			super::ExecModel::Compute
		}
		//
		else if (self.0.shader_stage & ffi::SPV_REFLECT_SHADER_STAGE_GEOMETRY_BIT) > 0 {
			super::ExecModel::Geometry
		}
		//
		else {
			super::ExecModel::None
		}
	}

	#[inline]
	pub fn input_variables(&self) -> super::InterfaceVariableList {
		super::InterfaceVariableList::from_sparse_layout(self.0.input_variable_count, self.0.input_variables)
	}

	#[inline]
	pub fn output_variables(&self) -> super::InterfaceVariableList {
		super::InterfaceVariableList::from_sparse_layout(self.0.output_variable_count, self.0.output_variables)
	}

	#[inline]
	pub const fn descriptor_sets(&self) -> super::DescriptorSetList {
		super::DescriptorSetList {
			count: self.0.descriptor_set_count,
			list: self.0.descriptor_sets,
		}
	}
}
