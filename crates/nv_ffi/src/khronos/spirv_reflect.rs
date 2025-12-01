use super::spirv::*;

// NOTE: The typenames of fields whose types are anonymous C structs are defined
// using the parent's typename followed by the suffix "AnonStruct."

pub type SpvReflectResult = u32;
pub const SPV_REFLECT_RESULT_SUCCESS: SpvReflectResult = 0;
pub const SPV_REFLECT_RESULT_NOT_READY: SpvReflectResult = 1;
pub const SPV_REFLECT_RESULT_ERROR_PARSE_FAILED: SpvReflectResult = 2;
pub const SPV_REFLECT_RESULT_ERROR_ALLOC_FAILED: SpvReflectResult = 3;
pub const SPV_REFLECT_RESULT_ERROR_RANGE_EXCEEDED: SpvReflectResult = 4;
pub const SPV_REFLECT_RESULT_ERROR_NULL_POINTER: SpvReflectResult = 5;
pub const SPV_REFLECT_RESULT_ERROR_INTERNAL_ERROR: SpvReflectResult = 6;
pub const SPV_REFLECT_RESULT_ERROR_COUNT_MISMATCH: SpvReflectResult = 7;
pub const SPV_REFLECT_RESULT_ERROR_ELEMENT_NOT_FOUND: SpvReflectResult = 8;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_CODE_SIZE: SpvReflectResult = 9;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_MAGIC_NUMBER: SpvReflectResult = 10;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_UNEXPECTED_EOF: SpvReflectResult = 11;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_ID_REFERENCE: SpvReflectResult = 12;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_SET_NUMBER_OVERFLOW: SpvReflectResult = 13;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_STORAGE_CLASS: SpvReflectResult = 14;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_RECURSION: SpvReflectResult = 15;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_INSTRUCTION: SpvReflectResult = 16;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_UNEXPECTED_BLOCK_DATA: SpvReflectResult = 17;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_BLOCK_MEMBER_REFERENCE: SpvReflectResult = 18;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_ENTRY_POINT: SpvReflectResult = 19;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_INVALID_EXECUTION_MODE: SpvReflectResult = 20;
pub const SPV_REFLECT_RESULT_ERROR_SPIRV_MAX_RECURSIVE_EXCEEDED: SpvReflectResult = 21;

pub type SpvReflectModuleFlagBits = u32;
pub const SPV_REFLECT_MODULE_FLAG_NONE: SpvReflectModuleFlagBits = 0;
pub const SPV_REFLECT_MODULE_FLAG_NO_COPY: SpvReflectModuleFlagBits = 1;
pub type SpvReflectModuleFlags = u32;

pub type SpvReflectTypeFlagBits = u32;
pub const SPV_REFLECT_TYPE_FLAG_UNDEFINED: SpvReflectTypeFlagBits = 0;
pub const SPV_REFLECT_TYPE_FLAG_VOID: SpvReflectTypeFlagBits = 1;
pub const SPV_REFLECT_TYPE_FLAG_BOOL: SpvReflectTypeFlagBits = 2;
pub const SPV_REFLECT_TYPE_FLAG_INT: SpvReflectTypeFlagBits = 4;
pub const SPV_REFLECT_TYPE_FLAG_FLOAT: SpvReflectTypeFlagBits = 8;
pub const SPV_REFLECT_TYPE_FLAG_VECTOR: SpvReflectTypeFlagBits = 256;
pub const SPV_REFLECT_TYPE_FLAG_MATRIX: SpvReflectTypeFlagBits = 512;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_IMAGE: SpvReflectTypeFlagBits = 65536;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLER: SpvReflectTypeFlagBits = 131072;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_SAMPLED_IMAGE: SpvReflectTypeFlagBits = 262144;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_BLOCK: SpvReflectTypeFlagBits = 524288;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_ACCELERATION_STRUCTURE: SpvReflectTypeFlagBits = 1048576;
pub const SPV_REFLECT_TYPE_FLAG_EXTERNAL_MASK: SpvReflectTypeFlagBits = 16711680;
pub const SPV_REFLECT_TYPE_FLAG_STRUCT: SpvReflectTypeFlagBits = 268435456;
pub const SPV_REFLECT_TYPE_FLAG_ARRAY: SpvReflectTypeFlagBits = 536870912;
pub const SPV_REFLECT_TYPE_FLAG_REF: SpvReflectTypeFlagBits = 1073741824;
pub type SpvReflectTypeFlags = u32;

