use crate::ffi::khronos::spirv_reflect as ffi;

impl super::Primitive {
	pub(crate) const fn from(variable: &ffi::SpvReflectInterfaceVariable) -> Self {
		let flags = unsafe { (*variable.type_description).type_flags };

		if (flags & ffi::SPV_REFLECT_TYPE_FLAG_VOID) > 0 {
			Self::Void
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_BOOL) > 0 {
			Self::Bool
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_FLOAT) > 0 {
			match variable.numeric.scalar.width {
				16 => Self::Half,
				32 => Self::Float,
				64 => Self::Double,
				_ => Self::Unknown,
			}
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_INT) > 0 {
			if variable.numeric.scalar.signedness == 1 {
				match variable.numeric.scalar.width {
					8 => Self::Int8,
					16 => Self::Int16,
					32 => Self::Int32,
					64 => Self::Int64,
					_ => Self::Unknown,
				}
			} else {
				match variable.numeric.scalar.width {
					8 => Self::UnsignedInt8,
					16 => Self::UnsignedInt16,
					32 => Self::UnsignedInt32,
					64 => Self::UnsignedInt64,
					_ => Self::Unknown,
				}
			}
		}
		//
		else if (flags & (ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_IMAGE | ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLER | ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLED_IMAGE)) > 0 {
			Self::Opaque
		}
		//
		else {
			Self::Unknown
		}
	}
}
