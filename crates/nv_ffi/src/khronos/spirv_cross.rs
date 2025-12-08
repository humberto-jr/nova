pub use super::spirv::*;

pub const SPVC_C_API_VERSION_MAJOR: u32 = 0;
pub const SPVC_C_API_VERSION_MINOR: u32 = 67;
pub const SPVC_C_API_VERSION_PATCH: u32 = 0;

pub const SPVC_COMPILER_OPTION_COMMON_BIT: u32 = 16777216;
pub const SPVC_COMPILER_OPTION_GLSL_BIT: u32 = 33554432;
pub const SPVC_COMPILER_OPTION_HLSL_BIT: u32 = 67108864;
pub const SPVC_COMPILER_OPTION_MSL_BIT: u32 = 134217728;
pub const SPVC_COMPILER_OPTION_LANG_BITS: u32 = 251658240;
pub const SPVC_COMPILER_OPTION_ENUM_BITS: u32 = 16777215;

pub const SPVC_MSL_PUSH_CONSTANT_DESC_SET: i32 = -1;
pub const SPVC_MSL_PUSH_CONSTANT_BINDING: u32 = 0;
pub const SPVC_MSL_SWIZZLE_BUFFER_BINDING: i32 = -2;
pub const SPVC_MSL_BUFFER_SIZE_BUFFER_BINDING: i32 = -3;
pub const SPVC_MSL_ARGUMENT_BUFFER_BINDING: i32 = -4;
pub const SPVC_MSL_AUX_BUFFER_STRUCT_VERSION: u32 = 1;

pub const SPVC_HLSL_PUSH_CONSTANT_DESC_SET: i32 = -1;
pub const SPVC_HLSL_PUSH_CONSTANT_BINDING: u32 = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_context_s {
	_unused: [u8; 0],
}

pub type spvc_context = *mut spvc_context_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_parsed_ir_s {
	_unused: [u8; 0],
}

pub type spvc_parsed_ir = *mut spvc_parsed_ir_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_compiler_s {
	_unused: [u8; 0],
}

pub type spvc_compiler = *mut spvc_compiler_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_compiler_options_s {
	_unused: [u8; 0],
}

pub type spvc_compiler_options = *mut spvc_compiler_options_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_resources_s {
	_unused: [u8; 0],
}

pub type spvc_resources = *mut spvc_resources_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_type_s {
	_unused: [u8; 0],
}

pub type spvc_type = *const spvc_type_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_constant_s {
	_unused: [u8; 0],
}

pub type spvc_constant = *mut spvc_constant_s;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_set_s {
	_unused: [u8; 0],
}