pub type SpvReflectDecorationFlagBits = u32;
pub const SPV_REFLECT_DECORATION_NONE: SpvReflectDecorationFlagBits = 0;
pub const SPV_REFLECT_DECORATION_BLOCK: SpvReflectDecorationFlagBits = 1;
pub const SPV_REFLECT_DECORATION_BUFFER_BLOCK: SpvReflectDecorationFlagBits = 2;
pub const SPV_REFLECT_DECORATION_ROW_MAJOR: SpvReflectDecorationFlagBits = 4;
pub const SPV_REFLECT_DECORATION_COLUMN_MAJOR: SpvReflectDecorationFlagBits = 8;
pub const SPV_REFLECT_DECORATION_BUILT_IN: SpvReflectDecorationFlagBits = 16;
pub const SPV_REFLECT_DECORATION_NOPERSPECTIVE: SpvReflectDecorationFlagBits = 32;
pub const SPV_REFLECT_DECORATION_FLAT: SpvReflectDecorationFlagBits = 64;
pub const SPV_REFLECT_DECORATION_NON_WRITABLE: SpvReflectDecorationFlagBits = 128;
pub const SPV_REFLECT_DECORATION_RELAXED_PRECISION: SpvReflectDecorationFlagBits = 256;
pub const SPV_REFLECT_DECORATION_NON_READABLE: SpvReflectDecorationFlagBits = 512;
pub const SPV_REFLECT_DECORATION_PATCH: SpvReflectDecorationFlagBits = 1024;
pub const SPV_REFLECT_DECORATION_PER_VERTEX: SpvReflectDecorationFlagBits = 2048;
pub const SPV_REFLECT_DECORATION_PER_TASK: SpvReflectDecorationFlagBits = 4096;
pub const SPV_REFLECT_DECORATION_WEIGHT_TEXTURE: SpvReflectDecorationFlagBits = 8192;
pub const SPV_REFLECT_DECORATION_BLOCK_MATCH_TEXTURE: SpvReflectDecorationFlagBits = 16384;
pub type SpvReflectDecorationFlags = u32;

pub type SpvReflectUserType = u32;
pub const SPV_REFLECT_USER_TYPE_INVALID: SpvReflectUserType = 0;
pub const SPV_REFLECT_USER_TYPE_CBUFFER: SpvReflectUserType = 1;
pub const SPV_REFLECT_USER_TYPE_TBUFFER: SpvReflectUserType = 2;
pub const SPV_REFLECT_USER_TYPE_APPEND_STRUCTURED_BUFFER: SpvReflectUserType = 3;
pub const SPV_REFLECT_USER_TYPE_BUFFER: SpvReflectUserType = 4;
pub const SPV_REFLECT_USER_TYPE_BYTE_ADDRESS_BUFFER: SpvReflectUserType = 5;
pub const SPV_REFLECT_USER_TYPE_CONSTANT_BUFFER: SpvReflectUserType = 6;
pub const SPV_REFLECT_USER_TYPE_CONSUME_STRUCTURED_BUFFER: SpvReflectUserType = 7;
pub const SPV_REFLECT_USER_TYPE_INPUT_PATCH: SpvReflectUserType = 8;
pub const SPV_REFLECT_USER_TYPE_OUTPUT_PATCH: SpvReflectUserType = 9;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_BUFFER: SpvReflectUserType = 10;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_BYTE_ADDRESS_BUFFER: SpvReflectUserType = 11;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_STRUCTURED_BUFFER: SpvReflectUserType = 12;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_TEXTURE_1D: SpvReflectUserType = 13;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_TEXTURE_1D_ARRAY: SpvReflectUserType = 14;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_TEXTURE_2D: SpvReflectUserType = 15;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_TEXTURE_2D_ARRAY: SpvReflectUserType = 16;
pub const SPV_REFLECT_USER_TYPE_RASTERIZER_ORDERED_TEXTURE_3D: SpvReflectUserType = 17;
pub const SPV_REFLECT_USER_TYPE_RAYTRACING_ACCELERATION_STRUCTURE: SpvReflectUserType = 18;
pub const SPV_REFLECT_USER_TYPE_RW_BUFFER: SpvReflectUserType = 19;
pub const SPV_REFLECT_USER_TYPE_RW_BYTE_ADDRESS_BUFFER: SpvReflectUserType = 20;
pub const SPV_REFLECT_USER_TYPE_RW_STRUCTURED_BUFFER: SpvReflectUserType = 21;
pub const SPV_REFLECT_USER_TYPE_RW_TEXTURE_1D: SpvReflectUserType = 22;
pub const SPV_REFLECT_USER_TYPE_RW_TEXTURE_1D_ARRAY: SpvReflectUserType = 23;
pub const SPV_REFLECT_USER_TYPE_RW_TEXTURE_2D: SpvReflectUserType = 24;
pub const SPV_REFLECT_USER_TYPE_RW_TEXTURE_2D_ARRAY: SpvReflectUserType = 25;
pub const SPV_REFLECT_USER_TYPE_RW_TEXTURE_3D: SpvReflectUserType = 26;
pub const SPV_REFLECT_USER_TYPE_STRUCTURED_BUFFER: SpvReflectUserType = 27;
pub const SPV_REFLECT_USER_TYPE_SUBPASS_INPUT: SpvReflectUserType = 28;
pub const SPV_REFLECT_USER_TYPE_SUBPASS_INPUT_MS: SpvReflectUserType = 29;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_1D: SpvReflectUserType = 30;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_1D_ARRAY: SpvReflectUserType = 31;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_2D: SpvReflectUserType = 32;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_2D_ARRAY: SpvReflectUserType = 33;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_2DMS: SpvReflectUserType = 34;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_2DMS_ARRAY: SpvReflectUserType = 35;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_3D: SpvReflectUserType = 36;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_BUFFER: SpvReflectUserType = 37;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_CUBE: SpvReflectUserType = 38;
pub const SPV_REFLECT_USER_TYPE_TEXTURE_CUBE_ARRAY: SpvReflectUserType = 39;

