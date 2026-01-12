use crate::ffi::khronos::spirv_reflect as ffi;

impl super::Descriptor {
	pub(crate) const fn from(descriptor: &ffi::SpvReflectDescriptorBinding) -> Self {
		match descriptor.descriptor_type {
			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLER => Self::Sampler {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER => Self::CombinedImageSampler {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLED_IMAGE => Self::SampledImage {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_IMAGE => Self::StorageImage {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_BUFFER => Self::UniformBuffer {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
				size: descriptor.block.size,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_BUFFER => Self::StorageBuffer {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
				size: descriptor.block.size,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_INPUT_ATTACHMENT => Self::InputAttachment {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
				input_index: descriptor.input_attachment_index,
			},

			ffi::SPV_REFLECT_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR => Self::AccelerationStructure {
				set_index: descriptor.set,
				binding_index: descriptor.binding,
				descriptor_count: descriptor.count,
			},

			_ => Self::Undefined,
		}
	}
}