pub type spvc_set = *const spvc_set_s;
pub type spvc_type_id = SpvId;
pub type spvc_variable_id = SpvId;
pub type spvc_constant_id = SpvId;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_reflected_resource {
	pub id: spvc_variable_id,
	pub base_type_id: spvc_type_id,
	pub type_id: spvc_type_id,
	pub name: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_reflected_builtin_resource {
	pub builtin: SpvBuiltIn,
	pub value_type_id: spvc_type_id,
	pub resource: spvc_reflected_resource,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_entry_point {
	pub execution_model: SpvExecutionModel,
	pub name: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_combined_image_sampler {
	pub combined_id: spvc_variable_id,
	pub image_id: spvc_variable_id,
	pub sampler_id: spvc_variable_id,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_specialization_constant {
	pub id: spvc_constant_id,
	pub constant_id: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_buffer_range {
	pub index: u32,
	pub offset: usize,
	pub range: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_hlsl_root_constants {
	pub start: u32,
	pub end: u32,
	pub binding: u32,
	pub space: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_hlsl_vertex_attribute_remap {
	pub location: u32,
	pub semantic: *const i8,
}

pub type spvc_bool = u8;

pub type spvc_result = i32;
pub const SPVC_SUCCESS: spvc_result = 0;
pub const SPVC_ERROR_INVALID_SPIRV: spvc_result = -1;
pub const SPVC_ERROR_UNSUPPORTED_SPIRV: spvc_result = -2;
pub const SPVC_ERROR_OUT_OF_MEMORY: spvc_result = -3;
pub const SPVC_ERROR_INVALID_ARGUMENT: spvc_result = -4;
pub const SPVC_ERROR_INT_MAX: spvc_result = 2147483647;

pub type spvc_capture_mode = u32;
pub const SPVC_CAPTURE_MODE_COPY: spvc_capture_mode = 0;
pub const SPVC_CAPTURE_MODE_TAKE_OWNERSHIP: spvc_capture_mode = 1;
pub const SPVC_CAPTURE_MODE_INT_MAX: spvc_capture_mode = 2147483647;

pub type spvc_backend = u32;
pub const SPVC_BACKEND_NONE: spvc_backend = 0;
pub const SPVC_BACKEND_GLSL: spvc_backend = 1;
pub const SPVC_BACKEND_HLSL: spvc_backend = 2;
pub const SPVC_BACKEND_MSL: spvc_backend = 3;
pub const SPVC_BACKEND_CPP: spvc_backend = 4;
pub const SPVC_BACKEND_JSON: spvc_backend = 5;
pub const SPVC_BACKEND_INT_MAX: spvc_backend = 2147483647;

pub type spvc_resource_type = u32;
pub const SPVC_RESOURCE_TYPE_UNKNOWN: spvc_resource_type = 0;
pub const SPVC_RESOURCE_TYPE_UNIFORM_BUFFER: spvc_resource_type = 1;
pub const SPVC_RESOURCE_TYPE_STORAGE_BUFFER: spvc_resource_type = 2;
pub const SPVC_RESOURCE_TYPE_STAGE_INPUT: spvc_resource_type = 3;
pub const SPVC_RESOURCE_TYPE_STAGE_OUTPUT: spvc_resource_type = 4;
pub const SPVC_RESOURCE_TYPE_SUBPASS_INPUT: spvc_resource_type = 5;
pub const SPVC_RESOURCE_TYPE_STORAGE_IMAGE: spvc_resource_type = 6;
pub const SPVC_RESOURCE_TYPE_SAMPLED_IMAGE: spvc_resource_type = 7;
pub const SPVC_RESOURCE_TYPE_ATOMIC_COUNTER: spvc_resource_type = 8;
pub const SPVC_RESOURCE_TYPE_PUSH_CONSTANT: spvc_resource_type = 9;
pub const SPVC_RESOURCE_TYPE_SEPARATE_IMAGE: spvc_resource_type = 10;
pub const SPVC_RESOURCE_TYPE_SEPARATE_SAMPLERS: spvc_resource_type = 11;
pub const SPVC_RESOURCE_TYPE_ACCELERATION_STRUCTURE: spvc_resource_type = 12;
pub const SPVC_RESOURCE_TYPE_RAY_QUERY: spvc_resource_type = 13;
pub const SPVC_RESOURCE_TYPE_SHADER_RECORD_BUFFER: spvc_resource_type = 14;
pub const SPVC_RESOURCE_TYPE_GL_PLAIN_UNIFORM: spvc_resource_type = 15;
pub const SPVC_RESOURCE_TYPE_TENSOR: spvc_resource_type = 16;
pub const SPVC_RESOURCE_TYPE_INT_MAX: spvc_resource_type = 2147483647;

pub type spvc_builtin_resource_type = u32;
pub const SPVC_BUILTIN_RESOURCE_TYPE_UNKNOWN: spvc_builtin_resource_type = 0;
pub const SPVC_BUILTIN_RESOURCE_TYPE_STAGE_INPUT: spvc_builtin_resource_type = 1;
pub const SPVC_BUILTIN_RESOURCE_TYPE_STAGE_OUTPUT: spvc_builtin_resource_type = 2;
pub const SPVC_BUILTIN_RESOURCE_TYPE_INT_MAX: spvc_builtin_resource_type = 2147483647;

pub type spvc_basetype = u32;
pub const SPVC_BASETYPE_UNKNOWN: spvc_basetype = 0;
pub const SPVC_BASETYPE_VOID: spvc_basetype = 1;
pub const SPVC_BASETYPE_BOOLEAN: spvc_basetype = 2;
pub const SPVC_BASETYPE_INT8: spvc_basetype = 3;
pub const SPVC_BASETYPE_UINT8: spvc_basetype = 4;
pub const SPVC_BASETYPE_INT16: spvc_basetype = 5;
pub const SPVC_BASETYPE_UINT16: spvc_basetype = 6;
pub const SPVC_BASETYPE_INT32: spvc_basetype = 7;
pub const SPVC_BASETYPE_UINT32: spvc_basetype = 8;
pub const SPVC_BASETYPE_INT64: spvc_basetype = 9;
pub const SPVC_BASETYPE_UINT64: spvc_basetype = 10;
pub const SPVC_BASETYPE_ATOMIC_COUNTER: spvc_basetype = 11;
pub const SPVC_BASETYPE_FP16: spvc_basetype = 12;
pub const SPVC_BASETYPE_FP32: spvc_basetype = 13;
pub const SPVC_BASETYPE_FP64: spvc_basetype = 14;
pub const SPVC_BASETYPE_STRUCT: spvc_basetype = 15;
pub const SPVC_BASETYPE_IMAGE: spvc_basetype = 16;
pub const SPVC_BASETYPE_SAMPLED_IMAGE: spvc_basetype = 17;
pub const SPVC_BASETYPE_SAMPLER: spvc_basetype = 18;
pub const SPVC_BASETYPE_ACCELERATION_STRUCTURE: spvc_basetype = 19;
pub const SPVC_BASETYPE_INT_MAX: spvc_basetype = 2147483647;

pub type spvc_msl_platform = u32;
pub const SPVC_MSL_PLATFORM_IOS: spvc_msl_platform = 0;
pub const SPVC_MSL_PLATFORM_MACOS: spvc_msl_platform = 1;
pub const SPVC_MSL_PLATFORM_MAX_INT: spvc_msl_platform = 2147483647;

pub type spvc_msl_index_type = u32;
pub const SPVC_MSL_INDEX_TYPE_NONE: spvc_msl_index_type = 0;
pub const SPVC_MSL_INDEX_TYPE_UINT16: spvc_msl_index_type = 1;
pub const SPVC_MSL_INDEX_TYPE_UINT32: spvc_msl_index_type = 2;
pub const SPVC_MSL_INDEX_TYPE_MAX_INT: spvc_msl_index_type = 2147483647;

pub type spvc_msl_shader_variable_format = u32;
pub const SPVC_MSL_SHADER_VARIABLE_FORMAT_OTHER: spvc_msl_shader_variable_format = 0;
pub const SPVC_MSL_SHADER_VARIABLE_FORMAT_UINT8: spvc_msl_shader_variable_format = 1;
pub const SPVC_MSL_SHADER_VARIABLE_FORMAT_UINT16: spvc_msl_shader_variable_format = 2;
pub const SPVC_MSL_SHADER_VARIABLE_FORMAT_ANY16: spvc_msl_shader_variable_format = 3;
pub const SPVC_MSL_SHADER_VARIABLE_FORMAT_ANY32: spvc_msl_shader_variable_format = 4;
pub const SPVC_MSL_VERTEX_FORMAT_OTHER: spvc_msl_shader_variable_format = 0;
pub const SPVC_MSL_VERTEX_FORMAT_UINT8: spvc_msl_shader_variable_format = 1;
pub const SPVC_MSL_VERTEX_FORMAT_UINT16: spvc_msl_shader_variable_format = 2;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_OTHER: spvc_msl_shader_variable_format = 0;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_UINT8: spvc_msl_shader_variable_format = 1;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_UINT16: spvc_msl_shader_variable_format = 2;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_ANY16: spvc_msl_shader_variable_format = 3;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_ANY32: spvc_msl_shader_variable_format = 4;
pub const SPVC_MSL_SHADER_INPUT_FORMAT_INT_MAX: spvc_msl_shader_variable_format = 2147483647;

pub use spvc_msl_shader_variable_format as spvc_msl_shader_input_format;
pub use spvc_msl_shader_variable_format as spvc_msl_vertex_format;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_vertex_attribute {
	pub location: u32,
	pub msl_buffer: u32,
	pub msl_offset: u32,
	pub msl_stride: u32,
	pub per_instance: spvc_bool,
	pub format: spvc_msl_vertex_format,
	pub builtin: SpvBuiltIn,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_shader_interface_var {
	pub location: u32,
	pub format: spvc_msl_vertex_format,
	pub builtin: SpvBuiltIn,
	pub vecsize: u32,
}

pub type spvc_msl_shader_input = spvc_msl_shader_interface_var;

pub type spvc_msl_shader_variable_rate = u32;
pub const SPVC_MSL_SHADER_VARIABLE_RATE_PER_VERTEX: spvc_msl_shader_variable_rate = 0;
pub const SPVC_MSL_SHADER_VARIABLE_RATE_PER_PRIMITIVE: spvc_msl_shader_variable_rate = 1;
pub const SPVC_MSL_SHADER_VARIABLE_RATE_PER_PATCH: spvc_msl_shader_variable_rate = 2;
pub const SPVC_MSL_SHADER_VARIABLE_RATE_INT_MAX: spvc_msl_shader_variable_rate = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_shader_interface_var_2 {
	pub location: u32,
	pub format: spvc_msl_shader_variable_format,
	pub builtin: SpvBuiltIn,
	pub vecsize: u32,
	pub rate: spvc_msl_shader_variable_rate,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_resource_binding {
	pub stage: SpvExecutionModel,
	pub desc_set: u32,
	pub binding: u32,
	pub msl_buffer: u32,
	pub msl_texture: u32,
	pub msl_sampler: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_resource_binding_2 {
	pub stage: SpvExecutionModel,
	pub desc_set: u32,
	pub binding: u32,
	pub count: u32,
	pub msl_buffer: u32,
	pub msl_texture: u32,
	pub msl_sampler: u32,
}

pub type spvc_msl_sampler_coord = u32;
pub const SPVC_MSL_SAMPLER_COORD_NORMALIZED: spvc_msl_sampler_coord = 0;
pub const SPVC_MSL_SAMPLER_COORD_PIXEL: spvc_msl_sampler_coord = 1;
pub const SPVC_MSL_SAMPLER_INT_MAX: spvc_msl_sampler_coord = 2147483647;

pub type spvc_msl_sampler_filter = u32;
pub const SPVC_MSL_SAMPLER_FILTER_NEAREST: spvc_msl_sampler_filter = 0;
pub const SPVC_MSL_SAMPLER_FILTER_LINEAR: spvc_msl_sampler_filter = 1;
pub const SPVC_MSL_SAMPLER_FILTER_INT_MAX: spvc_msl_sampler_filter = 2147483647;

pub type spvc_msl_sampler_mip_filter = u32;
pub const SPVC_MSL_SAMPLER_MIP_FILTER_NONE: spvc_msl_sampler_mip_filter = 0;
pub const SPVC_MSL_SAMPLER_MIP_FILTER_NEAREST: spvc_msl_sampler_mip_filter = 1;
pub const SPVC_MSL_SAMPLER_MIP_FILTER_LINEAR: spvc_msl_sampler_mip_filter = 2;
pub const SPVC_MSL_SAMPLER_MIP_FILTER_INT_MAX: spvc_msl_sampler_mip_filter = 2147483647;

pub type spvc_msl_sampler_address = u32;
pub const SPVC_MSL_SAMPLER_ADDRESS_CLAMP_TO_ZERO: spvc_msl_sampler_address = 0;
pub const SPVC_MSL_SAMPLER_ADDRESS_CLAMP_TO_EDGE: spvc_msl_sampler_address = 1;
pub const SPVC_MSL_SAMPLER_ADDRESS_CLAMP_TO_BORDER: spvc_msl_sampler_address = 2;
pub const SPVC_MSL_SAMPLER_ADDRESS_REPEAT: spvc_msl_sampler_address = 3;
pub const SPVC_MSL_SAMPLER_ADDRESS_MIRRORED_REPEAT: spvc_msl_sampler_address = 4;
pub const SPVC_MSL_SAMPLER_ADDRESS_INT_MAX: spvc_msl_sampler_address = 2147483647;

pub type spvc_msl_sampler_compare_func = u32;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_NEVER: spvc_msl_sampler_compare_func = 0;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_LESS: spvc_msl_sampler_compare_func = 1;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_LESS_EQUAL: spvc_msl_sampler_compare_func = 2;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_GREATER: spvc_msl_sampler_compare_func = 3;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_GREATER_EQUAL: spvc_msl_sampler_compare_func = 4;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_EQUAL: spvc_msl_sampler_compare_func = 5;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_NOT_EQUAL: spvc_msl_sampler_compare_func = 6;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_ALWAYS: spvc_msl_sampler_compare_func = 7;
pub const SPVC_MSL_SAMPLER_COMPARE_FUNC_INT_MAX: spvc_msl_sampler_compare_func = 2147483647;

pub type spvc_msl_sampler_border_color = u32;
pub const SPVC_MSL_SAMPLER_BORDER_COLOR_TRANSPARENT_BLACK: spvc_msl_sampler_border_color = 0;
pub const SPVC_MSL_SAMPLER_BORDER_COLOR_OPAQUE_BLACK: spvc_msl_sampler_border_color = 1;
pub const SPVC_MSL_SAMPLER_BORDER_COLOR_OPAQUE_WHITE: spvc_msl_sampler_border_color = 2;
pub const SPVC_MSL_SAMPLER_BORDER_COLOR_INT_MAX: spvc_msl_sampler_border_color = 2147483647;

pub type spvc_msl_format_resolution = u32;
pub const SPVC_MSL_FORMAT_RESOLUTION_444: spvc_msl_format_resolution = 0;
pub const SPVC_MSL_FORMAT_RESOLUTION_422: spvc_msl_format_resolution = 1;
pub const SPVC_MSL_FORMAT_RESOLUTION_420: spvc_msl_format_resolution = 2;
pub const SPVC_MSL_FORMAT_RESOLUTION_INT_MAX: spvc_msl_format_resolution = 2147483647;

pub type spvc_msl_chroma_location = u32;
pub const SPVC_MSL_CHROMA_LOCATION_COSITED_EVEN: spvc_msl_chroma_location = 0;
pub const SPVC_MSL_CHROMA_LOCATION_MIDPOINT: spvc_msl_chroma_location = 1;
pub const SPVC_MSL_CHROMA_LOCATION_INT_MAX: spvc_msl_chroma_location = 2147483647;

pub type spvc_msl_component_swizzle = u32;
pub const SPVC_MSL_COMPONENT_SWIZZLE_IDENTITY: spvc_msl_component_swizzle = 0;
pub const SPVC_MSL_COMPONENT_SWIZZLE_ZERO: spvc_msl_component_swizzle = 1;
pub const SPVC_MSL_COMPONENT_SWIZZLE_ONE: spvc_msl_component_swizzle = 2;
pub const SPVC_MSL_COMPONENT_SWIZZLE_R: spvc_msl_component_swizzle = 3;
pub const SPVC_MSL_COMPONENT_SWIZZLE_G: spvc_msl_component_swizzle = 4;
pub const SPVC_MSL_COMPONENT_SWIZZLE_B: spvc_msl_component_swizzle = 5;
pub const SPVC_MSL_COMPONENT_SWIZZLE_A: spvc_msl_component_swizzle = 6;
pub const SPVC_MSL_COMPONENT_SWIZZLE_INT_MAX: spvc_msl_component_swizzle = 2147483647;

pub type spvc_msl_sampler_ycbcr_model_conversion = u32;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: spvc_msl_sampler_ycbcr_model_conversion = 0;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: spvc_msl_sampler_ycbcr_model_conversion = 1;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_BT_709: spvc_msl_sampler_ycbcr_model_conversion = 2;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_BT_601: spvc_msl_sampler_ycbcr_model_conversion = 3;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_BT_2020: spvc_msl_sampler_ycbcr_model_conversion = 4;
pub const SPVC_MSL_SAMPLER_YCBCR_MODEL_CONVERSION_INT_MAX: spvc_msl_sampler_ycbcr_model_conversion = 2147483647;

pub type spvc_msl_sampler_ycbcr_range = u32;
pub const SPVC_MSL_SAMPLER_YCBCR_RANGE_ITU_FULL: spvc_msl_sampler_ycbcr_range = 0;
pub const SPVC_MSL_SAMPLER_YCBCR_RANGE_ITU_NARROW: spvc_msl_sampler_ycbcr_range = 1;
pub const SPVC_MSL_SAMPLER_YCBCR_RANGE_INT_MAX: spvc_msl_sampler_ycbcr_range = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_constexpr_sampler {
	pub coord: spvc_msl_sampler_coord,
	pub min_filter: spvc_msl_sampler_filter,
	pub mag_filter: spvc_msl_sampler_filter,
	pub mip_filter: spvc_msl_sampler_mip_filter,
	pub s_address: spvc_msl_sampler_address,
	pub t_address: spvc_msl_sampler_address,
	pub r_address: spvc_msl_sampler_address,
	pub compare_func: spvc_msl_sampler_compare_func,
	pub border_color: spvc_msl_sampler_border_color,
	pub lod_clamp_min: f32,
	pub lod_clamp_max: f32,
	pub max_anisotropy: i32,
	pub compare_enable: spvc_bool,
	pub lod_clamp_enable: spvc_bool,
	pub anisotropy_enable: spvc_bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_msl_sampler_ycbcr_conversion {
	pub planes: u32,
	pub resolution: spvc_msl_format_resolution,
	pub chroma_filter: spvc_msl_sampler_filter,
	pub x_chroma_offset: spvc_msl_chroma_location,
	pub y_chroma_offset: spvc_msl_chroma_location,
	pub swizzle: [spvc_msl_component_swizzle; 4],
	pub ycbcr_model: spvc_msl_sampler_ycbcr_model_conversion,
	pub ycbcr_range: spvc_msl_sampler_ycbcr_range,
	pub bpc: u32,
}

pub type spvc_hlsl_binding_flag_bits = u32;
pub const SPVC_HLSL_BINDING_AUTO_NONE_BIT: spvc_hlsl_binding_flag_bits = 0;
pub const SPVC_HLSL_BINDING_AUTO_PUSH_CONSTANT_BIT: spvc_hlsl_binding_flag_bits = 1;
pub const SPVC_HLSL_BINDING_AUTO_CBV_BIT: spvc_hlsl_binding_flag_bits = 2;
pub const SPVC_HLSL_BINDING_AUTO_SRV_BIT: spvc_hlsl_binding_flag_bits = 4;
pub const SPVC_HLSL_BINDING_AUTO_UAV_BIT: spvc_hlsl_binding_flag_bits = 8;
pub const SPVC_HLSL_BINDING_AUTO_SAMPLER_BIT: spvc_hlsl_binding_flag_bits = 16;
pub const SPVC_HLSL_BINDING_AUTO_ALL: spvc_hlsl_binding_flag_bits = 2147483647;

pub type spvc_hlsl_binding_flags = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_hlsl_resource_binding_mapping {
	pub register_space: u32,
	pub register_binding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct spvc_hlsl_resource_binding {
	pub stage: SpvExecutionModel,
	pub desc_set: u32,
	pub binding: u32,
	pub cbv: spvc_hlsl_resource_binding_mapping,
	pub uav: spvc_hlsl_resource_binding_mapping,
	pub srv: spvc_hlsl_resource_binding_mapping,
	pub sampler: spvc_hlsl_resource_binding_mapping,
}

pub type spvc_compiler_option = u32;
pub const SPVC_COMPILER_OPTION_UNKNOWN: spvc_compiler_option = 0;
pub const SPVC_COMPILER_OPTION_FORCE_TEMPORARY: spvc_compiler_option = 16777217;
pub const SPVC_COMPILER_OPTION_FLATTEN_MULTIDIMENSIONAL_ARRAYS: spvc_compiler_option = 16777218;
pub const SPVC_COMPILER_OPTION_FIXUP_DEPTH_CONVENTION: spvc_compiler_option = 16777219;
pub const SPVC_COMPILER_OPTION_FLIP_VERTEX_Y: spvc_compiler_option = 16777220;
pub const SPVC_COMPILER_OPTION_GLSL_SUPPORT_NONZERO_BASE_INSTANCE: spvc_compiler_option = 33554437;
pub const SPVC_COMPILER_OPTION_GLSL_SEPARATE_SHADER_OBJECTS: spvc_compiler_option = 33554438;
pub const SPVC_COMPILER_OPTION_GLSL_ENABLE_420PACK_EXTENSION: spvc_compiler_option = 33554439;
pub const SPVC_COMPILER_OPTION_GLSL_VERSION: spvc_compiler_option = 33554440;
pub const SPVC_COMPILER_OPTION_GLSL_ES: spvc_compiler_option = 33554441;
pub const SPVC_COMPILER_OPTION_GLSL_VULKAN_SEMANTICS: spvc_compiler_option = 33554442;
pub const SPVC_COMPILER_OPTION_GLSL_ES_DEFAULT_FLOAT_PRECISION_HIGHP: spvc_compiler_option = 33554443;
pub const SPVC_COMPILER_OPTION_GLSL_ES_DEFAULT_INT_PRECISION_HIGHP: spvc_compiler_option = 33554444;
pub const SPVC_COMPILER_OPTION_HLSL_SHADER_MODEL: spvc_compiler_option = 67108877;
pub const SPVC_COMPILER_OPTION_HLSL_POINT_SIZE_COMPAT: spvc_compiler_option = 67108878;
pub const SPVC_COMPILER_OPTION_HLSL_POINT_COORD_COMPAT: spvc_compiler_option = 67108879;
pub const SPVC_COMPILER_OPTION_HLSL_SUPPORT_NONZERO_BASE_VERTEX_BASE_INSTANCE: spvc_compiler_option = 67108880;
pub const SPVC_COMPILER_OPTION_MSL_VERSION: spvc_compiler_option = 134217745;
pub const SPVC_COMPILER_OPTION_MSL_TEXEL_BUFFER_TEXTURE_WIDTH: spvc_compiler_option = 134217746;
pub const SPVC_COMPILER_OPTION_MSL_AUX_BUFFER_INDEX: spvc_compiler_option = 134217747;
pub const SPVC_COMPILER_OPTION_MSL_SWIZZLE_BUFFER_INDEX: spvc_compiler_option = 134217747;
pub const SPVC_COMPILER_OPTION_MSL_INDIRECT_PARAMS_BUFFER_INDEX: spvc_compiler_option = 134217748;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_OUTPUT_BUFFER_INDEX: spvc_compiler_option = 134217749;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_PATCH_OUTPUT_BUFFER_INDEX: spvc_compiler_option = 134217750;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_TESS_FACTOR_OUTPUT_BUFFER_INDEX: spvc_compiler_option = 134217751;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_INPUT_WORKGROUP_INDEX: spvc_compiler_option = 134217752;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_POINT_SIZE_BUILTIN: spvc_compiler_option = 134217753;
pub const SPVC_COMPILER_OPTION_MSL_DISABLE_RASTERIZATION: spvc_compiler_option = 134217754;
pub const SPVC_COMPILER_OPTION_MSL_CAPTURE_OUTPUT_TO_BUFFER: spvc_compiler_option = 134217755;
pub const SPVC_COMPILER_OPTION_MSL_SWIZZLE_TEXTURE_SAMPLES: spvc_compiler_option = 134217756;
pub const SPVC_COMPILER_OPTION_MSL_PAD_FRAGMENT_OUTPUT_COMPONENTS: spvc_compiler_option = 134217757;
pub const SPVC_COMPILER_OPTION_MSL_TESS_DOMAIN_ORIGIN_LOWER_LEFT: spvc_compiler_option = 134217758;
pub const SPVC_COMPILER_OPTION_MSL_PLATFORM: spvc_compiler_option = 134217759;
pub const SPVC_COMPILER_OPTION_MSL_ARGUMENT_BUFFERS: spvc_compiler_option = 134217760;
pub const SPVC_COMPILER_OPTION_GLSL_EMIT_PUSH_CONSTANT_AS_UNIFORM_BUFFER: spvc_compiler_option = 33554465;
pub const SPVC_COMPILER_OPTION_MSL_TEXTURE_BUFFER_NATIVE: spvc_compiler_option = 134217762;
pub const SPVC_COMPILER_OPTION_GLSL_EMIT_UNIFORM_BUFFER_AS_PLAIN_UNIFORMS: spvc_compiler_option = 33554467;
pub const SPVC_COMPILER_OPTION_MSL_BUFFER_SIZE_BUFFER_INDEX: spvc_compiler_option = 134217764;
pub const SPVC_COMPILER_OPTION_EMIT_LINE_DIRECTIVES: spvc_compiler_option = 16777253;
pub const SPVC_COMPILER_OPTION_MSL_MULTIVIEW: spvc_compiler_option = 134217766;
pub const SPVC_COMPILER_OPTION_MSL_VIEW_MASK_BUFFER_INDEX: spvc_compiler_option = 134217767;
pub const SPVC_COMPILER_OPTION_MSL_DEVICE_INDEX: spvc_compiler_option = 134217768;
pub const SPVC_COMPILER_OPTION_MSL_VIEW_INDEX_FROM_DEVICE_INDEX: spvc_compiler_option = 134217769;
pub const SPVC_COMPILER_OPTION_MSL_DISPATCH_BASE: spvc_compiler_option = 134217770;
pub const SPVC_COMPILER_OPTION_MSL_DYNAMIC_OFFSETS_BUFFER_INDEX: spvc_compiler_option = 134217771;
pub const SPVC_COMPILER_OPTION_MSL_TEXTURE_1D_AS_2D: spvc_compiler_option = 134217772;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_BASE_INDEX_ZERO: spvc_compiler_option = 134217773;
pub const SPVC_COMPILER_OPTION_MSL_IOS_FRAMEBUFFER_FETCH_SUBPASS: spvc_compiler_option = 134217774;
pub const SPVC_COMPILER_OPTION_MSL_FRAMEBUFFER_FETCH_SUBPASS: spvc_compiler_option = 134217774;
pub const SPVC_COMPILER_OPTION_MSL_INVARIANT_FP_MATH: spvc_compiler_option = 134217775;
pub const SPVC_COMPILER_OPTION_MSL_EMULATE_CUBEMAP_ARRAY: spvc_compiler_option = 134217776;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_DECORATION_BINDING: spvc_compiler_option = 134217777;
pub const SPVC_COMPILER_OPTION_MSL_FORCE_ACTIVE_ARGUMENT_BUFFER_RESOURCES: spvc_compiler_option = 134217778;
pub const SPVC_COMPILER_OPTION_MSL_FORCE_NATIVE_ARRAYS: spvc_compiler_option = 134217779;
pub const SPVC_COMPILER_OPTION_ENABLE_STORAGE_IMAGE_QUALIFIER_DEDUCTION: spvc_compiler_option = 16777268;
pub const SPVC_COMPILER_OPTION_HLSL_FORCE_STORAGE_BUFFER_AS_UAV: spvc_compiler_option = 67108917;
pub const SPVC_COMPILER_OPTION_FORCE_ZERO_INITIALIZED_VARIABLES: spvc_compiler_option = 16777270;
pub const SPVC_COMPILER_OPTION_HLSL_NONWRITABLE_UAV_TEXTURE_AS_SRV: spvc_compiler_option = 67108919;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_FRAG_OUTPUT_MASK: spvc_compiler_option = 134217784;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_FRAG_DEPTH_BUILTIN: spvc_compiler_option = 134217785;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_FRAG_STENCIL_REF_BUILTIN: spvc_compiler_option = 134217786;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_CLIP_DISTANCE_USER_VARYING: spvc_compiler_option = 134217787;
pub const SPVC_COMPILER_OPTION_HLSL_ENABLE_16BIT_TYPES: spvc_compiler_option = 67108924;
pub const SPVC_COMPILER_OPTION_MSL_MULTI_PATCH_WORKGROUP: spvc_compiler_option = 134217789;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_INPUT_BUFFER_INDEX: spvc_compiler_option = 134217790;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_INDEX_BUFFER_INDEX: spvc_compiler_option = 134217791;
pub const SPVC_COMPILER_OPTION_MSL_VERTEX_FOR_TESSELLATION: spvc_compiler_option = 134217792;
pub const SPVC_COMPILER_OPTION_MSL_VERTEX_INDEX_TYPE: spvc_compiler_option = 134217793;
pub const SPVC_COMPILER_OPTION_GLSL_FORCE_FLATTENED_IO_BLOCKS: spvc_compiler_option = 33554498;
pub const SPVC_COMPILER_OPTION_MSL_MULTIVIEW_LAYERED_RENDERING: spvc_compiler_option = 134217795;
pub const SPVC_COMPILER_OPTION_MSL_ARRAYED_SUBPASS_INPUT: spvc_compiler_option = 134217796;
pub const SPVC_COMPILER_OPTION_MSL_R32UI_LINEAR_TEXTURE_ALIGNMENT: spvc_compiler_option = 134217797;
pub const SPVC_COMPILER_OPTION_MSL_R32UI_ALIGNMENT_CONSTANT_ID: spvc_compiler_option = 134217798;
pub const SPVC_COMPILER_OPTION_HLSL_FLATTEN_MATRIX_VERTEX_INPUT_SEMANTICS: spvc_compiler_option = 67108935;
pub const SPVC_COMPILER_OPTION_MSL_IOS_USE_SIMDGROUP_FUNCTIONS: spvc_compiler_option = 134217800;
pub const SPVC_COMPILER_OPTION_MSL_EMULATE_SUBGROUPS: spvc_compiler_option = 134217801;
pub const SPVC_COMPILER_OPTION_MSL_FIXED_SUBGROUP_SIZE: spvc_compiler_option = 134217802;
pub const SPVC_COMPILER_OPTION_MSL_FORCE_SAMPLE_RATE_SHADING: spvc_compiler_option = 134217803;
pub const SPVC_COMPILER_OPTION_MSL_IOS_SUPPORT_BASE_VERTEX_INSTANCE: spvc_compiler_option = 134217804;
pub const SPVC_COMPILER_OPTION_GLSL_OVR_MULTIVIEW_VIEW_COUNT: spvc_compiler_option = 33554509;
pub const SPVC_COMPILER_OPTION_RELAX_NAN_CHECKS: spvc_compiler_option = 16777294;
pub const SPVC_COMPILER_OPTION_MSL_RAW_BUFFER_TESE_INPUT: spvc_compiler_option = 134217807;
pub const SPVC_COMPILER_OPTION_MSL_SHADER_PATCH_INPUT_BUFFER_INDEX: spvc_compiler_option = 134217808;
pub const SPVC_COMPILER_OPTION_MSL_MANUAL_HELPER_INVOCATION_UPDATES: spvc_compiler_option = 134217809;
pub const SPVC_COMPILER_OPTION_MSL_CHECK_DISCARDED_FRAG_STORES: spvc_compiler_option = 134217810;
pub const SPVC_COMPILER_OPTION_GLSL_ENABLE_ROW_MAJOR_LOAD_WORKAROUND: spvc_compiler_option = 33554515;
pub const SPVC_COMPILER_OPTION_MSL_ARGUMENT_BUFFERS_TIER: spvc_compiler_option = 134217812;
pub const SPVC_COMPILER_OPTION_MSL_SAMPLE_DREF_LOD_ARRAY_AS_GRAD: spvc_compiler_option = 134217813;
pub const SPVC_COMPILER_OPTION_MSL_READWRITE_TEXTURE_FENCES: spvc_compiler_option = 134217814;
pub const SPVC_COMPILER_OPTION_MSL_REPLACE_RECURSIVE_INPUTS: spvc_compiler_option = 134217815;
pub const SPVC_COMPILER_OPTION_MSL_AGX_MANUAL_CUBE_GRAD_FIXUP: spvc_compiler_option = 134217816;
pub const SPVC_COMPILER_OPTION_MSL_FORCE_FRAGMENT_WITH_SIDE_EFFECTS_EXECUTION: spvc_compiler_option = 134217817;
pub const SPVC_COMPILER_OPTION_HLSL_USE_ENTRY_POINT_NAME: spvc_compiler_option = 67108954;
pub const SPVC_COMPILER_OPTION_HLSL_PRESERVE_STRUCTURED_BUFFERS: spvc_compiler_option = 67108955;
pub const SPVC_COMPILER_OPTION_MSL_AUTO_DISABLE_RASTERIZATION: spvc_compiler_option = 134217820;
pub const SPVC_COMPILER_OPTION_MSL_ENABLE_POINT_SIZE_DEFAULT: spvc_compiler_option = 134217821;
pub const SPVC_COMPILER_OPTION_INT_MAX: spvc_compiler_option = 2147483647;

pub type spvc_error_callback = unsafe extern "C" fn(userdata: *mut (), error: *const i8);

#[link(name = "spirv-cross-c-shared")]
unsafe extern "C" {
	pub fn spvc_get_version(major: *mut u32, minor: *mut u32, patch: *mut u32);

	pub fn spvc_get_commit_revision_and_timestamp() -> *const i8;

	pub fn spvc_msl_vertex_attribute_init(attr: *mut spvc_msl_vertex_attribute);

	pub fn spvc_msl_shader_interface_var_init(var: *mut spvc_msl_shader_interface_var);

	pub fn spvc_msl_shader_input_init(input: *mut spvc_msl_shader_input);

	pub fn spvc_msl_shader_interface_var_init_2(var: *mut spvc_msl_shader_interface_var_2);

	pub fn spvc_msl_resource_binding_init(binding: *mut spvc_msl_resource_binding);

	pub fn spvc_msl_resource_binding_init_2(binding: *mut spvc_msl_resource_binding_2);

	pub fn spvc_msl_get_aux_buffer_struct_version() -> u32;

	pub fn spvc_msl_constexpr_sampler_init(sampler: *mut spvc_msl_constexpr_sampler);

	pub fn spvc_msl_sampler_ycbcr_conversion_init(conv: *mut spvc_msl_sampler_ycbcr_conversion);

	pub fn spvc_hlsl_resource_binding_init(binding: *mut spvc_hlsl_resource_binding);

	pub fn spvc_context_create(context: *mut spvc_context) -> spvc_result;

	pub fn spvc_context_destroy(context: spvc_context);

	pub fn spvc_context_release_allocations(context: spvc_context);

	pub fn spvc_context_get_last_error_string(context: spvc_context) -> *const i8;

	pub fn spvc_context_set_error_callback(context: spvc_context, cb: spvc_error_callback, userdata: *mut ());

	pub fn spvc_context_parse_spirv(context: spvc_context, spirv: *const SpvId, word_count: usize, parsed_ir: *mut spvc_parsed_ir) -> spvc_result;

	pub fn spvc_context_create_compiler(context: spvc_context, backend: spvc_backend, parsed_ir: spvc_parsed_ir, mode: spvc_capture_mode, compiler: *mut spvc_compiler) -> spvc_result;

	pub fn spvc_compiler_get_current_id_bound(compiler: spvc_compiler) -> u32;

	pub fn spvc_compiler_create_compiler_options(compiler: spvc_compiler, options: *mut spvc_compiler_options) -> spvc_result;

	pub fn spvc_compiler_options_set_bool(options: spvc_compiler_options, option: spvc_compiler_option, value: spvc_bool) -> spvc_result;

	pub fn spvc_compiler_options_set_uint(options: spvc_compiler_options, option: spvc_compiler_option, value: u32) -> spvc_result;

	pub fn spvc_compiler_install_compiler_options(compiler: spvc_compiler, options: spvc_compiler_options) -> spvc_result;

	pub fn spvc_compiler_compile(compiler: spvc_compiler, source: *mut *const i8) -> spvc_result;

	pub fn spvc_compiler_add_header_line(compiler: spvc_compiler, line: *const i8) -> spvc_result;

	pub fn spvc_compiler_require_extension(compiler: spvc_compiler, ext: *const i8) -> spvc_result;

	pub fn spvc_compiler_get_num_required_extensions(compiler: spvc_compiler) -> usize;

	pub fn spvc_compiler_get_required_extension(compiler: spvc_compiler, index: usize) -> *const i8;

	pub fn spvc_compiler_flatten_buffer_block(compiler: spvc_compiler, id: spvc_variable_id) -> spvc_result;

	pub fn spvc_compiler_variable_is_depth_or_compare(compiler: spvc_compiler, id: spvc_variable_id) -> spvc_bool;

	pub fn spvc_compiler_mask_stage_output_by_location(compiler: spvc_compiler, location: u32, component: u32) -> spvc_result;

	pub fn spvc_compiler_mask_stage_output_by_builtin(compiler: spvc_compiler, builtin: SpvBuiltIn) -> spvc_result;

	pub fn spvc_compiler_hlsl_set_root_constants_layout(compiler: spvc_compiler, constant_info: *const spvc_hlsl_root_constants, count: usize) -> spvc_result;

	pub fn spvc_compiler_hlsl_add_vertex_attribute_remap(compiler: spvc_compiler, remap: *const spvc_hlsl_vertex_attribute_remap, remaps: usize) -> spvc_result;

	pub fn spvc_compiler_hlsl_remap_num_workgroups_builtin(compiler: spvc_compiler) -> spvc_variable_id;

	pub fn spvc_compiler_hlsl_set_resource_binding_flags(compiler: spvc_compiler, flags: spvc_hlsl_binding_flags) -> spvc_result;

	pub fn spvc_compiler_hlsl_add_resource_binding(compiler: spvc_compiler, binding: *const spvc_hlsl_resource_binding) -> spvc_result;

	pub fn spvc_compiler_hlsl_is_resource_used(compiler: spvc_compiler, model: SpvExecutionModel, set: u32, binding: u32) -> spvc_bool;

	pub fn spvc_compiler_msl_is_rasterization_disabled(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_aux_buffer(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_swizzle_buffer(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_buffer_size_buffer(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_output_buffer(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_patch_output_buffer(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_needs_input_threadgroup_mem(compiler: spvc_compiler) -> spvc_bool;

	pub fn spvc_compiler_msl_add_vertex_attribute(compiler: spvc_compiler, attrs: *const spvc_msl_vertex_attribute) -> spvc_result;

	pub fn spvc_compiler_msl_add_resource_binding(compiler: spvc_compiler, binding: *const spvc_msl_resource_binding) -> spvc_result;

	pub fn spvc_compiler_msl_add_resource_binding_2(compiler: spvc_compiler, binding: *const spvc_msl_resource_binding_2) -> spvc_result;

	pub fn spvc_compiler_msl_add_shader_input(compiler: spvc_compiler, input: *const spvc_msl_shader_interface_var) -> spvc_result;

	pub fn spvc_compiler_msl_add_shader_input_2(compiler: spvc_compiler, input: *const spvc_msl_shader_interface_var_2) -> spvc_result;

	pub fn spvc_compiler_msl_add_shader_output(compiler: spvc_compiler, output: *const spvc_msl_shader_interface_var) -> spvc_result;

	pub fn spvc_compiler_msl_add_shader_output_2(compiler: spvc_compiler, output: *const spvc_msl_shader_interface_var_2) -> spvc_result;

	pub fn spvc_compiler_msl_add_discrete_descriptor_set(compiler: spvc_compiler, desc_set: u32) -> spvc_result;

	pub fn spvc_compiler_msl_set_argument_buffer_device_address_space(compiler: spvc_compiler, desc_set: u32, device_address: spvc_bool) -> spvc_result;

	pub fn spvc_compiler_msl_is_vertex_attribute_used(compiler: spvc_compiler, location: u32) -> spvc_bool;

	pub fn spvc_compiler_msl_is_shader_input_used(compiler: spvc_compiler, location: u32) -> spvc_bool;

	pub fn spvc_compiler_msl_is_shader_output_used(compiler: spvc_compiler, location: u32) -> spvc_bool;

	pub fn spvc_compiler_msl_is_resource_used(compiler: spvc_compiler, model: SpvExecutionModel, set: u32, binding: u32) -> spvc_bool;

	pub fn spvc_compiler_msl_remap_constexpr_sampler(compiler: spvc_compiler, id: spvc_variable_id, sampler: *const spvc_msl_constexpr_sampler) -> spvc_result;

	pub fn spvc_compiler_msl_remap_constexpr_sampler_by_binding(compiler: spvc_compiler, desc_set: u32, binding: u32, sampler: *const spvc_msl_constexpr_sampler) -> spvc_result;

	pub fn spvc_compiler_msl_remap_constexpr_sampler_ycbcr(
		compiler: spvc_compiler,
		id: spvc_variable_id,
		sampler: *const spvc_msl_constexpr_sampler,
		conv: *const spvc_msl_sampler_ycbcr_conversion,
	) -> spvc_result;

	pub fn spvc_compiler_msl_remap_constexpr_sampler_by_binding_ycbcr(
		compiler: spvc_compiler,
		desc_set: u32,
		binding: u32,
		sampler: *const spvc_msl_constexpr_sampler,
		conv: *const spvc_msl_sampler_ycbcr_conversion,
	) -> spvc_result;

	pub fn spvc_compiler_msl_set_fragment_output_components(compiler: spvc_compiler, location: u32, components: u32) -> spvc_result;

	pub fn spvc_compiler_msl_get_automatic_resource_binding(compiler: spvc_compiler, id: spvc_variable_id) -> u32;

	pub fn spvc_compiler_msl_get_automatic_resource_binding_secondary(compiler: spvc_compiler, id: spvc_variable_id) -> u32;

	pub fn spvc_compiler_msl_add_dynamic_buffer(compiler: spvc_compiler, desc_set: u32, binding: u32, index: u32) -> spvc_result;

	pub fn spvc_compiler_msl_add_inline_uniform_block(compiler: spvc_compiler, desc_set: u32, binding: u32) -> spvc_result;

	pub fn spvc_compiler_msl_set_combined_sampler_suffix(compiler: spvc_compiler, suffix: *const i8) -> spvc_result;

	pub fn spvc_compiler_msl_get_combined_sampler_suffix(compiler: spvc_compiler) -> *const i8;

	pub fn spvc_compiler_get_active_interface_variables(compiler: spvc_compiler, set: *mut spvc_set) -> spvc_result;

	pub fn spvc_compiler_set_enabled_interface_variables(compiler: spvc_compiler, set: spvc_set) -> spvc_result;

	pub fn spvc_compiler_create_shader_resources(compiler: spvc_compiler, resources: *mut spvc_resources) -> spvc_result;

	pub fn spvc_compiler_create_shader_resources_for_active_variables(compiler: spvc_compiler, resources: *mut spvc_resources, active: spvc_set) -> spvc_result;

	pub fn spvc_resources_get_resource_list_for_type(
		resources: spvc_resources,
		type_: spvc_resource_type,
		resource_list: *mut *const spvc_reflected_resource,
		resource_size: *mut usize,
	) -> spvc_result;

	pub fn spvc_resources_get_builtin_resource_list_for_type(
		resources: spvc_resources,
		type_: spvc_builtin_resource_type,
		resource_list: *mut *const spvc_reflected_builtin_resource,
		resource_size: *mut usize,
	) -> spvc_result;

	pub fn spvc_compiler_set_decoration(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration, argument: u32);

	pub fn spvc_compiler_set_decoration_string(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration, argument: *const i8);

	pub fn spvc_compiler_set_name(compiler: spvc_compiler, id: SpvId, argument: *const i8);

	pub fn spvc_compiler_set_member_decoration(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration, argument: u32);

	pub fn spvc_compiler_set_member_decoration_string(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration, argument: *const i8);

	pub fn spvc_compiler_set_member_name(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, argument: *const i8);

	pub fn spvc_compiler_unset_decoration(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration);

	pub fn spvc_compiler_unset_member_decoration(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration);

	pub fn spvc_compiler_has_decoration(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration) -> spvc_bool;

	pub fn spvc_compiler_has_member_decoration(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration) -> spvc_bool;

	pub fn spvc_compiler_get_name(compiler: spvc_compiler, id: SpvId) -> *const i8;

	pub fn spvc_compiler_get_decoration(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration) -> u32;

	pub fn spvc_compiler_get_decoration_string(compiler: spvc_compiler, id: SpvId, decoration: SpvDecoration) -> *const i8;

	pub fn spvc_compiler_get_member_decoration(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration) -> u32;

	pub fn spvc_compiler_get_member_decoration_string(compiler: spvc_compiler, id: spvc_type_id, member_index: u32, decoration: SpvDecoration) -> *const i8;

	pub fn spvc_compiler_get_member_name(compiler: spvc_compiler, id: spvc_type_id, member_index: u32) -> *const i8;

	pub fn spvc_compiler_get_entry_points(compiler: spvc_compiler, entry_points: *mut *const spvc_entry_point, num_entry_points: *mut usize) -> spvc_result;

	pub fn spvc_compiler_set_entry_point(compiler: spvc_compiler, name: *const i8, model: SpvExecutionModel) -> spvc_result;

	pub fn spvc_compiler_rename_entry_point(compiler: spvc_compiler, old_name: *const i8, new_name: *const i8, model: SpvExecutionModel) -> spvc_result;

	pub fn spvc_compiler_get_cleansed_entry_point_name(compiler: spvc_compiler, name: *const i8, model: SpvExecutionModel) -> *const i8;

	pub fn spvc_compiler_set_execution_mode(compiler: spvc_compiler, mode: SpvExecutionMode);

	pub fn spvc_compiler_unset_execution_mode(compiler: spvc_compiler, mode: SpvExecutionMode);

	pub fn spvc_compiler_set_execution_mode_with_arguments(compiler: spvc_compiler, mode: SpvExecutionMode, arg0: u32, arg1: u32, arg2: u32);

	pub fn spvc_compiler_get_execution_modes(compiler: spvc_compiler, modes: *mut *const SpvExecutionMode, num_modes: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_execution_mode_argument(compiler: spvc_compiler, mode: SpvExecutionMode) -> u32;

	pub fn spvc_compiler_get_execution_mode_argument_by_index(compiler: spvc_compiler, mode: SpvExecutionMode, index: u32) -> u32;

	pub fn spvc_compiler_get_execution_model(compiler: spvc_compiler) -> SpvExecutionModel;

	pub fn spvc_compiler_update_active_builtins(compiler: spvc_compiler);

	pub fn spvc_compiler_has_active_builtin(compiler: spvc_compiler, builtin: SpvBuiltIn, storage: SpvStorageClass) -> spvc_bool;

	pub fn spvc_compiler_get_type_handle(compiler: spvc_compiler, id: spvc_type_id) -> spvc_type;

	pub fn spvc_type_get_base_type_id(type_: spvc_type) -> spvc_type_id;

	pub fn spvc_type_get_basetype(type_: spvc_type) -> spvc_basetype;

	pub fn spvc_type_get_bit_width(type_: spvc_type) -> u32;

	pub fn spvc_type_get_vector_size(type_: spvc_type) -> u32;

	pub fn spvc_type_get_columns(type_: spvc_type) -> u32;

	pub fn spvc_type_get_num_array_dimensions(type_: spvc_type) -> u32;

	pub fn spvc_type_array_dimension_is_literal(type_: spvc_type, dimension: u32) -> spvc_bool;

	pub fn spvc_type_get_array_dimension(type_: spvc_type, dimension: u32) -> SpvId;

	pub fn spvc_type_get_num_member_types(type_: spvc_type) -> u32;

	pub fn spvc_type_get_member_type(type_: spvc_type, index: u32) -> spvc_type_id;

	pub fn spvc_type_get_storage_class(type_: spvc_type) -> SpvStorageClass;

	pub fn spvc_type_get_image_sampled_type(type_: spvc_type) -> spvc_type_id;

	pub fn spvc_type_get_image_dimension(type_: spvc_type) -> SpvDim;

	pub fn spvc_type_get_image_is_depth(type_: spvc_type) -> spvc_bool;

	pub fn spvc_type_get_image_arrayed(type_: spvc_type) -> spvc_bool;

	pub fn spvc_type_get_image_multisampled(type_: spvc_type) -> spvc_bool;

	pub fn spvc_type_get_image_is_storage(type_: spvc_type) -> spvc_bool;

	pub fn spvc_type_get_image_storage_format(type_: spvc_type) -> SpvImageFormat;

	pub fn spvc_type_get_image_access_qualifier(type_: spvc_type) -> SpvAccessQualifier;

	pub fn spvc_compiler_get_declared_struct_size(compiler: spvc_compiler, struct_type: spvc_type, size: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_declared_struct_size_runtime_array(compiler: spvc_compiler, struct_type: spvc_type, array_size: usize, size: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_declared_struct_member_size(compiler: spvc_compiler, type_: spvc_type, index: u32, size: *mut usize) -> spvc_result;

	pub fn spvc_compiler_type_struct_member_offset(compiler: spvc_compiler, type_: spvc_type, index: u32, offset: *mut u32) -> spvc_result;

	pub fn spvc_compiler_type_struct_member_array_stride(compiler: spvc_compiler, type_: spvc_type, index: u32, stride: *mut u32) -> spvc_result;

	pub fn spvc_compiler_type_struct_member_matrix_stride(compiler: spvc_compiler, type_: spvc_type, index: u32, stride: *mut u32) -> spvc_result;

	pub fn spvc_compiler_build_dummy_sampler_for_combined_images(compiler: spvc_compiler, id: *mut spvc_variable_id) -> spvc_result;

	pub fn spvc_compiler_build_combined_image_samplers(compiler: spvc_compiler) -> spvc_result;

	pub fn spvc_compiler_get_combined_image_samplers(compiler: spvc_compiler, samplers: *mut *const spvc_combined_image_sampler, num_samplers: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_specialization_constants(compiler: spvc_compiler, constants: *mut *const spvc_specialization_constant, num_constants: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_constant_handle(compiler: spvc_compiler, id: spvc_constant_id) -> spvc_constant;

	pub fn spvc_compiler_get_work_group_size_specialization_constants(
		compiler: spvc_compiler,
		x: *mut spvc_specialization_constant,
		y: *mut spvc_specialization_constant,
		z: *mut spvc_specialization_constant,
	) -> spvc_constant_id;

	pub fn spvc_compiler_get_active_buffer_ranges(compiler: spvc_compiler, id: spvc_variable_id, ranges: *mut *const spvc_buffer_range, num_ranges: *mut usize) -> spvc_result;

	pub fn spvc_constant_get_scalar_fp16(constant: spvc_constant, column: u32, row: u32) -> f32;

	pub fn spvc_constant_get_scalar_fp32(constant: spvc_constant, column: u32, row: u32) -> f32;

	pub fn spvc_constant_get_scalar_fp64(constant: spvc_constant, column: u32, row: u32) -> f64;

	pub fn spvc_constant_get_scalar_u32(constant: spvc_constant, column: u32, row: u32) -> u32;

	pub fn spvc_constant_get_scalar_i32(constant: spvc_constant, column: u32, row: u32) -> i32;

	pub fn spvc_constant_get_scalar_u16(constant: spvc_constant, column: u32, row: u32) -> u32;

	pub fn spvc_constant_get_scalar_i16(constant: spvc_constant, column: u32, row: u32) -> i32;

	pub fn spvc_constant_get_scalar_u8(constant: spvc_constant, column: u32, row: u32) -> u32;

	pub fn spvc_constant_get_scalar_i8(constant: spvc_constant, column: u32, row: u32) -> i32;

	pub fn spvc_constant_get_subconstants(constant: spvc_constant, constituents: *mut *const spvc_constant_id, count: *mut usize);

	pub fn spvc_constant_get_scalar_u64(constant: spvc_constant, column: u32, row: u32) -> u64;

	pub fn spvc_constant_get_scalar_i64(constant: spvc_constant, column: u32, row: u32) -> i64;

	pub fn spvc_constant_get_type(constant: spvc_constant) -> spvc_type_id;

	pub fn spvc_constant_set_scalar_fp16(constant: spvc_constant, column: u32, row: u32, value: u16);

	pub fn spvc_constant_set_scalar_fp32(constant: spvc_constant, column: u32, row: u32, value: f32);

	pub fn spvc_constant_set_scalar_fp64(constant: spvc_constant, column: u32, row: u32, value: f64);

	pub fn spvc_constant_set_scalar_u32(constant: spvc_constant, column: u32, row: u32, value: u32);

	pub fn spvc_constant_set_scalar_i32(constant: spvc_constant, column: u32, row: u32, value: i32);

	pub fn spvc_constant_set_scalar_u64(constant: spvc_constant, column: u32, row: u32, value: u64);

	pub fn spvc_constant_set_scalar_i64(constant: spvc_constant, column: u32, row: u32, value: i64);

	pub fn spvc_constant_set_scalar_u16(constant: spvc_constant, column: u32, row: u32, value: u16);

	pub fn spvc_constant_set_scalar_i16(constant: spvc_constant, column: u32, row: u32, value: i16);

	pub fn spvc_constant_set_scalar_u8(constant: spvc_constant, column: u32, row: u32, value: u8);

	pub fn spvc_constant_set_scalar_i8(constant: spvc_constant, column: u32, row: u32, value: i8);

	pub fn spvc_compiler_get_binary_offset_for_decoration(compiler: spvc_compiler, id: spvc_variable_id, decoration: SpvDecoration, word_offset: *mut u32) -> spvc_bool;

	pub fn spvc_compiler_buffer_is_hlsl_counter_buffer(compiler: spvc_compiler, id: spvc_variable_id) -> spvc_bool;

	pub fn spvc_compiler_buffer_get_hlsl_counter_buffer(compiler: spvc_compiler, id: spvc_variable_id, counter_id: *mut spvc_variable_id) -> spvc_bool;

	pub fn spvc_compiler_get_declared_capabilities(compiler: spvc_compiler, capabilities: *mut *const SpvCapability, num_capabilities: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_declared_extensions(compiler: spvc_compiler, extensions: *mut *mut *const i8, num_extensions: *mut usize) -> spvc_result;

	pub fn spvc_compiler_get_remapped_declared_block_name(compiler: spvc_compiler, id: spvc_variable_id) -> *const i8;

	pub fn spvc_compiler_get_buffer_block_decorations(compiler: spvc_compiler, id: spvc_variable_id, decorations: *mut *const SpvDecoration, num_decorations: *mut usize) -> spvc_result;
}