pub type SpvReflectResourceType = u32;
pub const SPV_REFLECT_RESOURCE_FLAG_UNDEFINED: SpvReflectResourceType = 0;
pub const SPV_REFLECT_RESOURCE_FLAG_SAMPLER: SpvReflectResourceType = 1;
pub const SPV_REFLECT_RESOURCE_FLAG_CBV: SpvReflectResourceType = 2;
pub const SPV_REFLECT_RESOURCE_FLAG_SRV: SpvReflectResourceType = 4;
pub const SPV_REFLECT_RESOURCE_FLAG_UAV: SpvReflectResourceType = 8;

pub type SpvReflectFormat = u32;
pub const SPV_REFLECT_FORMAT_UNDEFINED: SpvReflectFormat = 0;
pub const SPV_REFLECT_FORMAT_R16_UINT: SpvReflectFormat = 74;
pub const SPV_REFLECT_FORMAT_R16_SINT: SpvReflectFormat = 75;
pub const SPV_REFLECT_FORMAT_R16_SFLOAT: SpvReflectFormat = 76;
pub const SPV_REFLECT_FORMAT_R16G16_UINT: SpvReflectFormat = 81;
pub const SPV_REFLECT_FORMAT_R16G16_SINT: SpvReflectFormat = 82;
pub const SPV_REFLECT_FORMAT_R16G16_SFLOAT: SpvReflectFormat = 83;
pub const SPV_REFLECT_FORMAT_R16G16B16_UINT: SpvReflectFormat = 88;
pub const SPV_REFLECT_FORMAT_R16G16B16_SINT: SpvReflectFormat = 89;
pub const SPV_REFLECT_FORMAT_R16G16B16_SFLOAT: SpvReflectFormat = 90;
pub const SPV_REFLECT_FORMAT_R16G16B16A16_UINT: SpvReflectFormat = 95;
pub const SPV_REFLECT_FORMAT_R16G16B16A16_SINT: SpvReflectFormat = 96;
pub const SPV_REFLECT_FORMAT_R16G16B16A16_SFLOAT: SpvReflectFormat = 97;
pub const SPV_REFLECT_FORMAT_R32_UINT: SpvReflectFormat = 98;
pub const SPV_REFLECT_FORMAT_R32_SINT: SpvReflectFormat = 99;
pub const SPV_REFLECT_FORMAT_R32_SFLOAT: SpvReflectFormat = 100;
pub const SPV_REFLECT_FORMAT_R32G32_UINT: SpvReflectFormat = 101;
pub const SPV_REFLECT_FORMAT_R32G32_SINT: SpvReflectFormat = 102;
pub const SPV_REFLECT_FORMAT_R32G32_SFLOAT: SpvReflectFormat = 103;
pub const SPV_REFLECT_FORMAT_R32G32B32_UINT: SpvReflectFormat = 104;
pub const SPV_REFLECT_FORMAT_R32G32B32_SINT: SpvReflectFormat = 105;
pub const SPV_REFLECT_FORMAT_R32G32B32_SFLOAT: SpvReflectFormat = 106;
pub const SPV_REFLECT_FORMAT_R32G32B32A32_UINT: SpvReflectFormat = 107;
pub const SPV_REFLECT_FORMAT_R32G32B32A32_SINT: SpvReflectFormat = 108;
pub const SPV_REFLECT_FORMAT_R32G32B32A32_SFLOAT: SpvReflectFormat = 109;
pub const SPV_REFLECT_FORMAT_R64_UINT: SpvReflectFormat = 110;
pub const SPV_REFLECT_FORMAT_R64_SINT: SpvReflectFormat = 111;
pub const SPV_REFLECT_FORMAT_R64_SFLOAT: SpvReflectFormat = 112;
pub const SPV_REFLECT_FORMAT_R64G64_UINT: SpvReflectFormat = 113;
pub const SPV_REFLECT_FORMAT_R64G64_SINT: SpvReflectFormat = 114;
pub const SPV_REFLECT_FORMAT_R64G64_SFLOAT: SpvReflectFormat = 115;
pub const SPV_REFLECT_FORMAT_R64G64B64_UINT: SpvReflectFormat = 116;
pub const SPV_REFLECT_FORMAT_R64G64B64_SINT: SpvReflectFormat = 117;
pub const SPV_REFLECT_FORMAT_R64G64B64_SFLOAT: SpvReflectFormat = 118;
pub const SPV_REFLECT_FORMAT_R64G64B64A64_UINT: SpvReflectFormat = 119;
pub const SPV_REFLECT_FORMAT_R64G64B64A64_SINT: SpvReflectFormat = 120;
pub const SPV_REFLECT_FORMAT_R64G64B64A64_SFLOAT: SpvReflectFormat = 121;

