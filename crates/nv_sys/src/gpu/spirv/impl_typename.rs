use crate::ffi::khronos::spirv_reflect as ffi;

impl super::Typename {
	pub(crate) fn from(variable: &ffi::SpvReflectInterfaceVariable) -> Self {
		let flags = unwrap_type_flags(variable);

		let primitive = super::Primitive::from(variable);

		if (flags & ffi::SPV_REFLECT_TYPE_FLAG_VECTOR) > 0 {
			Self::Vector {
				base_type: primitive,
				count: variable.numeric.vector.component_count,
			}
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_MATRIX) > 0 {
			if (variable.decoration_flags & ffi::SPV_REFLECT_DECORATION_ROW_MAJOR) > 0 {
				Self::MatrixRowMajor {
					base_type: primitive,
					row_count: variable.numeric.matrix.row_count,
					col_count: variable.numeric.matrix.column_count,
				}
			} else {
				Self::MatrixColMajor {
					base_type: primitive,
					row_count: variable.numeric.matrix.row_count,
					col_count: variable.numeric.matrix.column_count,
				}
			}
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_ARRAY) > 0 {
			// NOTE: The type of variable.array is SpvReflectArrayTraits. The field
			// variable.array.dims[n] is the element count in the n-th dimension and
			// the variable.array.stride the byte count for each element in the outer
			// array. As of now, we only reflect on the outer array. The total size
			// in bytes is given by variable.array.dims[0]*variable.array.stride.
			if variable.array.dims_count == ffi::SPV_REFLECT_ARRAY_DIM_RUNTIME {
				Self::RuntimeArray {
					type_size: variable.array.stride,
				}
			} else {
				Self::Array {
					outer_type_size: variable.array.stride,
					dim_count: variable.array.dims_count,
					dim_length: variable.array.dims,
				}
			}
		}
		//
		else if (flags & (ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_IMAGE | ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLED_IMAGE)) > 0 {
			let traits = unwrap_image_traits(variable);

			let (channel_size, channel_count) = image_layout(&traits);

			if traits.sampled == 0 {
				if traits.dim == ffi::SpvDim1D {
					Self::UncompressedImage1D {
						channel_size,
						channel_count,
					}
				}
				//
				else if traits.dim == ffi::SpvDim2D {
					Self::UncompressedImage2D {
						channel_size,
						channel_count,
					}
				}
				//
				else if traits.dim == ffi::SpvDim3D {
					Self::UncompressedImage3D {
						channel_size,
						channel_count,
					}
				}
				//
				else if traits.dim == ffi::SpvDimCube {
					Self::UncompressedImageCube {
						channel_size,
						channel_count,
					}
				}
				//
				else {
					crate::panic!("Unable to parse an unknown SPIRV external image type (1)");
				}
			} else {
				if traits.ms == 0 {
					if traits.dim == ffi::SpvDim1D {
						Self::SampledImage1D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDim2D {
						Self::SampledImage2D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDim3D {
						Self::SampledImage3D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDimCube {
						Self::SampledImageCube {
							channel_size,
							channel_count,
						}
					}
					//
					else {
						crate::panic!("Unable to parse an unknown SPIRV external image type (2)");
					}
				} else {
					if traits.dim == ffi::SpvDim1D {
						Self::MultisampledImage1D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDim2D {
						Self::MultisampledImage2D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDim3D {
						Self::MultisampledImage3D {
							channel_size,
							channel_count,
						}
					}
					//
					else if traits.dim == ffi::SpvDimCube {
						Self::MultisampledImageCube {
							channel_size,
							channel_count,
						}
					}
					//
					else {
						crate::panic!("Unable to parse an unknown SPIRV external image type (3)");
					}
				}
			}
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLER) > 0 {
			Self::Sampler
		}
		//
		else if (flags & (ffi::SPV_REFLECT_TYPE_FLAG_STRUCT | ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_BLOCK)) > 0 {
			if (variable.decoration_flags & ffi::SPV_REFLECT_DECORATION_BLOCK) > 0 {
				Self::UniformBlock
			}
			//
			else if (variable.decoration_flags & ffi::SPV_REFLECT_DECORATION_BUFFER_BLOCK) > 0 {
				Self::StorageBlock
			}
			//
			else {
				Self::Struct {
					member_count: variable.member_count,
				}
			}
		}
		//
		else if (flags & ffi::SPV_REFLECT_TYPE_FLAG_EXTERNAL_ACCELERATION_STRUCTURE) > 0 {
			Self::AccelerationStructure
		}
		//
		else if primitive != super::Primitive::Unknown {
			Self::Base {
				base_type: primitive,
			}
		}
		//
		else {
			Self::Undefined
		}
	}
}

//
// Internals:
//

const BYTE_WIDTH: u32 = 8;

pub const fn image_layout(traits: &ffi::SpvReflectImageTraits) -> (u32, u8) {
	match traits.image_format {
		ffi::SpvImageFormatR8 | ffi::SpvImageFormatR8i | ffi::SpvImageFormatR8Snorm | ffi::SpvImageFormatR8ui => (8 / BYTE_WIDTH, 1),

		ffi::SpvImageFormatR16f | ffi::SpvImageFormatR16 | ffi::SpvImageFormatR16i | ffi::SpvImageFormatR16Snorm | ffi::SpvImageFormatR16ui => (16 / BYTE_WIDTH, 1),

		ffi::SpvImageFormatR32f | ffi::SpvImageFormatR32i | ffi::SpvImageFormatR32ui => (32 / BYTE_WIDTH, 1),

		ffi::SpvImageFormatR64i | ffi::SpvImageFormatR64ui => (64 / BYTE_WIDTH, 1),

		ffi::SpvImageFormatRg8 | ffi::SpvImageFormatRg8i | ffi::SpvImageFormatRg8Snorm | ffi::SpvImageFormatRg8ui => (8 / BYTE_WIDTH, 2),

		ffi::SpvImageFormatRg16f | ffi::SpvImageFormatRg16 | ffi::SpvImageFormatRg16i | ffi::SpvImageFormatRg16Snorm | ffi::SpvImageFormatRg16ui => (16 / BYTE_WIDTH, 2),

		ffi::SpvImageFormatRg32f | ffi::SpvImageFormatRg32i | ffi::SpvImageFormatRg32ui => (32 / BYTE_WIDTH, 2),

		ffi::SpvImageFormatRgba8 | ffi::SpvImageFormatRgba8i | ffi::SpvImageFormatRgba8Snorm | ffi::SpvImageFormatRgba8ui => (8 / BYTE_WIDTH, 4),

		ffi::SpvImageFormatRgba16f | ffi::SpvImageFormatRgba16 | ffi::SpvImageFormatRgba16i | ffi::SpvImageFormatRgba16Snorm | ffi::SpvImageFormatRgba16ui => (16 / BYTE_WIDTH, 4),

		ffi::SpvImageFormatRgba32f | ffi::SpvImageFormatRgba32i | ffi::SpvImageFormatRgba32ui => (32 / BYTE_WIDTH, 4),

		_ => (0, 0),
	}
}

#[inline]
const fn unwrap_type_flags(variable: &ffi::SpvReflectInterfaceVariable) -> ffi::SpvReflectTypeFlags {
	unsafe { (*variable.type_description).type_flags }
}

#[inline]
const fn unwrap_image_traits(variable: &ffi::SpvReflectInterfaceVariable) -> &ffi::SpvReflectImageTraits {
	unsafe { &(*variable.type_description).traits.image }
}
