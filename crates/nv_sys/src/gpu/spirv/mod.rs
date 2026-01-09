use ::core::marker;

use crate::ffi::khronos::spirv_reflect as ffi;

pub const MAX_SHADER_LOCATION_COUNT: usize = 64;

pub const MAX_ARRAY_DIMENSION_COUNT: usize = 32;

pub type Word = u32;

//
// ExecModel:
//

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ExecModel {
	None,
	Vertex,
	Fragment,
	Compute,
	Geometry,
}

//
// Primitive:
//

mod impl_primitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Primitive {
	Unknown,
	Void,
	Bool,
	Half,
	Float,
	Double,
	Int8,
	Int16,
	Int32,
	Int64,
	UnsignedInt8,
	UnsignedInt16,
	UnsignedInt32,
	UnsignedInt64,
	Opaque,
}

//
// Typename:
//

mod impl_typename;

#[derive(Debug, Clone, PartialEq)]
pub enum Typename {
	Undefined,

	Base {
		base_type: Primitive,
	},

	// SpvOpTypeVector
	Vector {
		base_type: Primitive,
		count: u32,
	},

	// SpvOpTypeMatrix + SpvDecorationRowMajor
	MatrixRowMajor {
		base_type: Primitive,
		row_count: u32,
		col_count: u32,
	},

	// SpvOpTypeMatrix + SpvDecorationColumnMajor
	MatrixColMajor {
		base_type: Primitive,
		row_count: u32,
		col_count: u32,
	},

	// SpvOpTypeImage
	UncompressedImage1D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeImage
	UncompressedImage2D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeImage
	UncompressedImage3D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeImage
	UncompressedImageCube {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::sampled == 1
	SampledImage1D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::sampled == 1
	SampledImage2D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::sampled == 1
	SampledImage3D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::sampled == 1
	SampledImageCube {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::ms == 1
	MultisampledImage1D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::ms == 1
	MultisampledImage2D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::ms == 1
	MultisampledImage3D {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampledImage + SpvReflectImageTraits::ms == 1
	MultisampledImageCube {
		channel_size: u32,
		channel_count: u8,
	},

	// SpvOpTypeSampler
	Sampler,

	// SpvOpTypeArray
	Array {
		outer_type_size: u32,
		dim_count: u32,
		dim_length: [u32; MAX_ARRAY_DIMENSION_COUNT],
	},

	// SpvOpTypeRuntimeArray + SpvReflectArrayTraits::dims_count == SPV_REFLECT_ARRAY_DIM_RUNTIME
	RuntimeArray {
		type_size: u32,
	},

	// SpvOpTypeStruct
	Struct {
		member_count: u32,
	},

	// SpvOpTypeStruct + SpvDecorationBlock
	UniformBlock,

	// SpvOpTypeStruct + SpvDecorationBufferBlock
	StorageBlock,

	// SpvOpTypeAccelerationStructureKHR (see the Vulkan Ray Tracing extension)
	AccelerationStructure,
}

//
// Descriptor:
//

mod impl_descriptor;

#[derive(Debug, Clone, PartialEq)]
pub enum Descriptor {
	Undefined,

	Sampler {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
	},

	CombinedImageSampler {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
	},

	SampledImage {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
	},

	StorageImage {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
	},

	UniformBuffer {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
		size: u32,
	},

	StorageBuffer {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
		size: u32,
	},

	InputAttachment {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
		input_index: u32,
	},

	AccelerationStructure {
		set_index: u32,
		binding_index: u32,
		descriptor_count: u32,
	},
}

//
// PushConstant:
//

mod impl_push_constant;

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct PushConstant(ffi::SpvReflectBlockVariable);

//
// InterfaceVariableList and InterfaceVariable:
//

mod impl_interface_variable;

#[derive(Debug, PartialEq)]
pub struct InterfaceVariableList {
	count: u32,
	list: [*const ffi::SpvReflectInterfaceVariable; MAX_SHADER_LOCATION_COUNT],
}

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct InterfaceVariable(*const ffi::SpvReflectInterfaceVariable);

//
// DescriptorSetList and DescriptorSet:
//

mod impl_descriptor_set;

#[derive(Debug, PartialEq)]
pub struct DescriptorSetList {
	count: u32,
	list: *const ffi::SpvReflectDescriptorSet,
}

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct DescriptorSet(ffi::SpvReflectDescriptorSet);

//
// DescriptorBindingList and DescriptorBinding:
//

mod impl_descriptor_binding;

#[derive(Debug, PartialEq)]
pub struct DescriptorBindingList {
	count: u32,
	list: *const ffi::SpvReflectDescriptorBinding,
}

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct DescriptorBinding(ffi::SpvReflectDescriptorBinding);

//
// EntryPoint:
//

mod impl_entry_point;

#[repr(transparent)]
#[derive(Debug, PartialEq)]
pub struct EntryPoint(ffi::SpvReflectEntryPoint);

//
// Module:
//

mod impl_module;

#[derive(Debug, PartialEq)]
pub struct Module<'c>(ffi::SpvReflectShaderModule, marker::PhantomData<&'c ()>);