pub type SpvReflectVariableFlagBits = u32;
pub const SPV_REFLECT_VARIABLE_FLAGS_NONE: SpvReflectVariableFlagBits = 0;
pub const SPV_REFLECT_VARIABLE_FLAGS_UNUSED: SpvReflectVariableFlagBits = 1;
pub const SPV_REFLECT_VARIABLE_FLAGS_PHYSICAL_POINTER_COPY: SpvReflectVariableFlagBits = 2;
pub type SpvReflectVariableFlags = u32;

pub type SpvReflectDescriptorType = u32;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLER: SpvReflectDescriptorType = 0;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: SpvReflectDescriptorType = 1;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_SAMPLED_IMAGE: SpvReflectDescriptorType = 2;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_IMAGE: SpvReflectDescriptorType = 3;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: SpvReflectDescriptorType = 4;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: SpvReflectDescriptorType = 5;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_BUFFER: SpvReflectDescriptorType = 6;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_BUFFER: SpvReflectDescriptorType = 7;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: SpvReflectDescriptorType = 8;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: SpvReflectDescriptorType = 9;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: SpvReflectDescriptorType = 10;
pub const SPV_REFLECT_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR: SpvReflectDescriptorType = 1000150000;

pub type SpvReflectShaderStageFlagBits = u32;
pub const SPV_REFLECT_SHADER_STAGE_VERTEX_BIT: SpvReflectShaderStageFlagBits = 1;
pub const SPV_REFLECT_SHADER_STAGE_TESSELLATION_CONTROL_BIT: SpvReflectShaderStageFlagBits = 2;
pub const SPV_REFLECT_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: SpvReflectShaderStageFlagBits = 4;
pub const SPV_REFLECT_SHADER_STAGE_GEOMETRY_BIT: SpvReflectShaderStageFlagBits = 8;
pub const SPV_REFLECT_SHADER_STAGE_FRAGMENT_BIT: SpvReflectShaderStageFlagBits = 16;
pub const SPV_REFLECT_SHADER_STAGE_COMPUTE_BIT: SpvReflectShaderStageFlagBits = 32;
pub const SPV_REFLECT_SHADER_STAGE_TASK_BIT_NV: SpvReflectShaderStageFlagBits = 64;
pub const SPV_REFLECT_SHADER_STAGE_TASK_BIT_EXT: SpvReflectShaderStageFlagBits = 64;
pub const SPV_REFLECT_SHADER_STAGE_MESH_BIT_NV: SpvReflectShaderStageFlagBits = 128;
pub const SPV_REFLECT_SHADER_STAGE_MESH_BIT_EXT: SpvReflectShaderStageFlagBits = 128;
pub const SPV_REFLECT_SHADER_STAGE_RAYGEN_BIT_KHR: SpvReflectShaderStageFlagBits = 256;
pub const SPV_REFLECT_SHADER_STAGE_ANY_HIT_BIT_KHR: SpvReflectShaderStageFlagBits = 512;
pub const SPV_REFLECT_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: SpvReflectShaderStageFlagBits = 1024;
pub const SPV_REFLECT_SHADER_STAGE_MISS_BIT_KHR: SpvReflectShaderStageFlagBits = 2048;
pub const SPV_REFLECT_SHADER_STAGE_INTERSECTION_BIT_KHR: SpvReflectShaderStageFlagBits = 4096;
pub const SPV_REFLECT_SHADER_STAGE_CALLABLE_BIT_KHR: SpvReflectShaderStageFlagBits = 8192;

pub type SpvReflectGenerator = u32;
pub const SPV_REFLECT_GENERATOR_KHRONOS_LLVM_SPIRV_TRANSLATOR: SpvReflectGenerator = 6;
pub const SPV_REFLECT_GENERATOR_KHRONOS_SPIRV_TOOLS_ASSEMBLER: SpvReflectGenerator = 7;
pub const SPV_REFLECT_GENERATOR_KHRONOS_GLSLANG_REFERENCE_FRONT_END: SpvReflectGenerator = 8;
pub const SPV_REFLECT_GENERATOR_GOOGLE_SHADERC_OVER_GLSLANG: SpvReflectGenerator = 13;
pub const SPV_REFLECT_GENERATOR_GOOGLE_SPIREGG: SpvReflectGenerator = 14;
pub const SPV_REFLECT_GENERATOR_GOOGLE_RSPIRV: SpvReflectGenerator = 15;
pub const SPV_REFLECT_GENERATOR_X_LEGEND_MESA_MESAIR_SPIRV_TRANSLATOR: SpvReflectGenerator = 16;
pub const SPV_REFLECT_GENERATOR_KHRONOS_SPIRV_TOOLS_LINKER: SpvReflectGenerator = 17;
pub const SPV_REFLECT_GENERATOR_WINE_VKD3D_SHADER_COMPILER: SpvReflectGenerator = 18;
pub const SPV_REFLECT_GENERATOR_CLAY_CLAY_SHADER_COMPILER: SpvReflectGenerator = 19;

pub const SPV_REFLECT_MAX_ARRAY_DIMS: i32 = 32;
pub const SPV_REFLECT_MAX_DESCRIPTOR_SETS: i32 = 64;

pub const SPV_REFLECT_BINDING_NUMBER_DONT_CHANGE: i32 = -1;
pub const SPV_REFLECT_SET_NUMBER_DONT_CHANGE: i32 = -1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectNumericTraits {
	pub scalar: SpvReflectNumericTraitsScalar,
	pub vector: SpvReflectNumericTraitsVector,
	pub matrix: SpvReflectNumericTraitsMatrix,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectNumericTraitsScalar {
	pub width: u32,
	pub signedness: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectNumericTraitsVector {
	pub component_count: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectNumericTraitsMatrix {
	pub column_count: u32,
	pub row_count: u32,
	pub stride: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectImageTraits {
	pub dim: SpvDim,
	pub depth: u32,
	pub arrayed: u32,
	pub ms: u32,
	pub sampled: u32,
	pub image_format: SpvImageFormat,
}

pub type SpvReflectArrayDimType = u32;
pub const SPV_REFLECT_ARRAY_DIM_RUNTIME: SpvReflectArrayDimType = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectArrayTraits {
	pub dims_count: u32,
	pub dims: [u32; 32],
	pub spec_constant_op_ids: [u32; 32],
	pub stride: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectBindingArrayTraits {
	pub dims_count: u32,
	pub dims: [u32; 32],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectTypeDescription {
	pub id: u32,
	pub op: SpvOp,
	pub type_name: *const i8,
	pub struct_member_name: *const i8,
	pub storage_class: i32,
	pub type_flags: SpvReflectTypeFlags,
	pub decoration_flags: SpvReflectDecorationFlags,
	pub traits: SpvReflectTypeDescriptionTraits,
	pub struct_type_description: *mut SpvReflectTypeDescription,
	pub copied: u32,
	pub member_count: u32,
	pub members: *mut SpvReflectTypeDescription,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectTypeDescriptionTraits {
	pub numeric: SpvReflectNumericTraits,
	pub image: SpvReflectImageTraits,
	pub array: SpvReflectArrayTraits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectInterfaceVariable {
	pub spirv_id: u32,
	pub name: *const i8,
	pub location: u32,
	pub component: u32,
	pub storage_class: SpvStorageClass,
	pub semantic: *const i8,
	pub decoration_flags: SpvReflectDecorationFlags,
	pub built_in: i32,
	pub numeric: SpvReflectNumericTraits,
	pub array: SpvReflectArrayTraits,
	pub member_count: u32,
	pub members: *mut SpvReflectInterfaceVariable,
	pub format: SpvReflectFormat,
	pub type_description: *mut SpvReflectTypeDescription,
	pub word_offset: SpvReflectInterfaceVariableAnonStruct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectInterfaceVariableAnonStruct {
	pub location: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectBlockVariable {
	pub spirv_id: u32,
	pub name: *const i8,
	pub offset: u32,
	pub absolute_offset: u32,
	pub size: u32,
	pub padded_size: u32,
	pub decoration_flags: SpvReflectDecorationFlags,
	pub numeric: SpvReflectNumericTraits,
	pub array: SpvReflectArrayTraits,
	pub flags: SpvReflectVariableFlags,
	pub member_count: u32,
	pub members: *mut SpvReflectBlockVariable,
	pub type_description: *mut SpvReflectTypeDescription,
	pub word_offset: SpvReflectBlockVariableAnonStruct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectBlockVariableAnonStruct {
	pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectDescriptorBinding {
	pub spirv_id: u32,
	pub name: *const i8,
	pub binding: u32,
	pub input_attachment_index: u32,
	pub set: u32,
	pub descriptor_type: SpvReflectDescriptorType,
	pub resource_type: SpvReflectResourceType,
	pub image: SpvReflectImageTraits,
	pub block: SpvReflectBlockVariable,
	pub array: SpvReflectBindingArrayTraits,
	pub count: u32,
	pub accessed: u32,
	pub uav_counter_id: u32,
	pub uav_counter_binding: *mut SpvReflectDescriptorBinding,
	pub byte_address_buffer_offset_count: u32,
	pub byte_address_buffer_offsets: *mut u32,
	pub type_description: *mut SpvReflectTypeDescription,
	pub word_offset: SpvReflectDescriptorBindingAnonStruct,
	pub decoration_flags: SpvReflectDecorationFlags,
	pub user_type: SpvReflectUserType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectDescriptorBindingAnonStruct {
	pub binding: u32,
	pub set: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectDescriptorSet {
	pub set: u32,
	pub binding_count: u32,
	pub bindings: *mut *mut SpvReflectDescriptorBinding,
}

pub type SpvReflectExecutionModeValue = u32;
pub const SPV_REFLECT_EXECUTION_MODE_SPEC_CONSTANT: SpvReflectExecutionModeValue = 4294967295;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectEntryPoint {
	pub name: *const i8,
	pub id: u32,
	pub spirv_execution_model: SpvExecutionModel,
	pub shader_stage: SpvReflectShaderStageFlagBits,
	pub input_variable_count: u32,
	pub input_variables: *mut *mut SpvReflectInterfaceVariable,
	pub output_variable_count: u32,
	pub output_variables: *mut *mut SpvReflectInterfaceVariable,
	pub interface_variable_count: u32,
	pub interface_variables: *mut SpvReflectInterfaceVariable,
	pub descriptor_set_count: u32,
	pub descriptor_sets: *mut SpvReflectDescriptorSet,
	pub used_uniform_count: u32,
	pub used_uniforms: *mut u32,
	pub used_push_constant_count: u32,
	pub used_push_constants: *mut u32,
	pub execution_mode_count: u32,
	pub execution_modes: *mut SpvExecutionMode,
	pub local_size: SpvReflectEntryPointLocalSize,
	pub invocations: u32,
	pub output_vertices: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectEntryPointLocalSize {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectCapability {
	pub value: SpvCapability,
	pub word_offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectSpecializationConstant {
	pub spirv_id: u32,
	pub constant_id: u32,
	pub name: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectShaderModule {
	pub generator: SpvReflectGenerator,
	pub entry_point_name: *const i8,
	pub entry_point_id: u32,
	pub entry_point_count: u32,
	pub entry_points: *mut SpvReflectEntryPoint,
	pub source_language: SpvSourceLanguage,
	pub source_language_version: u32,
	pub source_file: *const i8,
	pub source_source: *const i8,
	pub capability_count: u32,
	pub capabilities: *mut SpvReflectCapability,
	pub spirv_execution_model: SpvExecutionModel,
	pub shader_stage: SpvReflectShaderStageFlagBits,
	pub descriptor_binding_count: u32,
	pub descriptor_bindings: *mut SpvReflectDescriptorBinding,
	pub descriptor_set_count: u32,
	pub descriptor_sets: [SpvReflectDescriptorSet; 64],
	pub input_variable_count: u32,
	pub input_variables: *mut *mut SpvReflectInterfaceVariable,
	pub output_variable_count: u32,
	pub output_variables: *mut *mut SpvReflectInterfaceVariable,
	pub interface_variable_count: u32,
	pub interface_variables: *mut SpvReflectInterfaceVariable,
	pub push_constant_block_count: u32,
	pub push_constant_blocks: *mut SpvReflectBlockVariable,
	pub spec_constant_count: u32,
	pub spec_constants: *mut SpvReflectSpecializationConstant,
	pub _internal: *mut SpvReflectShaderModuleInternal,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SpvReflectShaderModuleInternal {
	pub module_flags: SpvReflectModuleFlags,
	pub spirv_size: usize,
	pub spirv_code: *mut u32,
	pub spirv_word_count: u32,
	pub type_description_count: usize,
	pub type_descriptions: *mut SpvReflectTypeDescription,
}

unsafe extern "C" {
	pub fn spvReflectCreateShaderModule(size: usize, p_code: *const (), p_module: *mut SpvReflectShaderModule) -> SpvReflectResult;

	pub fn spvReflectCreateShaderModule2(flags: SpvReflectModuleFlags, size: usize, p_code: *const (), p_module: *mut SpvReflectShaderModule) -> SpvReflectResult;

	pub fn spvReflectGetShaderModule(size: usize, p_code: *const (), p_module: *mut SpvReflectShaderModule) -> SpvReflectResult;

	pub fn spvReflectDestroyShaderModule(p_module: *mut SpvReflectShaderModule);

	pub fn spvReflectGetCodeSize(p_module: *const SpvReflectShaderModule) -> u32;

	pub fn spvReflectGetCode(p_module: *const SpvReflectShaderModule) -> *const u32;

	pub fn spvReflectGetEntryPoint(p_module: *const SpvReflectShaderModule, entry_point: *const i8) -> *const SpvReflectEntryPoint;

	pub fn spvReflectEnumerateDescriptorBindings(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_bindings: *mut *mut SpvReflectDescriptorBinding) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointDescriptorBindings(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_bindings: *mut *mut SpvReflectDescriptorBinding,
	) -> SpvReflectResult;

	pub fn spvReflectEnumerateDescriptorSets(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_sets: *mut *mut SpvReflectDescriptorSet) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointDescriptorSets(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_sets: *mut *mut SpvReflectDescriptorSet,
	) -> SpvReflectResult;

	pub fn spvReflectEnumerateInterfaceVariables(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_variables: *mut *mut SpvReflectInterfaceVariable) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointInterfaceVariables(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_variables: *mut *mut SpvReflectInterfaceVariable,
	) -> SpvReflectResult;

	pub fn spvReflectEnumerateInputVariables(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_variables: *mut *mut SpvReflectInterfaceVariable) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointInputVariables(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_variables: *mut *mut SpvReflectInterfaceVariable,
	) -> SpvReflectResult;

	pub fn spvReflectEnumerateOutputVariables(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_variables: *mut *mut SpvReflectInterfaceVariable) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointOutputVariables(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_variables: *mut *mut SpvReflectInterfaceVariable,
	) -> SpvReflectResult;

	pub fn spvReflectEnumeratePushConstantBlocks(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_blocks: *mut *mut SpvReflectBlockVariable) -> SpvReflectResult;

	pub fn spvReflectEnumeratePushConstants(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_blocks: *mut *mut SpvReflectBlockVariable) -> SpvReflectResult;

	pub fn spvReflectEnumerateEntryPointPushConstantBlocks(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		p_count: *mut u32,
		pp_blocks: *mut *mut SpvReflectBlockVariable,
	) -> SpvReflectResult;

	pub fn spvReflectEnumerateSpecializationConstants(p_module: *const SpvReflectShaderModule, p_count: *mut u32, pp_constants: *mut *mut SpvReflectSpecializationConstant) -> SpvReflectResult;

	pub fn spvReflectGetDescriptorBinding(p_module: *const SpvReflectShaderModule, binding_number: u32, set_number: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectDescriptorBinding;

	pub fn spvReflectGetEntryPointDescriptorBinding(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		binding_number: u32,
		set_number: u32,
		p_result: *mut SpvReflectResult,
	) -> *const SpvReflectDescriptorBinding;

	pub fn spvReflectGetDescriptorSet(p_module: *const SpvReflectShaderModule, set_number: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectDescriptorSet;

	pub fn spvReflectGetEntryPointDescriptorSet(p_module: *const SpvReflectShaderModule, entry_point: *const i8, set_number: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectDescriptorSet;

	pub fn spvReflectGetInputVariableByLocation(p_module: *const SpvReflectShaderModule, location: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetInputVariable(p_module: *const SpvReflectShaderModule, location: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetEntryPointInputVariableByLocation(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		location: u32,
		p_result: *mut SpvReflectResult,
	) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetInputVariableBySemantic(p_module: *const SpvReflectShaderModule, semantic: *const i8, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetEntryPointInputVariableBySemantic(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		semantic: *const i8,
		p_result: *mut SpvReflectResult,
	) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetOutputVariableByLocation(p_module: *const SpvReflectShaderModule, location: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetOutputVariable(p_module: *const SpvReflectShaderModule, location: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetEntryPointOutputVariableByLocation(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		location: u32,
		p_result: *mut SpvReflectResult,
	) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetOutputVariableBySemantic(p_module: *const SpvReflectShaderModule, semantic: *const i8, p_result: *mut SpvReflectResult) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetEntryPointOutputVariableBySemantic(
		p_module: *const SpvReflectShaderModule,
		entry_point: *const i8,
		semantic: *const i8,
		p_result: *mut SpvReflectResult,
	) -> *const SpvReflectInterfaceVariable;

	pub fn spvReflectGetPushConstantBlock(p_module: *const SpvReflectShaderModule, index: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectBlockVariable;

	pub fn spvReflectGetPushConstant(p_module: *const SpvReflectShaderModule, index: u32, p_result: *mut SpvReflectResult) -> *const SpvReflectBlockVariable;

	pub fn spvReflectGetEntryPointPushConstantBlock(p_module: *const SpvReflectShaderModule, entry_point: *const i8, p_result: *mut SpvReflectResult) -> *const SpvReflectBlockVariable;

	pub fn spvReflectChangeDescriptorBindingNumbers(
		p_module: *mut SpvReflectShaderModule,
		p_binding: *const SpvReflectDescriptorBinding,
		new_binding_number: u32,
		new_set_number: u32,
	) -> SpvReflectResult;

	pub fn spvReflectChangeDescriptorBindingNumber(
		p_module: *mut SpvReflectShaderModule,
		p_descriptor_binding: *const SpvReflectDescriptorBinding,
		new_binding_number: u32,
		optional_new_set_number: u32,
	) -> SpvReflectResult;

	pub fn spvReflectChangeDescriptorSetNumber(p_module: *mut SpvReflectShaderModule, p_set: *const SpvReflectDescriptorSet, new_set_number: u32) -> SpvReflectResult;

	pub fn spvReflectChangeInputVariableLocation(p_module: *mut SpvReflectShaderModule, p_input_variable: *const SpvReflectInterfaceVariable, new_location: u32) -> SpvReflectResult;

	pub fn spvReflectChangeOutputVariableLocation(p_module: *mut SpvReflectShaderModule, p_output_variable: *const SpvReflectInterfaceVariable, new_location: u32) -> SpvReflectResult;

	pub fn spvReflectSourceLanguage(source_lang: SpvSourceLanguage) -> *const i8;

	pub fn spvReflectBlockVariableTypeName(p_var: *const SpvReflectBlockVariable) -> *const i8;
}
