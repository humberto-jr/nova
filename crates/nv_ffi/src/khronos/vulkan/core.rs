// NOTE: These bindings are from the vulkan_core.h header file, version 211,
// released in the Vulkan SDK version 1.3.211 on April 18, 2022.
//
// Original file:
// https://github.com/KhronosGroup/Vulkan-Headers/blob/23842a31df9c9c2b3bc7c6c2bb56044bc5e51c05/include/vulkan/vulkan_core.h

pub const VK_VERSION_1_0: u32 = 1;
pub const VK_VERSION_1_1: u32 = 1;
pub const VK_VERSION_1_2: u32 = 1;
pub const VK_VERSION_1_3: u32 = 1;

pub const VK_HEADER_VERSION: u32 = 211;

pub const VK_API_VERSION_1_0: u32 = (0 << 29) | (1 << 22);

pub const VK_USE_64_BIT_PTR_DEFINES: u32 = 1;
pub const VK_ATTACHMENT_UNUSED: u32 = !0;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
pub const VK_SUBPASS_EXTERNAL: u32 = !0;

pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0;

pub const VK_WHOLE_SIZE: u64 = !0;
pub const VK_UUID_SIZE: u32 = 16;
pub const VK_LUID_SIZE: u32 = 8;
pub const VK_LUID_SIZE_KHR: u32 = 8;

pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0;
pub const VK_QUEUE_FAMILY_EXTERNAL: i32 = -2;
pub const VK_QUEUE_FAMILY_EXTERNAL_KHR: i32 = -2;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: i32 = -3;

pub const VK_SHADER_UNUSED_KHR: u32 = !0;
pub const VK_SHADER_UNUSED_NV: i32 = -1;

pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;

pub const VK_FALSE: u32 = 0;
pub const VK_TRUE: u32 = 1;

pub const VK_KHR_surface: u32 = 1;
pub const VK_KHR_SURFACE_SPEC_VERSION: u32 = 25;
pub const VK_KHR_SURFACE_EXTENSION_NAME: &[u8; 15] = b"VK_KHR_surface\0";

pub const VK_KHR_swapchain: u32 = 1;
pub const VK_KHR_SWAPCHAIN_SPEC_VERSION: u32 = 70;
pub const VK_KHR_SWAPCHAIN_EXTENSION_NAME: &[u8; 17] = b"VK_KHR_swapchain\0";

pub const VK_KHR_display: u32 = 1;
pub const VK_KHR_DISPLAY_SPEC_VERSION: u32 = 23;
pub const VK_KHR_DISPLAY_EXTENSION_NAME: &[u8; 15] = b"VK_KHR_display\0";

pub const VK_KHR_display_swapchain: u32 = 1;
pub const VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
pub const VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_display_swapchain\0";

pub const VK_KHR_sampler_mirror_clamp_to_edge: u32 = 1;
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_SPEC_VERSION: u32 = 3;
pub const VK_KHR_SAMPLER_MIRROR_CLAMP_TO_EDGE_EXTENSION_NAME: &[u8; 36] = b"VK_KHR_sampler_mirror_clamp_to_edge\0";

pub const VK_KHR_dynamic_rendering: u32 = 1;
pub const VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_dynamic_rendering\0";

pub const VK_KHR_multiview: u32 = 1;
pub const VK_KHR_MULTIVIEW_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MULTIVIEW_EXTENSION_NAME: &[u8; 17] = b"VK_KHR_multiview\0";

pub const VK_KHR_get_physical_device_properties2: u32 = 1;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_SPEC_VERSION: u32 = 2;
pub const VK_KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME: &[u8; 39] = b"VK_KHR_get_physical_device_properties2\0";

pub const VK_KHR_device_group: u32 = 1;
pub const VK_KHR_DEVICE_GROUP_SPEC_VERSION: u32 = 4;
pub const VK_KHR_DEVICE_GROUP_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_device_group\0";

pub const VK_KHR_shader_draw_parameters: u32 = 1;
pub const VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME: &[u8; 30] = b"VK_KHR_shader_draw_parameters\0";

pub const VK_KHR_maintenance1: u32 = 1;
pub const VK_KHR_MAINTENANCE_1_SPEC_VERSION: u32 = 2;
pub const VK_KHR_MAINTENANCE_1_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance1\0";
pub const VK_KHR_MAINTENANCE1_SPEC_VERSION: u32 = 2;
pub const VK_KHR_MAINTENANCE1_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance1\0";

pub const VK_KHR_device_group_creation: u32 = 1;
pub const VK_KHR_DEVICE_GROUP_CREATION_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DEVICE_GROUP_CREATION_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_device_group_creation\0";
pub const VK_MAX_DEVICE_GROUP_SIZE_KHR: u32 = 32;

pub const VK_KHR_external_memory_capabilities: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &[u8; 36] = b"VK_KHR_external_memory_capabilities\0";

pub const VK_KHR_external_memory: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &[u8; 23] = b"VK_KHR_external_memory\0";

pub const VK_KHR_external_memory_fd: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &[u8; 26] = b"VK_KHR_external_memory_fd\0";

pub const VK_KHR_external_semaphore_capabilities: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_CAPABILITIES_EXTENSION_NAME: &[u8; 39] = b"VK_KHR_external_semaphore_capabilities\0";

pub const VK_KHR_external_semaphore: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &[u8; 26] = b"VK_KHR_external_semaphore\0";

pub const VK_KHR_external_semaphore_fd: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_external_semaphore_fd\0";

pub const VK_KHR_push_descriptor: u32 = 1;
pub const VK_KHR_PUSH_DESCRIPTOR_SPEC_VERSION: u32 = 2;
pub const VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME: &[u8; 23] = b"VK_KHR_push_descriptor\0";

pub const VK_KHR_shader_float16_int8: u32 = 1;
pub const VK_KHR_SHADER_FLOAT16_INT8_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_FLOAT16_INT8_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_shader_float16_int8\0";

pub const VK_KHR_16bit_storage: u32 = 1;
pub const VK_KHR_16BIT_STORAGE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_16BIT_STORAGE_EXTENSION_NAME: &[u8; 21] = b"VK_KHR_16bit_storage\0";

pub const VK_KHR_incremental_present: u32 = 1;
pub const VK_KHR_INCREMENTAL_PRESENT_SPEC_VERSION: u32 = 2;
pub const VK_KHR_INCREMENTAL_PRESENT_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_incremental_present\0";

pub const VK_KHR_descriptor_update_template: u32 = 1;
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME: &[u8; 34] = b"VK_KHR_descriptor_update_template\0";

pub const VK_KHR_imageless_framebuffer: u32 = 1;
pub const VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
pub const VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_imageless_framebuffer\0";

pub const VK_KHR_create_renderpass2: u32 = 1;
pub const VK_KHR_CREATE_RENDERPASS_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_CREATE_RENDERPASS_2_EXTENSION_NAME: &[u8; 26] = b"VK_KHR_create_renderpass2\0";

pub const VK_KHR_shared_presentable_image: u32 = 1;
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_shared_presentable_image\0";

pub const VK_KHR_external_fence_capabilities: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_CAPABILITIES_EXTENSION_NAME: &[u8; 35] = b"VK_KHR_external_fence_capabilities\0";

pub const VK_KHR_external_fence: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME: &[u8; 22] = b"VK_KHR_external_fence\0";

pub const VK_KHR_external_fence_fd: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
pub const VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_external_fence_fd\0";

pub const VK_KHR_performance_query: u32 = 1;
pub const VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_performance_query\0";

pub const VK_KHR_maintenance2: u32 = 1;
pub const VK_KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE_2_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance2\0";
pub const VK_KHR_MAINTENANCE2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE2_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance2\0";

pub const VK_KHR_get_surface_capabilities2: u32 = 1;
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_SURFACE_CAPABILITIES_2_EXTENSION_NAME: &[u8; 33] = b"VK_KHR_get_surface_capabilities2\0";

pub const VK_KHR_variable_pointers: u32 = 1;
pub const VK_KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_variable_pointers\0";

pub const VK_KHR_get_display_properties2: u32 = 1;
pub const VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME: &[u8; 31] = b"VK_KHR_get_display_properties2\0";

pub const VK_KHR_dedicated_allocation: u32 = 1;
pub const VK_KHR_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 3;
pub const VK_KHR_DEDICATED_ALLOCATION_EXTENSION_NAME: &[u8; 28] = b"VK_KHR_dedicated_allocation\0";

pub const VK_KHR_storage_buffer_storage_class: u32 = 1;
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_STORAGE_BUFFER_STORAGE_CLASS_EXTENSION_NAME: &[u8; 36] = b"VK_KHR_storage_buffer_storage_class\0";

pub const VK_KHR_relaxed_block_layout: u32 = 1;
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_RELAXED_BLOCK_LAYOUT_EXTENSION_NAME: &[u8; 28] = b"VK_KHR_relaxed_block_layout\0";

pub const VK_KHR_get_memory_requirements2: u32 = 1;
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GET_MEMORY_REQUIREMENTS_2_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_get_memory_requirements2\0";

pub const VK_KHR_image_format_list: u32 = 1;
pub const VK_KHR_IMAGE_FORMAT_LIST_SPEC_VERSION: u32 = 1;
pub const VK_KHR_IMAGE_FORMAT_LIST_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_image_format_list\0";

pub const VK_KHR_sampler_ycbcr_conversion: u32 = 1;
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_SPEC_VERSION: u32 = 14;
pub const VK_KHR_SAMPLER_YCBCR_CONVERSION_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_sampler_ycbcr_conversion\0";

pub const VK_KHR_bind_memory2: u32 = 1;
pub const VK_KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_BIND_MEMORY_2_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_bind_memory2\0";

pub const VK_KHR_maintenance3: u32 = 1;
pub const VK_KHR_MAINTENANCE_3_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE_3_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance3\0";
pub const VK_KHR_MAINTENANCE3_SPEC_VERSION: u32 = 1;
pub const VK_KHR_MAINTENANCE3_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance3\0";

pub const VK_KHR_draw_indirect_count: u32 = 1;
pub const VK_KHR_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_draw_indirect_count\0";

pub const VK_KHR_shader_subgroup_extended_types: u32 = 1;
pub const VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME: &[u8; 38] = b"VK_KHR_shader_subgroup_extended_types\0";

pub const VK_KHR_8bit_storage: u32 = 1;
pub const VK_KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_8BIT_STORAGE_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_8bit_storage\0";

pub const VK_KHR_shader_atomic_int64: u32 = 1;
pub const VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_shader_atomic_int64\0";

pub const VK_KHR_shader_clock: u32 = 1;
pub const VK_KHR_SHADER_CLOCK_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_CLOCK_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_shader_clock\0";

pub const VK_KHR_global_priority: u32 = 1;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE_KHR: u32 = 16;
pub const VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &[u8; 23] = b"VK_KHR_global_priority\0";

pub const VK_KHR_driver_properties: u32 = 1;
pub const VK_KHR_DRIVER_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DRIVER_PROPERTIES_EXTENSION_NAME: &[u8; 25] = b"VK_KHR_driver_properties\0";

pub const VK_MAX_DRIVER_NAME_SIZE_KHR: u32 = 256;
pub const VK_MAX_DRIVER_INFO_SIZE_KHR: u32 = 256;

pub const VK_KHR_shader_float_controls: u32 = 1;
pub const VK_KHR_SHADER_FLOAT_CONTROLS_SPEC_VERSION: u32 = 4;
pub const VK_KHR_SHADER_FLOAT_CONTROLS_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_shader_float_controls\0";

pub const VK_KHR_depth_stencil_resolve: u32 = 1;
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_DEPTH_STENCIL_RESOLVE_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_depth_stencil_resolve\0";

pub const VK_KHR_swapchain_mutable_format: u32 = 1;
pub const VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SWAPCHAIN_MUTABLE_FORMAT_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_swapchain_mutable_format\0";

pub const VK_KHR_timeline_semaphore: u32 = 1;
pub const VK_KHR_TIMELINE_SEMAPHORE_SPEC_VERSION: u32 = 2;
pub const VK_KHR_TIMELINE_SEMAPHORE_EXTENSION_NAME: &[u8; 26] = b"VK_KHR_timeline_semaphore\0";

pub const VK_KHR_vulkan_memory_model: u32 = 1;
pub const VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
pub const VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: &[u8; 27] = b"VK_KHR_vulkan_memory_model\0";

pub const VK_KHR_shader_terminate_invocation: u32 = 1;
pub const VK_KHR_SHADER_TERMINATE_INVOCATION_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_TERMINATE_INVOCATION_EXTENSION_NAME: &[u8; 35] = b"VK_KHR_shader_terminate_invocation\0";

pub const VK_KHR_fragment_shading_rate: u32 = 1;
pub const VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
pub const VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_fragment_shading_rate\0";

pub const VK_KHR_spirv_1_4: u32 = 1;
pub const VK_KHR_SPIRV_1_4_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SPIRV_1_4_EXTENSION_NAME: &[u8; 17] = b"VK_KHR_spirv_1_4\0";

pub const VK_KHR_surface_protected_capabilities: u32 = 1;
pub const VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME: &[u8; 38] = b"VK_KHR_surface_protected_capabilities\0";

pub const VK_KHR_separate_depth_stencil_layouts: u32 = 1;
pub const VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: &[u8; 38] = b"VK_KHR_separate_depth_stencil_layouts\0";

pub const VK_KHR_present_wait: u32 = 1;
pub const VK_KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PRESENT_WAIT_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_present_wait\0";

pub const VK_KHR_uniform_buffer_standard_layout: u32 = 1;
pub const VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_UNIFORM_BUFFER_STANDARD_LAYOUT_EXTENSION_NAME: &[u8; 38] = b"VK_KHR_uniform_buffer_standard_layout\0";

pub const VK_KHR_buffer_device_address: u32 = 1;
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 1;
pub const VK_KHR_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_buffer_device_address\0";

pub const VK_KHR_deferred_host_operations: u32 = 1;
pub const VK_KHR_DEFERRED_HOST_OPERATIONS_SPEC_VERSION: u32 = 4;
pub const VK_KHR_DEFERRED_HOST_OPERATIONS_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_deferred_host_operations\0";

pub const VK_KHR_pipeline_executable_properties: u32 = 1;
pub const VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PIPELINE_EXECUTABLE_PROPERTIES_EXTENSION_NAME: &[u8; 38] = b"VK_KHR_pipeline_executable_properties\0";

pub const VK_KHR_shader_integer_dot_product: u32 = 1;
pub const VK_KHR_SHADER_INTEGER_DOT_PRODUCT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_INTEGER_DOT_PRODUCT_EXTENSION_NAME: &[u8; 34] = b"VK_KHR_shader_integer_dot_product\0";

pub const VK_KHR_pipeline_library: u32 = 1;
pub const VK_KHR_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PIPELINE_LIBRARY_EXTENSION_NAME: &[u8; 24] = b"VK_KHR_pipeline_library\0";

pub const VK_KHR_shader_non_semantic_info: u32 = 1;
pub const VK_KHR_SHADER_NON_SEMANTIC_INFO_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_NON_SEMANTIC_INFO_EXTENSION_NAME: &[u8; 32] = b"VK_KHR_shader_non_semantic_info\0";

pub const VK_KHR_present_id: u32 = 1;
pub const VK_KHR_PRESENT_ID_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PRESENT_ID_EXTENSION_NAME: &[u8; 18] = b"VK_KHR_present_id\0";

pub const VK_KHR_synchronization2: u32 = 1;
pub const VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &[u8; 24] = b"VK_KHR_synchronization2\0";

pub const VK_KHR_shader_subgroup_uniform_control_flow: u32 = 1;
pub const VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_SPEC_VERSION: u32 = 1;
pub const VK_KHR_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_EXTENSION_NAME: &[u8; 44] = b"VK_KHR_shader_subgroup_uniform_control_flow\0";

pub const VK_KHR_zero_initialize_workgroup_memory: u32 = 1;
pub const VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_ZERO_INITIALIZE_WORKGROUP_MEMORY_EXTENSION_NAME: &[u8; 40] = b"VK_KHR_zero_initialize_workgroup_memory\0";

pub const VK_KHR_workgroup_memory_explicit_layout: u32 = 1;
pub const VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION: u32 = 1;
pub const VK_KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME: &[u8; 40] = b"VK_KHR_workgroup_memory_explicit_layout\0";

pub const VK_KHR_copy_commands2: u32 = 1;
pub const VK_KHR_COPY_COMMANDS_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_COPY_COMMANDS_2_EXTENSION_NAME: &[u8; 22] = b"VK_KHR_copy_commands2\0";

pub const VK_KHR_format_feature_flags2: u32 = 1;
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 1;
pub const VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME: &[u8; 29] = b"VK_KHR_format_feature_flags2\0";

pub const VK_KHR_portability_enumeration: u32 = 1;
pub const VK_KHR_PORTABILITY_ENUMERATION_SPEC_VERSION: u32 = 1;
pub const VK_KHR_PORTABILITY_ENUMERATION_EXTENSION_NAME: &[u8; 31] = b"VK_KHR_portability_enumeration\0";

pub const VK_KHR_maintenance4: u32 = 1;
pub const VK_KHR_MAINTENANCE_4_SPEC_VERSION: u32 = 2;
pub const VK_KHR_MAINTENANCE_4_EXTENSION_NAME: &[u8; 20] = b"VK_KHR_maintenance4\0";

pub const VK_EXT_debug_report: u32 = 1;
pub const VK_EXT_DEBUG_REPORT_SPEC_VERSION: u32 = 10;
pub const VK_EXT_DEBUG_REPORT_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_debug_report\0";

pub const VK_NV_glsl_shader: u32 = 1;
pub const VK_NV_GLSL_SHADER_SPEC_VERSION: u32 = 1;
pub const VK_NV_GLSL_SHADER_EXTENSION_NAME: &[u8; 18] = b"VK_NV_glsl_shader\0";

pub const VK_EXT_depth_range_unrestricted: u32 = 1;
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DEPTH_RANGE_UNRESTRICTED_EXTENSION_NAME: &[u8; 32] = b"VK_EXT_depth_range_unrestricted\0";

pub const VK_IMG_filter_cubic: u32 = 1;
pub const VK_IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;
pub const VK_IMG_FILTER_CUBIC_EXTENSION_NAME: &[u8; 20] = b"VK_IMG_filter_cubic\0";

pub const VK_AMD_rasterization_order: u32 = 1;
pub const VK_AMD_RASTERIZATION_ORDER_SPEC_VERSION: u32 = 1;
pub const VK_AMD_RASTERIZATION_ORDER_EXTENSION_NAME: &[u8; 27] = b"VK_AMD_rasterization_order\0";

pub const VK_AMD_shader_trinary_minmax: u32 = 1;
pub const VK_AMD_SHADER_TRINARY_MINMAX_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_TRINARY_MINMAX_EXTENSION_NAME: &[u8; 29] = b"VK_AMD_shader_trinary_minmax\0";

pub const VK_AMD_shader_explicit_vertex_parameter: u32 = 1;
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_EXPLICIT_VERTEX_PARAMETER_EXTENSION_NAME: &[u8; 40] = b"VK_AMD_shader_explicit_vertex_parameter\0";

pub const VK_EXT_debug_marker: u32 = 1;
pub const VK_EXT_DEBUG_MARKER_SPEC_VERSION: u32 = 4;
pub const VK_EXT_DEBUG_MARKER_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_debug_marker\0";

pub const VK_AMD_gcn_shader: u32 = 1;
pub const VK_AMD_GCN_SHADER_SPEC_VERSION: u32 = 1;
pub const VK_AMD_GCN_SHADER_EXTENSION_NAME: &[u8; 18] = b"VK_AMD_gcn_shader\0";

pub const VK_NV_dedicated_allocation: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_SPEC_VERSION: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_EXTENSION_NAME: &[u8; 27] = b"VK_NV_dedicated_allocation\0";

pub const VK_EXT_transform_feedback: u32 = 1;
pub const VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
pub const VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_transform_feedback\0";

pub const VK_NVX_binary_import: u32 = 1;
pub const VK_NVX_BINARY_IMPORT_SPEC_VERSION: u32 = 1;
pub const VK_NVX_BINARY_IMPORT_EXTENSION_NAME: &[u8; 21] = b"VK_NVX_binary_import\0";

pub const VK_NVX_image_view_handle: u32 = 1;
pub const VK_NVX_IMAGE_VIEW_HANDLE_SPEC_VERSION: u32 = 2;
pub const VK_NVX_IMAGE_VIEW_HANDLE_EXTENSION_NAME: &[u8; 25] = b"VK_NVX_image_view_handle\0";

pub const VK_AMD_draw_indirect_count: u32 = 1;
pub const VK_AMD_DRAW_INDIRECT_COUNT_SPEC_VERSION: u32 = 2;
pub const VK_AMD_DRAW_INDIRECT_COUNT_EXTENSION_NAME: &[u8; 27] = b"VK_AMD_draw_indirect_count\0";

pub const VK_AMD_negative_viewport_height: u32 = 1;
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION: u32 = 1;
pub const VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME: &[u8; 32] = b"VK_AMD_negative_viewport_height\0";

pub const VK_AMD_gpu_shader_half_float: u32 = 1;
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_SPEC_VERSION: u32 = 2;
pub const VK_AMD_GPU_SHADER_HALF_FLOAT_EXTENSION_NAME: &[u8; 29] = b"VK_AMD_gpu_shader_half_float\0";

pub const VK_AMD_shader_ballot: u32 = 1;
pub const VK_AMD_SHADER_BALLOT_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_BALLOT_EXTENSION_NAME: &[u8; 21] = b"VK_AMD_shader_ballot\0";

pub const VK_AMD_texture_gather_bias_lod: u32 = 1;
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_SPEC_VERSION: u32 = 1;
pub const VK_AMD_TEXTURE_GATHER_BIAS_LOD_EXTENSION_NAME: &[u8; 31] = b"VK_AMD_texture_gather_bias_lod\0";

pub const VK_AMD_shader_info: u32 = 1;
pub const VK_AMD_SHADER_INFO_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_INFO_EXTENSION_NAME: &[u8; 19] = b"VK_AMD_shader_info\0";

pub const VK_AMD_shader_image_load_store_lod: u32 = 1;
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_IMAGE_LOAD_STORE_LOD_EXTENSION_NAME: &[u8; 35] = b"VK_AMD_shader_image_load_store_lod\0";

pub const VK_NV_corner_sampled_image: u32 = 1;
pub const VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
pub const VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &[u8; 27] = b"VK_NV_corner_sampled_image\0";

pub const VK_IMG_format_pvrtc: u32 = 1;
pub const VK_IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
pub const VK_IMG_FORMAT_PVRTC_EXTENSION_NAME: &[u8; 20] = b"VK_IMG_format_pvrtc\0";

pub const VK_NV_external_memory_capabilities: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_CAPABILITIES_EXTENSION_NAME: &[u8; 35] = b"VK_NV_external_memory_capabilities\0";

pub const VK_NV_external_memory: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_EXTENSION_NAME: &[u8; 22] = b"VK_NV_external_memory\0";

pub const VK_EXT_validation_flags: u32 = 1;
pub const VK_EXT_VALIDATION_FLAGS_SPEC_VERSION: u32 = 2;
pub const VK_EXT_VALIDATION_FLAGS_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_validation_flags\0";

pub const VK_EXT_shader_subgroup_ballot: u32 = 1;
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME: &[u8; 30] = b"VK_EXT_shader_subgroup_ballot\0";

pub const VK_EXT_shader_subgroup_vote: u32 = 1;
pub const VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_shader_subgroup_vote\0";

pub const VK_EXT_texture_compression_astc_hdr: u32 = 1;
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_SPEC_VERSION: u32 = 1;
pub const VK_EXT_TEXTURE_COMPRESSION_ASTC_HDR_EXTENSION_NAME: &[u8; 36] = b"VK_EXT_texture_compression_astc_hdr\0";

pub const VK_EXT_astc_decode_mode: u32 = 1;
pub const VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_astc_decode_mode\0";

pub const VK_EXT_conditional_rendering: u32 = 1;
pub const VK_EXT_CONDITIONAL_RENDERING_SPEC_VERSION: u32 = 2;
pub const VK_EXT_CONDITIONAL_RENDERING_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_conditional_rendering\0";

pub const VK_NV_clip_space_w_scaling: u32 = 1;
pub const VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
pub const VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &[u8; 27] = b"VK_NV_clip_space_w_scaling\0";

pub const VK_EXT_direct_mode_display: u32 = 1;
pub const VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_direct_mode_display\0";

pub const VK_EXT_display_surface_counter: u32 = 1;
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME: &[u8; 31] = b"VK_EXT_display_surface_counter\0";

pub const VK_EXT_display_control: u32 = 1;
pub const VK_EXT_DISPLAY_CONTROL_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISPLAY_CONTROL_EXTENSION_NAME: &[u8; 23] = b"VK_EXT_display_control\0";

pub const VK_GOOGLE_display_timing: u32 = 1;
pub const VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &[u8; 25] = b"VK_GOOGLE_display_timing\0";

pub const VK_NV_sample_mask_override_coverage: u32 = 1;
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_SPEC_VERSION: u32 = 1;
pub const VK_NV_SAMPLE_MASK_OVERRIDE_COVERAGE_EXTENSION_NAME: &[u8; 36] = b"VK_NV_sample_mask_override_coverage\0";

pub const VK_NV_geometry_shader_passthrough: u32 = 1;
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_SPEC_VERSION: u32 = 1;
pub const VK_NV_GEOMETRY_SHADER_PASSTHROUGH_EXTENSION_NAME: &[u8; 34] = b"VK_NV_geometry_shader_passthrough\0";

pub const VK_NV_viewport_array2: u32 = 1;
pub const VK_NV_VIEWPORT_ARRAY_2_SPEC_VERSION: u32 = 1;
pub const VK_NV_VIEWPORT_ARRAY_2_EXTENSION_NAME: &[u8; 22] = b"VK_NV_viewport_array2\0";
pub const VK_NV_VIEWPORT_ARRAY2_SPEC_VERSION: u32 = 1;
pub const VK_NV_VIEWPORT_ARRAY2_EXTENSION_NAME: &[u8; 22] = b"VK_NV_viewport_array2\0";

pub const VK_NVX_multiview_per_view_attributes: u32 = 1;
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
pub const VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &[u8; 37] = b"VK_NVX_multiview_per_view_attributes\0";

pub const VK_NV_viewport_swizzle: u32 = 1;
pub const VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &[u8; 23] = b"VK_NV_viewport_swizzle\0";

pub const VK_EXT_discard_rectangles: u32 = 1;
pub const VK_EXT_DISCARD_RECTANGLES_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DISCARD_RECTANGLES_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_discard_rectangles\0";

pub const VK_EXT_conservative_rasterization: u32 = 1;
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION: u32 = 1;
pub const VK_EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME: &[u8; 34] = b"VK_EXT_conservative_rasterization\0";

pub const VK_EXT_depth_clip_enable: u32 = 1;
pub const VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &[u8; 25] = b"VK_EXT_depth_clip_enable\0";

pub const VK_EXT_swapchain_colorspace: u32 = 1;
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 4;
pub const VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_swapchain_colorspace\0";

pub const VK_EXT_hdr_metadata: u32 = 1;
pub const VK_EXT_HDR_METADATA_SPEC_VERSION: u32 = 2;
pub const VK_EXT_HDR_METADATA_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_hdr_metadata\0";

pub const VK_EXT_external_memory_dma_buf: u32 = 1;
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_SPEC_VERSION: u32 = 1;
pub const VK_EXT_EXTERNAL_MEMORY_DMA_BUF_EXTENSION_NAME: &[u8; 31] = b"VK_EXT_external_memory_dma_buf\0";

pub const VK_EXT_queue_family_foreign: u32 = 1;
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_SPEC_VERSION: u32 = 1;
pub const VK_EXT_QUEUE_FAMILY_FOREIGN_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_queue_family_foreign\0";

pub const VK_EXT_debug_utils: u32 = 1;
pub const VK_EXT_DEBUG_UTILS_SPEC_VERSION: u32 = 2;
pub const VK_EXT_DEBUG_UTILS_EXTENSION_NAME: &[u8; 19] = b"VK_EXT_debug_utils\0";

pub const VK_EXT_sampler_filter_minmax: u32 = 1;
pub const VK_EXT_SAMPLER_FILTER_MINMAX_SPEC_VERSION: u32 = 2;
pub const VK_EXT_SAMPLER_FILTER_MINMAX_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_sampler_filter_minmax\0";

pub const VK_AMD_gpu_shader_int16: u32 = 1;
pub const VK_AMD_GPU_SHADER_INT16_SPEC_VERSION: u32 = 2;
pub const VK_AMD_GPU_SHADER_INT16_EXTENSION_NAME: &[u8; 24] = b"VK_AMD_gpu_shader_int16\0";

pub const VK_AMD_mixed_attachment_samples: u32 = 1;
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_SPEC_VERSION: u32 = 1;
pub const VK_AMD_MIXED_ATTACHMENT_SAMPLES_EXTENSION_NAME: &[u8; 32] = b"VK_AMD_mixed_attachment_samples\0";

pub const VK_AMD_shader_fragment_mask: u32 = 1;
pub const VK_AMD_SHADER_FRAGMENT_MASK_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_FRAGMENT_MASK_EXTENSION_NAME: &[u8; 28] = b"VK_AMD_shader_fragment_mask\0";

pub const VK_EXT_inline_uniform_block: u32 = 1;
pub const VK_EXT_INLINE_UNIFORM_BLOCK_SPEC_VERSION: u32 = 1;
pub const VK_EXT_INLINE_UNIFORM_BLOCK_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_inline_uniform_block\0";

pub const VK_EXT_shader_stencil_export: u32 = 1;
pub const VK_EXT_SHADER_STENCIL_EXPORT_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_STENCIL_EXPORT_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_shader_stencil_export\0";

pub const VK_EXT_sample_locations: u32 = 1;
pub const VK_EXT_SAMPLE_LOCATIONS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SAMPLE_LOCATIONS_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_sample_locations\0";

pub const VK_EXT_blend_operation_advanced: u32 = 1;
pub const VK_EXT_BLEND_OPERATION_ADVANCED_SPEC_VERSION: u32 = 2;
pub const VK_EXT_BLEND_OPERATION_ADVANCED_EXTENSION_NAME: &[u8; 32] = b"VK_EXT_blend_operation_advanced\0";

pub const VK_NV_fragment_coverage_to_color: u32 = 1;
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAGMENT_COVERAGE_TO_COLOR_EXTENSION_NAME: &[u8; 33] = b"VK_NV_fragment_coverage_to_color\0";

pub const VK_NV_framebuffer_mixed_samples: u32 = 1;
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &[u8; 32] = b"VK_NV_framebuffer_mixed_samples\0";

pub const VK_NV_fill_rectangle: u32 = 1;
pub const VK_NV_FILL_RECTANGLE_SPEC_VERSION: u32 = 1;
pub const VK_NV_FILL_RECTANGLE_EXTENSION_NAME: &[u8; 21] = b"VK_NV_fill_rectangle\0";

pub const VK_NV_shader_sm_builtins: u32 = 1;
pub const VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
pub const VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &[u8; 25] = b"VK_NV_shader_sm_builtins\0";

pub const VK_EXT_post_depth_coverage: u32 = 1;
pub const VK_EXT_POST_DEPTH_COVERAGE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_POST_DEPTH_COVERAGE_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_post_depth_coverage\0";

pub const VK_EXT_image_drm_format_modifier: u32 = 1;
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
pub const VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &[u8; 33] = b"VK_EXT_image_drm_format_modifier\0";

pub const VK_EXT_validation_cache: u32 = 1;
pub const VK_EXT_VALIDATION_CACHE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_VALIDATION_CACHE_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_validation_cache\0";

pub const VK_EXT_descriptor_indexing: u32 = 1;
pub const VK_EXT_DESCRIPTOR_INDEXING_SPEC_VERSION: u32 = 2;
pub const VK_EXT_DESCRIPTOR_INDEXING_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_descriptor_indexing\0";

pub const VK_EXT_shader_viewport_index_layer: u32 = 1;
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_VIEWPORT_INDEX_LAYER_EXTENSION_NAME: &[u8; 35] = b"VK_EXT_shader_viewport_index_layer\0";

pub const VK_NV_shading_rate_image: u32 = 1;
pub const VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
pub const VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &[u8; 25] = b"VK_NV_shading_rate_image\0";

pub const VK_NV_ray_tracing: u32 = 1;
pub const VK_NV_RAY_TRACING_SPEC_VERSION: u32 = 3;
pub const VK_NV_RAY_TRACING_EXTENSION_NAME: &[u8; 18] = b"VK_NV_ray_tracing\0";

pub const VK_NV_representative_fragment_test: u32 = 1;
pub const VK_NV_REPRESENTATIVE_FRAGMENT_TEST_SPEC_VERSION: u32 = 2;
pub const VK_NV_REPRESENTATIVE_FRAGMENT_TEST_EXTENSION_NAME: &[u8; 35] = b"VK_NV_representative_fragment_test\0";

pub const VK_EXT_filter_cubic: u32 = 1;
pub const VK_EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
pub const VK_EXT_FILTER_CUBIC_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_filter_cubic\0";

pub const VK_QCOM_render_pass_shader_resolve: u32 = 1;
pub const VK_QCOM_RENDER_PASS_SHADER_RESOLVE_SPEC_VERSION: u32 = 4;
pub const VK_QCOM_RENDER_PASS_SHADER_RESOLVE_EXTENSION_NAME: &[u8; 35] = b"VK_QCOM_render_pass_shader_resolve\0";

pub const VK_EXT_global_priority: u32 = 1;
pub const VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
pub const VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &[u8; 23] = b"VK_EXT_global_priority\0";

pub const VK_EXT_external_memory_host: u32 = 1;
pub const VK_EXT_EXTERNAL_MEMORY_HOST_SPEC_VERSION: u32 = 1;
pub const VK_EXT_EXTERNAL_MEMORY_HOST_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_external_memory_host\0";

pub const VK_AMD_buffer_marker: u32 = 1;
pub const VK_AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
pub const VK_AMD_BUFFER_MARKER_EXTENSION_NAME: &[u8; 21] = b"VK_AMD_buffer_marker\0";

pub const VK_AMD_pipeline_compiler_control: u32 = 1;
pub const VK_AMD_PIPELINE_COMPILER_CONTROL_SPEC_VERSION: u32 = 1;
pub const VK_AMD_PIPELINE_COMPILER_CONTROL_EXTENSION_NAME: &[u8; 33] = b"VK_AMD_pipeline_compiler_control\0";

pub const VK_EXT_calibrated_timestamps: u32 = 1;
pub const VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
pub const VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_calibrated_timestamps\0";

pub const VK_AMD_shader_core_properties: u32 = 1;
pub const VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION: u32 = 2;
pub const VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME: &[u8; 30] = b"VK_AMD_shader_core_properties\0";

pub const VK_AMD_memory_overallocation_behavior: u32 = 1;
pub const VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
pub const VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &[u8; 38] = b"VK_AMD_memory_overallocation_behavior\0";

pub const VK_EXT_vertex_attribute_divisor: u32 = 1;
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION: u32 = 3;
pub const VK_EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME: &[u8; 32] = b"VK_EXT_vertex_attribute_divisor\0";

pub const VK_EXT_pipeline_creation_feedback: u32 = 1;
pub const VK_EXT_PIPELINE_CREATION_FEEDBACK_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PIPELINE_CREATION_FEEDBACK_EXTENSION_NAME: &[u8; 34] = b"VK_EXT_pipeline_creation_feedback\0";

pub const VK_NV_shader_subgroup_partitioned: u32 = 1;
pub const VK_NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION: u32 = 1;
pub const VK_NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME: &[u8; 34] = b"VK_NV_shader_subgroup_partitioned\0";

pub const VK_NV_compute_shader_derivatives: u32 = 1;
pub const VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
pub const VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &[u8; 33] = b"VK_NV_compute_shader_derivatives\0";

pub const VK_NV_mesh_shader: u32 = 1;
pub const VK_NV_MESH_SHADER_SPEC_VERSION: u32 = 1;
pub const VK_NV_MESH_SHADER_EXTENSION_NAME: &[u8; 18] = b"VK_NV_mesh_shader\0";

pub const VK_NV_fragment_shader_barycentric: u32 = 1;
pub const VK_NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME: &[u8; 34] = b"VK_NV_fragment_shader_barycentric\0";

pub const VK_NV_shader_image_footprint: u32 = 1;
pub const VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
pub const VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: &[u8; 29] = b"VK_NV_shader_image_footprint\0";

pub const VK_NV_scissor_exclusive: u32 = 1;
pub const VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
pub const VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &[u8; 24] = b"VK_NV_scissor_exclusive\0";

pub const VK_NV_device_diagnostic_checkpoints: u32 = 1;
pub const VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
pub const VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &[u8; 36] = b"VK_NV_device_diagnostic_checkpoints\0";

pub const VK_INTEL_shader_integer_functions2: u32 = 1;
pub const VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_SPEC_VERSION: u32 = 1;
pub const VK_INTEL_SHADER_INTEGER_FUNCTIONS_2_EXTENSION_NAME: &[u8; 35] = b"VK_INTEL_shader_integer_functions2\0";

pub const VK_INTEL_performance_query: u32 = 1;
pub const VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
pub const VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &[u8; 27] = b"VK_INTEL_performance_query\0";

pub const VK_EXT_pci_bus_info: u32 = 1;
pub const VK_EXT_PCI_BUS_INFO_SPEC_VERSION: u32 = 2;
pub const VK_EXT_PCI_BUS_INFO_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_pci_bus_info\0";

pub const VK_AMD_display_native_hdr: u32 = 1;
pub const VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION: u32 = 1;
pub const VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME: &[u8; 26] = b"VK_AMD_display_native_hdr\0";

pub const VK_EXT_fragment_density_map: u32 = 1;
pub const VK_EXT_FRAGMENT_DENSITY_MAP_SPEC_VERSION: u32 = 2;
pub const VK_EXT_FRAGMENT_DENSITY_MAP_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_fragment_density_map\0";

pub const VK_EXT_scalar_block_layout: u32 = 1;
pub const VK_EXT_SCALAR_BLOCK_LAYOUT_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SCALAR_BLOCK_LAYOUT_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_scalar_block_layout\0";

pub const VK_GOOGLE_hlsl_functionality1: u32 = 1;
pub const VK_GOOGLE_HLSL_FUNCTIONALITY_1_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_HLSL_FUNCTIONALITY_1_EXTENSION_NAME: &[u8; 30] = b"VK_GOOGLE_hlsl_functionality1\0";
pub const VK_GOOGLE_HLSL_FUNCTIONALITY1_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_HLSL_FUNCTIONALITY1_EXTENSION_NAME: &[u8; 30] = b"VK_GOOGLE_hlsl_functionality1\0";

pub const VK_GOOGLE_decorate_string: u32 = 1;
pub const VK_GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME: &[u8; 26] = b"VK_GOOGLE_decorate_string\0";

pub const VK_EXT_subgroup_size_control: u32 = 1;
pub const VK_EXT_SUBGROUP_SIZE_CONTROL_SPEC_VERSION: u32 = 2;
pub const VK_EXT_SUBGROUP_SIZE_CONTROL_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_subgroup_size_control\0";

pub const VK_AMD_shader_core_properties2: u32 = 1;
pub const VK_AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION: u32 = 1;
pub const VK_AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME: &[u8; 31] = b"VK_AMD_shader_core_properties2\0";

pub const VK_AMD_device_coherent_memory: u32 = 1;
pub const VK_AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME: &[u8; 30] = b"VK_AMD_device_coherent_memory\0";

pub const VK_EXT_shader_image_atomic_int64: u32 = 1;
pub const VK_EXT_SHADER_IMAGE_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_IMAGE_ATOMIC_INT64_EXTENSION_NAME: &[u8; 33] = b"VK_EXT_shader_image_atomic_int64\0";

pub const VK_EXT_memory_budget: u32 = 1;
pub const VK_EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
pub const VK_EXT_MEMORY_BUDGET_EXTENSION_NAME: &[u8; 21] = b"VK_EXT_memory_budget\0";

pub const VK_EXT_memory_priority: u32 = 1;
pub const VK_EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME: &[u8; 23] = b"VK_EXT_memory_priority\0";

pub const VK_NV_dedicated_allocation_image_aliasing: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_SPEC_VERSION: u32 = 1;
pub const VK_NV_DEDICATED_ALLOCATION_IMAGE_ALIASING_EXTENSION_NAME: &[u8; 42] = b"VK_NV_dedicated_allocation_image_aliasing\0";

pub const VK_EXT_buffer_device_address: u32 = 1;
pub const VK_EXT_BUFFER_DEVICE_ADDRESS_SPEC_VERSION: u32 = 2;
pub const VK_EXT_BUFFER_DEVICE_ADDRESS_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_buffer_device_address\0";

pub const VK_EXT_tooling_info: u32 = 1;
pub const VK_EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
pub const VK_EXT_TOOLING_INFO_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_tooling_info\0";

pub const VK_EXT_separate_stencil_usage: u32 = 1;
pub const VK_EXT_SEPARATE_STENCIL_USAGE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SEPARATE_STENCIL_USAGE_EXTENSION_NAME: &[u8; 30] = b"VK_EXT_separate_stencil_usage\0";

pub const VK_EXT_validation_features: u32 = 1;
pub const VK_EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 5;
pub const VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_validation_features\0";

pub const VK_NV_cooperative_matrix: u32 = 1;
pub const VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION: u32 = 1;
pub const VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME: &[u8; 25] = b"VK_NV_cooperative_matrix\0";

pub const VK_NV_coverage_reduction_mode: u32 = 1;
pub const VK_NV_COVERAGE_REDUCTION_MODE_SPEC_VERSION: u32 = 1;
pub const VK_NV_COVERAGE_REDUCTION_MODE_EXTENSION_NAME: &[u8; 30] = b"VK_NV_coverage_reduction_mode\0";

pub const VK_EXT_fragment_shader_interlock: u32 = 1;
pub const VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
pub const VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: &[u8; 33] = b"VK_EXT_fragment_shader_interlock\0";

pub const VK_EXT_ycbcr_image_arrays: u32 = 1;
pub const VK_EXT_YCBCR_IMAGE_ARRAYS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_YCBCR_IMAGE_ARRAYS_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_ycbcr_image_arrays\0";

pub const VK_EXT_provoking_vertex: u32 = 1;
pub const VK_EXT_PROVOKING_VERTEX_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PROVOKING_VERTEX_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_provoking_vertex\0";

pub const VK_EXT_headless_surface: u32 = 1;
pub const VK_EXT_HEADLESS_SURFACE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_headless_surface\0";

pub const VK_EXT_line_rasterization: u32 = 1;
pub const VK_EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
pub const VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_line_rasterization\0";

pub const VK_EXT_shader_atomic_float: u32 = 1;
pub const VK_EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_shader_atomic_float\0";

pub const VK_EXT_host_query_reset: u32 = 1;
pub const VK_EXT_HOST_QUERY_RESET_SPEC_VERSION: u32 = 1;
pub const VK_EXT_HOST_QUERY_RESET_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_host_query_reset\0";

pub const VK_EXT_index_type_uint8: u32 = 1;
pub const VK_EXT_INDEX_TYPE_UINT8_SPEC_VERSION: u32 = 1;
pub const VK_EXT_INDEX_TYPE_UINT8_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_index_type_uint8\0";

pub const VK_EXT_extended_dynamic_state: u32 = 1;
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_EXTENSION_NAME: &[u8; 30] = b"VK_EXT_extended_dynamic_state\0";

pub const VK_EXT_shader_atomic_float2: u32 = 1;
pub const VK_EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_shader_atomic_float2\0";

pub const VK_EXT_shader_demote_to_helper_invocation: u32 = 1;
pub const VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_SPEC_VERSION: u32 = 1;
pub const VK_EXT_SHADER_DEMOTE_TO_HELPER_INVOCATION_EXTENSION_NAME: &[u8; 42] = b"VK_EXT_shader_demote_to_helper_invocation\0";

pub const VK_NV_device_generated_commands: u32 = 1;
pub const VK_NV_DEVICE_GENERATED_COMMANDS_SPEC_VERSION: u32 = 3;
pub const VK_NV_DEVICE_GENERATED_COMMANDS_EXTENSION_NAME: &[u8; 32] = b"VK_NV_device_generated_commands\0";

pub const VK_NV_inherited_viewport_scissor: u32 = 1;
pub const VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
pub const VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: &[u8; 33] = b"VK_NV_inherited_viewport_scissor\0";

pub const VK_EXT_texel_buffer_alignment: u32 = 1;
pub const VK_EXT_TEXEL_BUFFER_ALIGNMENT_SPEC_VERSION: u32 = 1;
pub const VK_EXT_TEXEL_BUFFER_ALIGNMENT_EXTENSION_NAME: &[u8; 30] = b"VK_EXT_texel_buffer_alignment\0";

pub const VK_QCOM_render_pass_transform: u32 = 1;
pub const VK_QCOM_RENDER_PASS_TRANSFORM_SPEC_VERSION: u32 = 2;
pub const VK_QCOM_RENDER_PASS_TRANSFORM_EXTENSION_NAME: &[u8; 30] = b"VK_QCOM_render_pass_transform\0";

pub const VK_EXT_device_memory_report: u32 = 1;
pub const VK_EXT_DEVICE_MEMORY_REPORT_SPEC_VERSION: u32 = 2;
pub const VK_EXT_DEVICE_MEMORY_REPORT_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_device_memory_report\0";

pub const VK_EXT_acquire_drm_display: u32 = 1;
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_acquire_drm_display\0";

pub const VK_EXT_robustness2: u32 = 1;
pub const VK_EXT_ROBUSTNESS_2_SPEC_VERSION: u32 = 1;
pub const VK_EXT_ROBUSTNESS_2_EXTENSION_NAME: &[u8; 19] = b"VK_EXT_robustness2\0";

pub const VK_EXT_custom_border_color: u32 = 1;
pub const VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
pub const VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_custom_border_color\0";

pub const VK_GOOGLE_user_type: u32 = 1;
pub const VK_GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_USER_TYPE_EXTENSION_NAME: &[u8; 20] = b"VK_GOOGLE_user_type\0";

pub const VK_EXT_private_data: u32 = 1;
pub const VK_EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PRIVATE_DATA_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_private_data\0";

pub const VK_EXT_pipeline_creation_cache_control: u32 = 1;
pub const VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_SPEC_VERSION: u32 = 3;
pub const VK_EXT_PIPELINE_CREATION_CACHE_CONTROL_EXTENSION_NAME: &[u8; 39] = b"VK_EXT_pipeline_creation_cache_control\0";

pub const VK_NV_device_diagnostics_config: u32 = 1;
pub const VK_NV_DEVICE_DIAGNOSTICS_CONFIG_SPEC_VERSION: u32 = 1;
pub const VK_NV_DEVICE_DIAGNOSTICS_CONFIG_EXTENSION_NAME: &[u8; 32] = b"VK_NV_device_diagnostics_config\0";

pub const VK_QCOM_render_pass_store_ops: u32 = 1;
pub const VK_QCOM_RENDER_PASS_STORE_OPS_SPEC_VERSION: u32 = 2;
pub const VK_QCOM_RENDER_PASS_STORE_OPS_EXTENSION_NAME: &[u8; 30] = b"VK_QCOM_render_pass_store_ops\0";

pub const VK_EXT_graphics_pipeline_library: u32 = 1;
pub const VK_EXT_GRAPHICS_PIPELINE_LIBRARY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_GRAPHICS_PIPELINE_LIBRARY_EXTENSION_NAME: &[u8; 33] = b"VK_EXT_graphics_pipeline_library\0";

pub const VK_NV_fragment_shading_rate_enums: u32 = 1;
pub const VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
pub const VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &[u8; 34] = b"VK_NV_fragment_shading_rate_enums\0";

pub const VK_NV_ray_tracing_motion_blur: u32 = 1;
pub const VK_NV_RAY_TRACING_MOTION_BLUR_SPEC_VERSION: u32 = 1;
pub const VK_NV_RAY_TRACING_MOTION_BLUR_EXTENSION_NAME: &[u8; 30] = b"VK_NV_ray_tracing_motion_blur\0";

pub const VK_EXT_ycbcr_2plane_444_formats: u32 = 1;
pub const VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: &[u8; 32] = b"VK_EXT_ycbcr_2plane_444_formats\0";

pub const VK_EXT_fragment_density_map2: u32 = 1;
pub const VK_EXT_FRAGMENT_DENSITY_MAP_2_SPEC_VERSION: u32 = 1;
pub const VK_EXT_FRAGMENT_DENSITY_MAP_2_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_fragment_density_map2\0";

pub const VK_QCOM_rotated_copy_commands: u32 = 1;
pub const VK_QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 1;
pub const VK_QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: &[u8; 30] = b"VK_QCOM_rotated_copy_commands\0";

pub const VK_EXT_image_robustness: u32 = 1;
pub const VK_EXT_IMAGE_ROBUSTNESS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_IMAGE_ROBUSTNESS_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_image_robustness\0";

pub const VK_EXT_4444_formats: u32 = 1;
pub const VK_EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_4444_FORMATS_EXTENSION_NAME: &[u8; 20] = b"VK_EXT_4444_formats\0";

pub const VK_ARM_rasterization_order_attachment_access: u32 = 1;
pub const VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_SPEC_VERSION: u32 = 1;
pub const VK_ARM_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXTENSION_NAME: &[u8; 45] = b"VK_ARM_rasterization_order_attachment_access\0";

pub const VK_EXT_rgba10x6_formats: u32 = 1;
pub const VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
pub const VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &[u8; 24] = b"VK_EXT_rgba10x6_formats\0";

pub const VK_NV_acquire_winrt_display: u32 = 1;
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
pub const VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &[u8; 28] = b"VK_NV_acquire_winrt_display\0";

pub const VK_VALVE_mutable_descriptor_type: u32 = 1;
pub const VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_SPEC_VERSION: u32 = 1;
pub const VK_VALVE_MUTABLE_DESCRIPTOR_TYPE_EXTENSION_NAME: &[u8; 33] = b"VK_VALVE_mutable_descriptor_type\0";

pub const VK_EXT_vertex_input_dynamic_state: u32 = 1;
pub const VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_SPEC_VERSION: u32 = 2;
pub const VK_EXT_VERTEX_INPUT_DYNAMIC_STATE_EXTENSION_NAME: &[u8; 34] = b"VK_EXT_vertex_input_dynamic_state\0";

pub const VK_EXT_physical_device_drm: u32 = 1;
pub const VK_EXT_PHYSICAL_DEVICE_DRM_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PHYSICAL_DEVICE_DRM_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_physical_device_drm\0";

pub const VK_EXT_depth_clip_control: u32 = 1;
pub const VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
pub const VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_depth_clip_control\0";

pub const VK_EXT_primitive_topology_list_restart: u32 = 1;
pub const VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME: &[u8; 39] = b"VK_EXT_primitive_topology_list_restart\0";

pub const VK_HUAWEI_subpass_shading: u32 = 1;
pub const VK_HUAWEI_SUBPASS_SHADING_SPEC_VERSION: u32 = 2;
pub const VK_HUAWEI_SUBPASS_SHADING_EXTENSION_NAME: &[u8; 26] = b"VK_HUAWEI_subpass_shading\0";

pub const VK_HUAWEI_invocation_mask: u32 = 1;
pub const VK_HUAWEI_INVOCATION_MASK_SPEC_VERSION: u32 = 1;
pub const VK_HUAWEI_INVOCATION_MASK_EXTENSION_NAME: &[u8; 26] = b"VK_HUAWEI_invocation_mask\0";

pub const VK_NV_external_memory_rdma: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_RDMA_SPEC_VERSION: u32 = 1;
pub const VK_NV_EXTERNAL_MEMORY_RDMA_EXTENSION_NAME: &[u8; 27] = b"VK_NV_external_memory_rdma\0";

pub const VK_EXT_extended_dynamic_state2: u32 = 1;
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_2_SPEC_VERSION: u32 = 1;
pub const VK_EXT_EXTENDED_DYNAMIC_STATE_2_EXTENSION_NAME: &[u8; 31] = b"VK_EXT_extended_dynamic_state2\0";

pub const VK_EXT_color_write_enable: u32 = 1;
pub const VK_EXT_COLOR_WRITE_ENABLE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_COLOR_WRITE_ENABLE_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_color_write_enable\0";

pub const VK_EXT_primitives_generated_query: u32 = 1;
pub const VK_EXT_PRIMITIVES_GENERATED_QUERY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PRIMITIVES_GENERATED_QUERY_EXTENSION_NAME: &[u8; 34] = b"VK_EXT_primitives_generated_query\0";

pub const VK_EXT_global_priority_query: u32 = 1;
pub const VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: &[u8; 29] = b"VK_EXT_global_priority_query\0";

pub const VK_MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = 16;

pub const VK_EXT_image_view_min_lod: u32 = 1;
pub const VK_EXT_IMAGE_VIEW_MIN_LOD_SPEC_VERSION: u32 = 1;
pub const VK_EXT_IMAGE_VIEW_MIN_LOD_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_image_view_min_lod\0";

pub const VK_EXT_multi_draw: u32 = 1;
pub const VK_EXT_MULTI_DRAW_SPEC_VERSION: u32 = 1;
pub const VK_EXT_MULTI_DRAW_EXTENSION_NAME: &[u8; 18] = b"VK_EXT_multi_draw\0";

pub const VK_EXT_image_2d_view_of_3d: u32 = 1;
pub const VK_EXT_IMAGE_2D_VIEW_OF_3D_SPEC_VERSION: u32 = 1;
pub const VK_EXT_IMAGE_2D_VIEW_OF_3D_EXTENSION_NAME: &[u8; 27] = b"VK_EXT_image_2d_view_of_3d\0";

pub const VK_EXT_load_store_op_none: u32 = 1;
pub const VK_EXT_LOAD_STORE_OP_NONE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_LOAD_STORE_OP_NONE_EXTENSION_NAME: &[u8; 26] = b"VK_EXT_load_store_op_none\0";

pub const VK_EXT_border_color_swizzle: u32 = 1;
pub const VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
pub const VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &[u8; 28] = b"VK_EXT_border_color_swizzle\0";

pub const VK_EXT_pageable_device_local_memory: u32 = 1;
pub const VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
pub const VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &[u8; 36] = b"VK_EXT_pageable_device_local_memory\0";

pub const VK_VALVE_descriptor_set_host_mapping: u32 = 1;
pub const VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION: u32 = 1;
pub const VK_VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME: &[u8; 37] = b"VK_VALVE_descriptor_set_host_mapping\0";

pub const VK_QCOM_fragment_density_map_offset: u32 = 1;
pub const VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_SPEC_VERSION: u32 = 1;
pub const VK_QCOM_FRAGMENT_DENSITY_MAP_OFFSET_EXTENSION_NAME: &[u8; 36] = b"VK_QCOM_fragment_density_map_offset\0";

pub const VK_NV_linear_color_attachment: u32 = 1;
pub const VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
pub const VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &[u8; 30] = b"VK_NV_linear_color_attachment\0";

pub const VK_GOOGLE_surfaceless_query: u32 = 1;
pub const VK_GOOGLE_SURFACELESS_QUERY_SPEC_VERSION: u32 = 1;
pub const VK_GOOGLE_SURFACELESS_QUERY_EXTENSION_NAME: &[u8; 28] = b"VK_GOOGLE_surfaceless_query\0";

pub const VK_KHR_acceleration_structure: u32 = 1;
pub const VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
pub const VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &[u8; 30] = b"VK_KHR_acceleration_structure\0";

pub const VK_KHR_ray_tracing_pipeline: u32 = 1;
pub const VK_KHR_RAY_TRACING_PIPELINE_SPEC_VERSION: u32 = 1;
pub const VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME: &[u8; 28] = b"VK_KHR_ray_tracing_pipeline\0";

pub const VK_KHR_ray_query: u32 = 1;
pub const VK_KHR_RAY_QUERY_SPEC_VERSION: u32 = 1;
pub const VK_KHR_RAY_QUERY_EXTENSION_NAME: &[u8; 17] = b"VK_KHR_ray_query\0";

pub type VkBool32 = u32;
pub type VkDeviceAddress = u64;
pub type VkDeviceSize = u64;
pub type VkFlags = u32;
pub type VkSampleMask = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBuffer_T {
	_unused: [u8; 0],
}

pub type VkBuffer = *mut VkBuffer_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImage_T {
	_unused: [u8; 0],
}

pub type VkImage = *mut VkImage_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkInstance_T {
	_unused: [u8; 0],
}

pub type VkInstance = *mut VkInstance_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevice_T {
	_unused: [u8; 0],
}

pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDevice_T {
	_unused: [u8; 0],
}

pub type VkDevice = *mut VkDevice_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueue_T {
	_unused: [u8; 0],
}

pub type VkQueue = *mut VkQueue_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphore_T {
	_unused: [u8; 0],
}

pub type VkSemaphore = *mut VkSemaphore_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBuffer_T {
	_unused: [u8; 0],
}

pub type VkCommandBuffer = *mut VkCommandBuffer_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFence_T {
	_unused: [u8; 0],
}

pub type VkFence = *mut VkFence_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceMemory_T {
	_unused: [u8; 0],
}

pub type VkDeviceMemory = *mut VkDeviceMemory_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkEvent_T {
	_unused: [u8; 0],
}

pub type VkEvent = *mut VkEvent_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueryPool_T {
	_unused: [u8; 0],
}

pub type VkQueryPool = *mut VkQueryPool_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferView_T {
	_unused: [u8; 0],
}

pub type VkBufferView = *mut VkBufferView_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageView_T {
	_unused: [u8; 0],
}

pub type VkImageView = *mut VkImageView_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShaderModule_T {
	_unused: [u8; 0],
}

pub type VkShaderModule = *mut VkShaderModule_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCache_T {
	_unused: [u8; 0],
}

pub type VkPipelineCache = *mut VkPipelineCache_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineLayout_T {
	_unused: [u8; 0],
}

pub type VkPipelineLayout = *mut VkPipelineLayout_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipeline_T {
	_unused: [u8; 0],
}

pub type VkPipeline = *mut VkPipeline_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPass_T {
	_unused: [u8; 0],
}

pub type VkRenderPass = *mut VkRenderPass_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayout_T {
	_unused: [u8; 0],
}

pub type VkDescriptorSetLayout = *mut VkDescriptorSetLayout_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSampler_T {
	_unused: [u8; 0],
}

pub type VkSampler = *mut VkSampler_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSet_T {
	_unused: [u8; 0],
}

pub type VkDescriptorSet = *mut VkDescriptorSet_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorPool_T {
	_unused: [u8; 0],
}

pub type VkDescriptorPool = *mut VkDescriptorPool_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFramebuffer_T {
	_unused: [u8; 0],
}

pub type VkFramebuffer = *mut VkFramebuffer_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandPool_T {
	_unused: [u8; 0],
}

pub type VkCommandPool = *mut VkCommandPool_T;

pub type VkResult = i32;
pub const VK_SUCCESS: VkResult = 0;
pub const VK_NOT_READY: VkResult = 1;
pub const VK_TIMEOUT: VkResult = 2;
pub const VK_EVENT_SET: VkResult = 3;
pub const VK_EVENT_RESET: VkResult = 4;
pub const VK_INCOMPLETE: VkResult = 5;
pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
pub const VK_ERROR_UNKNOWN: VkResult = -13;
pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = -1000069000;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = -1000072003;
pub const VK_ERROR_FRAGMENTATION: VkResult = -1000161000;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = -1000257000;
pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = 1000297000;
pub const VK_ERROR_SURFACE_LOST_KHR: VkResult = -1000000000;
pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR: VkResult = -1000000001;
pub const VK_SUBOPTIMAL_KHR: VkResult = 1000001003;
pub const VK_ERROR_OUT_OF_DATE_KHR: VkResult = -1000001004;
pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR: VkResult = -1000003001;
pub const VK_ERROR_VALIDATION_FAILED_EXT: VkResult = -1000011001;
pub const VK_ERROR_INVALID_SHADER_NV: VkResult = -1000012000;
pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: VkResult = -1000158000;
pub const VK_ERROR_NOT_PERMITTED_KHR: VkResult = -1000174001;
pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: VkResult = -1000255000;
pub const VK_THREAD_IDLE_KHR: VkResult = 1000268000;
pub const VK_THREAD_DONE_KHR: VkResult = 1000268001;
pub const VK_OPERATION_DEFERRED_KHR: VkResult = 1000268002;
pub const VK_OPERATION_NOT_DEFERRED_KHR: VkResult = 1000268003;
pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR: VkResult = -1000069000;
pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR: VkResult = -1000072003;
pub const VK_ERROR_FRAGMENTATION_EXT: VkResult = -1000161000;
pub const VK_ERROR_NOT_PERMITTED_EXT: VkResult = -1000174001;
pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT: VkResult = -1000257000;
pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: VkResult = -1000257000;
pub const VK_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = 1000297000;
pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT: VkResult = 1000297000;
pub const VK_RESULT_MAX_ENUM: VkResult = 2147483647;

pub type VkStructureType = u32;
pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = 6;
pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = 7;
pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = 10;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = 11;
pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = 13;
pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = 14;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = 17;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = 19;
pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = 20;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = 21;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = 22;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = 23;
pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = 24;
pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = 25;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = 26;
pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = 27;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = 28;
pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = 31;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = 36;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = 37;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = 38;
pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = 41;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = 43;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = 44;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = 45;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = 46;
pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = 47;
pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = 48;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType = 1000094000;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = 1000157000;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = 1000157001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType = 1000083000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = 1000127000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = 1000127001;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType = 1000060000;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType = 1000060003;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 1000060004;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType = 1000060005;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType = 1000060006;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060013;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060014;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = 1000070000;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType = 1000070001;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146000;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146001;
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146002;
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146003;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = 1000059000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = 1000059001;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = 1000059002;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059004;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = 1000059005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = 1000059006;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059008;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = 1000117000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = 1000117001;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = 1000117002;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = 1000117003;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = 1000053000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = 1000053001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = 1000053002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType = 1000120000;
pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType = 1000145000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType = 1000145001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType = 1000145002;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType = 1000145003;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = 1000156000;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = 1000156001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = 1000156002;
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = 1000156003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = 1000156004;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000156005;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType = 1000085000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = 1000071000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000071001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = 1000071002;
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = 1000071003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = 1000071004;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType = 1000072000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType = 1000072001;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType = 1000072002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = 1000112000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = 1000112001;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType = 1000113000;
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType = 1000077000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType = 1000076000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType = 1000076001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = 1000168000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = 1000168001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: VkStructureType = 1000063000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: VkStructureType = 49;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: VkStructureType = 50;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: VkStructureType = 51;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: VkStructureType = 52;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType = 1000147000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2: VkStructureType = 1000109000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2: VkStructureType = 1000109001;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2: VkStructureType = 1000109002;
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2: VkStructureType = 1000109003;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2: VkStructureType = 1000109004;
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO: VkStructureType = 1000109005;
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO: VkStructureType = 1000109006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: VkStructureType = 1000177000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES: VkStructureType = 1000196000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: VkStructureType = 1000180000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: VkStructureType = 1000082000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: VkStructureType = 1000197000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: VkStructureType = 1000161000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: VkStructureType = 1000161001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: VkStructureType = 1000161002;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: VkStructureType = 1000161003;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: VkStructureType = 1000161004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: VkStructureType = 1000199000;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: VkStructureType = 1000199001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: VkStructureType = 1000221000;
pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO: VkStructureType = 1000246000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: VkStructureType = 1000130000;
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO: VkStructureType = 1000130001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: VkStructureType = 1000211000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: VkStructureType = 1000108000;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: VkStructureType = 1000108001;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: VkStructureType = 1000108002;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO: VkStructureType = 1000108003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: VkStructureType = 1000253000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: VkStructureType = 1000175000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: VkStructureType = 1000241000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT: VkStructureType = 1000241001;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: VkStructureType = 1000241002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: VkStructureType = 1000261000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType = 1000207000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: VkStructureType = 1000207001;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = 1000207002;
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000207003;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType = 1000207004;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType = 1000207005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: VkStructureType = 1000257000;
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType = 1000244001;
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType = 1000257002;
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType = 1000257003;
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType = 1000257004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: VkStructureType = 53;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: VkStructureType = 54;
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO: VkStructureType = 1000192000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: VkStructureType = 1000215000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES: VkStructureType = 1000245000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: VkStructureType = 1000276000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: VkStructureType = 1000295000;
pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO: VkStructureType = 1000295001;
pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO: VkStructureType = 1000295002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: VkStructureType = 1000297000;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2: VkStructureType = 1000314000;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2: VkStructureType = 1000314001;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2: VkStructureType = 1000314002;
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO: VkStructureType = 1000314003;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2: VkStructureType = 1000314004;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000314005;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO: VkStructureType = 1000314006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: VkStructureType = 1000314007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: VkStructureType = 1000325000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: VkStructureType = 1000335000;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2: VkStructureType = 1000337000;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2: VkStructureType = 1000337001;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2: VkStructureType = 1000337002;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2: VkStructureType = 1000337003;
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2: VkStructureType = 1000337004;
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2: VkStructureType = 1000337005;
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2: VkStructureType = 1000337006;
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2: VkStructureType = 1000337007;
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2: VkStructureType = 1000337008;
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2: VkStructureType = 1000337009;
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2: VkStructureType = 1000337010;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: VkStructureType = 1000225000;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: VkStructureType = 1000225001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: VkStructureType = 1000225002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: VkStructureType = 1000138000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: VkStructureType = 1000138001;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: VkStructureType = 1000138002;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: VkStructureType = 1000138003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: VkStructureType = 1000066000;
pub const VK_STRUCTURE_TYPE_RENDERING_INFO: VkStructureType = 1000044000;
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO: VkStructureType = 1000044001;
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO: VkStructureType = 1000044002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: VkStructureType = 1000044003;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: VkStructureType = 1000044004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: VkStructureType = 1000280000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: VkStructureType = 1000280001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: VkStructureType = 1000281001;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3: VkStructureType = 1000360000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: VkStructureType = 1000413000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: VkStructureType = 1000413001;
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS: VkStructureType = 1000413002;
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS: VkStructureType = 1000413003;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000001000;
pub const VK_STRUCTURE_TYPE_PRESENT_INFO_KHR: VkStructureType = 1000001001;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType = 1000060007;
pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060008;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType = 1000060009;
pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType = 1000060010;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType = 1000060011;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060012;
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR: VkStructureType = 1000002000;
pub const VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000002001;
pub const VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR: VkStructureType = 1000003000;
pub const VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000004000;
pub const VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000005000;
pub const VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000006000;
pub const VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000008000;
pub const VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR: VkStructureType = 1000009000;
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: VkStructureType = 1000011000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: VkStructureType = 1000018000;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT: VkStructureType = 1000022000;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_TAG_INFO_EXT: VkStructureType = 1000022001;
pub const VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT: VkStructureType = 1000022002;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: VkStructureType = 1000026000;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: VkStructureType = 1000026001;
pub const VK_STRUCTURE_TYPE_DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: VkStructureType = 1000026002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: VkStructureType = 1000028000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: VkStructureType = 1000028001;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: VkStructureType = 1000028002;
pub const VK_STRUCTURE_TYPE_CU_MODULE_CREATE_INFO_NVX: VkStructureType = 1000029000;
pub const VK_STRUCTURE_TYPE_CU_FUNCTION_CREATE_INFO_NVX: VkStructureType = 1000029001;
pub const VK_STRUCTURE_TYPE_CU_LAUNCH_INFO_NVX: VkStructureType = 1000029002;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_HANDLE_INFO_NVX: VkStructureType = 1000030000;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: VkStructureType = 1000030001;
pub const VK_STRUCTURE_TYPE_TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: VkStructureType = 1000041000;
pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: VkStructureType = 1000044006;
pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: VkStructureType = 1000044007;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD: VkStructureType = 1000044008;
pub const VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: VkStructureType = 1000044009;
pub const VK_STRUCTURE_TYPE_STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: VkStructureType = 1000049000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: VkStructureType = 1000050000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: VkStructureType = 1000056000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_NV: VkStructureType = 1000056001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 1000057000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: VkStructureType = 1000057001;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: VkStructureType = 1000058000;
pub const VK_STRUCTURE_TYPE_VALIDATION_FLAGS_EXT: VkStructureType = 1000061000;
pub const VK_STRUCTURE_TYPE_VI_SURFACE_CREATE_INFO_NN: VkStructureType = 1000062000;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_ASTC_DECODE_MODE_EXT: VkStructureType = 1000067000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: VkStructureType = 1000067001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073000;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073001;
pub const VK_STRUCTURE_TYPE_MEMORY_WIN32_HANDLE_PROPERTIES_KHR: VkStructureType = 1000073002;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000073003;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_FD_INFO_KHR: VkStructureType = 1000074000;
pub const VK_STRUCTURE_TYPE_MEMORY_FD_PROPERTIES_KHR: VkStructureType = 1000074001;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_FD_INFO_KHR: VkStructureType = 1000074002;
pub const VK_STRUCTURE_TYPE_WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: VkStructureType = 1000075000;
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078000;
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078001;
pub const VK_STRUCTURE_TYPE_D3D12_FENCE_SUBMIT_INFO_KHR: VkStructureType = 1000078002;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000078003;
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_FD_INFO_KHR: VkStructureType = 1000079000;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_FD_INFO_KHR: VkStructureType = 1000079001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: VkStructureType = 1000080000;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: VkStructureType = 1000081000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: VkStructureType = 1000081001;
pub const VK_STRUCTURE_TYPE_CONDITIONAL_RENDERING_BEGIN_INFO_EXT: VkStructureType = 1000081002;
pub const VK_STRUCTURE_TYPE_PRESENT_REGIONS_KHR: VkStructureType = 1000084000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: VkStructureType = 1000087000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT: VkStructureType = 1000090000;
pub const VK_STRUCTURE_TYPE_DISPLAY_POWER_INFO_EXT: VkStructureType = 1000091000;
pub const VK_STRUCTURE_TYPE_DEVICE_EVENT_INFO_EXT: VkStructureType = 1000091001;
pub const VK_STRUCTURE_TYPE_DISPLAY_EVENT_INFO_EXT: VkStructureType = 1000091002;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_COUNTER_CREATE_INFO_EXT: VkStructureType = 1000091003;
pub const VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE: VkStructureType = 1000092000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: VkStructureType = 1000097000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: VkStructureType = 1000098000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: VkStructureType = 1000099000;
pub const VK_STRUCTURE_TYPE_PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: VkStructureType = 1000099001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: VkStructureType = 1000101000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: VkStructureType = 1000101001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: VkStructureType = 1000102000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: VkStructureType = 1000102001;
pub const VK_STRUCTURE_TYPE_HDR_METADATA_EXT: VkStructureType = 1000105000;
pub const VK_STRUCTURE_TYPE_SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: VkStructureType = 1000111000;
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114000;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114001;
pub const VK_STRUCTURE_TYPE_FENCE_GET_WIN32_HANDLE_INFO_KHR: VkStructureType = 1000114002;
pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_FD_INFO_KHR: VkStructureType = 1000115000;
pub const VK_STRUCTURE_TYPE_FENCE_GET_FD_INFO_KHR: VkStructureType = 1000115001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: VkStructureType = 1000116000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: VkStructureType = 1000116001;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: VkStructureType = 1000116002;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR: VkStructureType = 1000116003;
pub const VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR: VkStructureType = 1000116004;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR: VkStructureType = 1000116005;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR: VkStructureType = 1000116006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: VkStructureType = 1000119000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_KHR: VkStructureType = 1000119001;
pub const VK_STRUCTURE_TYPE_SURFACE_FORMAT_2_KHR: VkStructureType = 1000119002;
pub const VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR: VkStructureType = 1000121000;
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR: VkStructureType = 1000121001;
pub const VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR: VkStructureType = 1000121002;
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR: VkStructureType = 1000121003;
pub const VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR: VkStructureType = 1000121004;
pub const VK_STRUCTURE_TYPE_IOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 1000122000;
pub const VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK: VkStructureType = 1000123000;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT: VkStructureType = 1000128000;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT: VkStructureType = 1000128001;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT: VkStructureType = 1000128002;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: VkStructureType = 1000128003;
pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: VkStructureType = 1000128004;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: VkStructureType = 1000129000;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: VkStructureType = 1000129001;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: VkStructureType = 1000129002;
pub const VK_STRUCTURE_TYPE_IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = 1000129003;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: VkStructureType = 1000129004;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FORMAT_ANDROID: VkStructureType = 1000129005;
pub const VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: VkStructureType = 1000129006;
pub const VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT: VkStructureType = 1000143000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: VkStructureType = 1000143001;
pub const VK_STRUCTURE_TYPE_PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: VkStructureType = 1000143002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: VkStructureType = 1000143003;
pub const VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT: VkStructureType = 1000143004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: VkStructureType = 1000148000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: VkStructureType = 1000148001;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: VkStructureType = 1000148002;
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: VkStructureType = 1000149000;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: VkStructureType = 1000150007;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: VkStructureType = 1000150000;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: VkStructureType = 1000150002;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: VkStructureType = 1000150003;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: VkStructureType = 1000150004;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: VkStructureType = 1000150005;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR: VkStructureType = 1000150006;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR: VkStructureType = 1000150009;
pub const VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR: VkStructureType = 1000150010;
pub const VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: VkStructureType = 1000150011;
pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: VkStructureType = 1000150012;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: VkStructureType = 1000150013;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: VkStructureType = 1000150014;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR: VkStructureType = 1000150017;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: VkStructureType = 1000150020;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: VkStructureType = 1000347000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: VkStructureType = 1000347001;
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR: VkStructureType = 1000150015;
pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: VkStructureType = 1000150016;
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: VkStructureType = 1000150018;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: VkStructureType = 1000348013;
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: VkStructureType = 1000152000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: VkStructureType = 1000154000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: VkStructureType = 1000154001;
pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: VkStructureType = 1000158000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: VkStructureType = 1000158002;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: VkStructureType = 1000158003;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: VkStructureType = 1000158004;
pub const VK_STRUCTURE_TYPE_IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: VkStructureType = 1000158005;
pub const VK_STRUCTURE_TYPE_DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: VkStructureType = 1000158006;
pub const VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 1000160000;
pub const VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: VkStructureType = 1000160001;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: VkStructureType = 1000164000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: VkStructureType = 1000164001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: VkStructureType = 1000164002;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: VkStructureType = 1000164005;
pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_NV: VkStructureType = 1000165000;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV: VkStructureType = 1000165001;
pub const VK_STRUCTURE_TYPE_GEOMETRY_NV: VkStructureType = 1000165003;
pub const VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV: VkStructureType = 1000165004;
pub const VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV: VkStructureType = 1000165005;
pub const VK_STRUCTURE_TYPE_BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: VkStructureType = 1000165006;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: VkStructureType = 1000165007;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: VkStructureType = 1000165008;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: VkStructureType = 1000165009;
pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: VkStructureType = 1000165011;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV: VkStructureType = 1000165012;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: VkStructureType = 1000166000;
pub const VK_STRUCTURE_TYPE_PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: VkStructureType = 1000166001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: VkStructureType = 1000170000;
pub const VK_STRUCTURE_TYPE_FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: VkStructureType = 1000170001;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_HOST_POINTER_INFO_EXT: VkStructureType = 1000178000;
pub const VK_STRUCTURE_TYPE_MEMORY_HOST_POINTER_PROPERTIES_EXT: VkStructureType = 1000178001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: VkStructureType = 1000178002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: VkStructureType = 1000181000;
pub const VK_STRUCTURE_TYPE_PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: VkStructureType = 1000183000;
pub const VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT: VkStructureType = 1000184000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: VkStructureType = 1000185000;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: VkStructureType = 1000174000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: VkStructureType = 1000388000;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: VkStructureType = 1000388001;
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: VkStructureType = 1000189000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: VkStructureType = 1000190000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: VkStructureType = 1000190001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: VkStructureType = 1000190002;
pub const VK_STRUCTURE_TYPE_PRESENT_FRAME_TOKEN_GGP: VkStructureType = 1000191000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: VkStructureType = 1000201000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: VkStructureType = 1000202000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: VkStructureType = 1000202001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: VkStructureType = 1000203000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: VkStructureType = 1000204000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: VkStructureType = 1000205000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: VkStructureType = 1000205002;
pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_NV: VkStructureType = 1000206000;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: VkStructureType = 1000206001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: VkStructureType = 1000209000;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: VkStructureType = 1000210000;
pub const VK_STRUCTURE_TYPE_INITIALIZE_PERFORMANCE_API_INFO_INTEL: VkStructureType = 1000210001;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_MARKER_INFO_INTEL: VkStructureType = 1000210002;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_STREAM_MARKER_INFO_INTEL: VkStructureType = 1000210003;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL: VkStructureType = 1000210004;
pub const VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: VkStructureType = 1000210005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: VkStructureType = 1000212000;
pub const VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: VkStructureType = 1000213000;
pub const VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: VkStructureType = 1000213001;
pub const VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: VkStructureType = 1000214000;
pub const VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000217000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: VkStructureType = 1000218000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: VkStructureType = 1000218001;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: VkStructureType = 1000218002;
pub const VK_STRUCTURE_TYPE_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: VkStructureType = 1000226000;
pub const VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: VkStructureType = 1000226001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: VkStructureType = 1000226002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: VkStructureType = 1000226003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: VkStructureType = 1000226004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: VkStructureType = 1000227000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: VkStructureType = 1000229000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: VkStructureType = 1000234000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: VkStructureType = 1000237000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: VkStructureType = 1000238000;
pub const VK_STRUCTURE_TYPE_MEMORY_PRIORITY_ALLOCATE_INFO_EXT: VkStructureType = 1000238001;
pub const VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR: VkStructureType = 1000239000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: VkStructureType = 1000240000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: VkStructureType = 1000244000;
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: VkStructureType = 1000244002;
pub const VK_STRUCTURE_TYPE_VALIDATION_FEATURES_EXT: VkStructureType = 1000247000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: VkStructureType = 1000248000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: VkStructureType = 1000249000;
pub const VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV: VkStructureType = 1000249001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: VkStructureType = 1000249002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: VkStructureType = 1000250000;
pub const VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: VkStructureType = 1000250001;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: VkStructureType = 1000250002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: VkStructureType = 1000251000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: VkStructureType = 1000252000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: VkStructureType = 1000254000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: VkStructureType = 1000254001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: VkStructureType = 1000254002;
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: VkStructureType = 1000255000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: VkStructureType = 1000255002;
pub const VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: VkStructureType = 1000255001;
pub const VK_STRUCTURE_TYPE_HEADLESS_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000256000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: VkStructureType = 1000259000;
pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: VkStructureType = 1000259001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: VkStructureType = 1000259002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: VkStructureType = 1000260000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: VkStructureType = 1000265000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: VkStructureType = 1000267000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: VkStructureType = 1000269000;
pub const VK_STRUCTURE_TYPE_PIPELINE_INFO_KHR: VkStructureType = 1000269001;
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_PROPERTIES_KHR: VkStructureType = 1000269002;
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INFO_KHR: VkStructureType = 1000269003;
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_STATISTIC_KHR: VkStructureType = 1000269004;
pub const VK_STRUCTURE_TYPE_PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: VkStructureType = 1000269005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: VkStructureType = 1000273000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: VkStructureType = 1000277000;
pub const VK_STRUCTURE_TYPE_GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: VkStructureType = 1000277001;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: VkStructureType = 1000277002;
pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: VkStructureType = 1000277003;
pub const VK_STRUCTURE_TYPE_INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: VkStructureType = 1000277004;
pub const VK_STRUCTURE_TYPE_GENERATED_COMMANDS_INFO_NV: VkStructureType = 1000277005;
pub const VK_STRUCTURE_TYPE_GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: VkStructureType = 1000277006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: VkStructureType = 1000277007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: VkStructureType = 1000278000;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: VkStructureType = 1000278001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: VkStructureType = 1000281000;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: VkStructureType = 1000282000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: VkStructureType = 1000282001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: VkStructureType = 1000284000;
pub const VK_STRUCTURE_TYPE_DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: VkStructureType = 1000284001;
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: VkStructureType = 1000284002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: VkStructureType = 1000286000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: VkStructureType = 1000286001;
pub const VK_STRUCTURE_TYPE_SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: VkStructureType = 1000287000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: VkStructureType = 1000287001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: VkStructureType = 1000287002;
pub const VK_STRUCTURE_TYPE_PIPELINE_LIBRARY_CREATE_INFO_KHR: VkStructureType = 1000290000;
pub const VK_STRUCTURE_TYPE_PRESENT_ID_KHR: VkStructureType = 1000294000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: VkStructureType = 1000294001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: VkStructureType = 1000300000;
pub const VK_STRUCTURE_TYPE_DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: VkStructureType = 1000300001;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: VkStructureType = 1000314008;
pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV: VkStructureType = 1000314009;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: VkStructureType = 1000320000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: VkStructureType = 1000320001;
pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: VkStructureType = 1000320002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: VkStructureType = 1000323000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: VkStructureType = 1000326000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: VkStructureType = 1000326001;
pub const VK_STRUCTURE_TYPE_PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: VkStructureType = 1000326002;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: VkStructureType = 1000327000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: VkStructureType = 1000327001;
pub const VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_MOTION_INFO_NV: VkStructureType = 1000327002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: VkStructureType = 1000330000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: VkStructureType = 1000332000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: VkStructureType = 1000332001;
pub const VK_STRUCTURE_TYPE_COPY_COMMAND_TRANSFORM_INFO_QCOM: VkStructureType = 1000333000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: VkStructureType = 1000336000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: VkStructureType = 1000340000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: VkStructureType = 1000342000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: VkStructureType = 1000344000;
pub const VK_STRUCTURE_TYPE_DIRECTFB_SURFACE_CREATE_INFO_EXT: VkStructureType = 1000346000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: VkStructureType = 1000351000;
pub const VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: VkStructureType = 1000351002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: VkStructureType = 1000352000;
pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: VkStructureType = 1000352001;
pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: VkStructureType = 1000352002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: VkStructureType = 1000353000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: VkStructureType = 1000355000;
pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: VkStructureType = 1000355001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: VkStructureType = 1000356000;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000364000;
pub const VK_STRUCTURE_TYPE_MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: VkStructureType = 1000364001;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000364002;
pub const VK_STRUCTURE_TYPE_IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000365000;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: VkStructureType = 1000365001;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: VkStructureType = 1000366000;
pub const VK_STRUCTURE_TYPE_IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: VkStructureType = 1000366001;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: VkStructureType = 1000366002;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA: VkStructureType = 1000366003;
pub const VK_STRUCTURE_TYPE_BUFFER_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366004;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: VkStructureType = 1000366005;
pub const VK_STRUCTURE_TYPE_IMAGE_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366006;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366007;
pub const VK_STRUCTURE_TYPE_SYSMEM_COLOR_SPACE_FUCHSIA: VkStructureType = 1000366008;
pub const VK_STRUCTURE_TYPE_BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: VkStructureType = 1000366009;
pub const VK_STRUCTURE_TYPE_SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: VkStructureType = 1000369000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: VkStructureType = 1000369001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: VkStructureType = 1000369002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: VkStructureType = 1000370000;
pub const VK_STRUCTURE_TYPE_MEMORY_GET_REMOTE_ADDRESS_INFO_NV: VkStructureType = 1000371000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: VkStructureType = 1000371001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: VkStructureType = 1000377000;
pub const VK_STRUCTURE_TYPE_SCREEN_SURFACE_CREATE_INFO_QNX: VkStructureType = 1000378000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: VkStructureType = 1000381000;
pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: VkStructureType = 1000381001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: VkStructureType = 1000382000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: VkStructureType = 1000391000;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: VkStructureType = 1000391001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: VkStructureType = 1000392000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: VkStructureType = 1000392001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: VkStructureType = 1000393000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: VkStructureType = 1000411000;
pub const VK_STRUCTURE_TYPE_SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: VkStructureType = 1000411001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: VkStructureType = 1000412000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: VkStructureType = 1000420000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: VkStructureType = 1000420001;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: VkStructureType = 1000420002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: VkStructureType = 1000425000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: VkStructureType = 1000425001;
pub const VK_STRUCTURE_TYPE_SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: VkStructureType = 1000425002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: VkStructureType = 1000430000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: VkStructureType = 1000120000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: VkStructureType = 1000063000;
pub const VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT: VkStructureType = 1000011000;
pub const VK_STRUCTURE_TYPE_RENDERING_INFO_KHR: VkStructureType = 1000044000;
pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO_KHR: VkStructureType = 1000044001;
pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO_KHR: VkStructureType = 1000044002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: VkStructureType = 1000044003;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: VkStructureType = 1000044004;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_NV: VkStructureType = 1000044008;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: VkStructureType = 1000053000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: VkStructureType = 1000053001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: VkStructureType = 1000053002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2_KHR: VkStructureType = 1000059000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2_KHR: VkStructureType = 1000059001;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059002;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = 1000059004;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2_KHR: VkStructureType = 1000059005;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: VkStructureType = 1000059006;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: VkStructureType = 1000059007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: VkStructureType = 1000059008;
pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO_KHR: VkStructureType = 1000060000;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: VkStructureType = 1000060003;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: VkStructureType = 1000060004;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO_KHR: VkStructureType = 1000060005;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO_KHR: VkStructureType = 1000060006;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = 1000060013;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: VkStructureType = 1000060014;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: VkStructureType = 1000066000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: VkStructureType = 1000070000;
pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: VkStructureType = 1000070001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: VkStructureType = 1000071000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = 1000071001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: VkStructureType = 1000071002;
pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES_KHR: VkStructureType = 1000071003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES_KHR: VkStructureType = 1000071004;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: VkStructureType = 1000072000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: VkStructureType = 1000072001;
pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO_KHR: VkStructureType = 1000072002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: VkStructureType = 1000076000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES_KHR: VkStructureType = 1000076001;
pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO_KHR: VkStructureType = 1000077000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: VkStructureType = 1000082000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: VkStructureType = 1000082000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: VkStructureType = 1000083000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: VkStructureType = 1000085000;
pub const VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT: VkStructureType = 1000090000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: VkStructureType = 1000108000;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: VkStructureType = 1000108001;
pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: VkStructureType = 1000108002;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: VkStructureType = 1000108003;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2_KHR: VkStructureType = 1000109000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2_KHR: VkStructureType = 1000109001;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2_KHR: VkStructureType = 1000109002;
pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2_KHR: VkStructureType = 1000109003;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2_KHR: VkStructureType = 1000109004;
pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO_KHR: VkStructureType = 1000109005;
pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO_KHR: VkStructureType = 1000109006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: VkStructureType = 1000112000;
pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES_KHR: VkStructureType = 1000112001;
pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO_KHR: VkStructureType = 1000113000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: VkStructureType = 1000117000;
pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: VkStructureType = 1000117001;
pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO_KHR: VkStructureType = 1000117002;
pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: VkStructureType = 1000117003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: VkStructureType = 1000120000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: VkStructureType = 1000120000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS_KHR: VkStructureType = 1000127000;
pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO_KHR: VkStructureType = 1000127001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: VkStructureType = 1000130000;
pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: VkStructureType = 1000130001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: VkStructureType = 1000138000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: VkStructureType = 1000138001;
pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: VkStructureType = 1000138002;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: VkStructureType = 1000138003;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146000;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146001;
pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: VkStructureType = 1000146002;
pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = 1000146003;
pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: VkStructureType = 1000146004;
pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO_KHR: VkStructureType = 1000147000;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: VkStructureType = 1000156000;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO_KHR: VkStructureType = 1000156001;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO_KHR: VkStructureType = 1000156002;
pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: VkStructureType = 1000156003;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: VkStructureType = 1000156004;
pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: VkStructureType = 1000156005;
pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO_KHR: VkStructureType = 1000157000;
pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO_KHR: VkStructureType = 1000157001;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: VkStructureType = 1000161000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: VkStructureType = 1000161001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: VkStructureType = 1000161002;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: VkStructureType = 1000161003;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: VkStructureType = 1000161004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: VkStructureType = 1000168000;
pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: VkStructureType = 1000168001;
pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: VkStructureType = 1000174000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: VkStructureType = 1000175000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: VkStructureType = 1000177000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: VkStructureType = 1000180000;
pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: VkStructureType = 1000192000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: VkStructureType = 1000196000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: VkStructureType = 1000197000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: VkStructureType = 1000199000;
pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: VkStructureType = 1000199001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: VkStructureType = 1000207000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: VkStructureType = 1000207001;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO_KHR: VkStructureType = 1000207002;
pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = 1000207003;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO_KHR: VkStructureType = 1000207004;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO_KHR: VkStructureType = 1000207005;
pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO_INTEL: VkStructureType = 1000210000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: VkStructureType = 1000211000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: VkStructureType = 1000215000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: VkStructureType = 1000221000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: VkStructureType = 1000225000;
pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: VkStructureType = 1000225001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: VkStructureType = 1000225002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: VkStructureType = 1000241000;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: VkStructureType = 1000241001;
pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: VkStructureType = 1000241002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: VkStructureType = 1000244000;
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_EXT: VkStructureType = 1000244001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: VkStructureType = 1000245000;
pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: VkStructureType = 1000246000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: VkStructureType = 1000253000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: VkStructureType = 1000257000;
pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO_KHR: VkStructureType = 1000244001;
pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: VkStructureType = 1000257002;
pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: VkStructureType = 1000257003;
pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: VkStructureType = 1000257004;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: VkStructureType = 1000261000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: VkStructureType = 1000276000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: VkStructureType = 1000280000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: VkStructureType = 1000280001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: VkStructureType = 1000281001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: VkStructureType = 1000295000;
pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: VkStructureType = 1000295001;
pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO_EXT: VkStructureType = 1000295002;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: VkStructureType = 1000297000;
pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314000;
pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314001;
pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR: VkStructureType = 1000314002;
pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR: VkStructureType = 1000314003;
pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR: VkStructureType = 1000314004;
pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR: VkStructureType = 1000314005;
pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR: VkStructureType = 1000314006;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: VkStructureType = 1000314007;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: VkStructureType = 1000325000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: VkStructureType = 1000335000;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2_KHR: VkStructureType = 1000337000;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2_KHR: VkStructureType = 1000337001;
pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2_KHR: VkStructureType = 1000337002;
pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2_KHR: VkStructureType = 1000337003;
pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2_KHR: VkStructureType = 1000337004;
pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2_KHR: VkStructureType = 1000337005;
pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2_KHR: VkStructureType = 1000337006;
pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2_KHR: VkStructureType = 1000337007;
pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2_KHR: VkStructureType = 1000337008;
pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2_KHR: VkStructureType = 1000337009;
pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2_KHR: VkStructureType = 1000337010;
pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3_KHR: VkStructureType = 1000360000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: VkStructureType = 1000388000;
pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: VkStructureType = 1000388001;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: VkStructureType = 1000413000;
pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: VkStructureType = 1000413001;
pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: VkStructureType = 1000413002;
pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: VkStructureType = 1000413003;
pub const VK_STRUCTURE_TYPE_MAX_ENUM: VkStructureType = 2147483647;

pub type VkImageLayout = u32;
pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = 0;
pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;
pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = 2;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 3;
pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 4;
pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = 5;
pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = 6;
pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = 7;
pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = 8;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000117000;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000117001;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241000;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL: VkImageLayout = 1000241001;
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241002;
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000241003;
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL: VkImageLayout = 1000314000;
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL: VkImageLayout = 1000314001;
pub const VK_IMAGE_LAYOUT_PRESENT_SRC_KHR: VkImageLayout = 1000001002;
pub const VK_IMAGE_LAYOUT_SHARED_PRESENT_KHR: VkImageLayout = 1000111000;
pub const VK_IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: VkImageLayout = 1000218000;
pub const VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000164003;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000117000;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = 1000117001;
pub const VK_IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV: VkImageLayout = 1000164003;
pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000241000;
pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR: VkImageLayout = 1000241001;
pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000241002;
pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR: VkImageLayout = 1000241003;
pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR: VkImageLayout = 1000314000;
pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000314001;
pub const VK_IMAGE_LAYOUT_MAX_ENUM: VkImageLayout = 2147483647;

pub type VkObjectType = u32;
pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = 0;
pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = 1;
pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = 2;
pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = 3;
pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = 4;
pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = 5;
pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = 6;
pub const VK_OBJECT_TYPE_FENCE: VkObjectType = 7;
pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = 8;
pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = 9;
pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = 10;
pub const VK_OBJECT_TYPE_EVENT: VkObjectType = 11;
pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = 12;
pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = 13;
pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = 14;
pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = 15;
pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = 16;
pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = 17;
pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = 18;
pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = 19;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = 20;
pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = 21;
pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = 22;
pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = 23;
pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = 24;
pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = 25;
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkObjectType = 1000156000;
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkObjectType = 1000085000;
pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT: VkObjectType = 1000295000;
pub const VK_OBJECT_TYPE_SURFACE_KHR: VkObjectType = 1000000000;
pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR: VkObjectType = 1000001000;
pub const VK_OBJECT_TYPE_DISPLAY_KHR: VkObjectType = 1000002000;
pub const VK_OBJECT_TYPE_DISPLAY_MODE_KHR: VkObjectType = 1000002001;
pub const VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT: VkObjectType = 1000011000;
pub const VK_OBJECT_TYPE_CU_MODULE_NVX: VkObjectType = 1000029000;
pub const VK_OBJECT_TYPE_CU_FUNCTION_NVX: VkObjectType = 1000029001;
pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT: VkObjectType = 1000128000;
pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR: VkObjectType = 1000150000;
pub const VK_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkObjectType = 1000160000;
pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV: VkObjectType = 1000165000;
pub const VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL: VkObjectType = 1000210000;
pub const VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR: VkObjectType = 1000268000;
pub const VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV: VkObjectType = 1000277000;
pub const VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA: VkObjectType = 1000366000;
pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR: VkObjectType = 1000085000;
pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR: VkObjectType = 1000156000;
pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT: VkObjectType = 1000295000;
pub const VK_OBJECT_TYPE_MAX_ENUM: VkObjectType = 2147483647;

pub type VkPipelineCacheHeaderVersion = u32;
pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion = 1;
pub const VK_PIPELINE_CACHE_HEADER_VERSION_MAX_ENUM: VkPipelineCacheHeaderVersion = 2147483647;

pub type VkVendorId = u32;
pub const VK_VENDOR_ID_VIV: VkVendorId = 65537;
pub const VK_VENDOR_ID_VSI: VkVendorId = 65538;
pub const VK_VENDOR_ID_KAZAN: VkVendorId = 65539;
pub const VK_VENDOR_ID_CODEPLAY: VkVendorId = 65540;
pub const VK_VENDOR_ID_MESA: VkVendorId = 65541;
pub const VK_VENDOR_ID_POCL: VkVendorId = 65542;
pub const VK_VENDOR_ID_MAX_ENUM: VkVendorId = 2147483647;

pub type VkSystemAllocationScope = u32;
pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;
pub const VK_SYSTEM_ALLOCATION_SCOPE_MAX_ENUM: VkSystemAllocationScope = 2147483647;

pub type VkInternalAllocationType = u32;
pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;
pub const VK_INTERNAL_ALLOCATION_TYPE_MAX_ENUM: VkInternalAllocationType = 2147483647;

pub type VkFormat = u32;
pub const VK_FORMAT_UNDEFINED: VkFormat = 0;
pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = 1;
pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = 2;
pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = 3;
pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = 4;
pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = 5;
pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = 6;
pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = 7;
pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = 8;
pub const VK_FORMAT_R8_UNORM: VkFormat = 9;
pub const VK_FORMAT_R8_SNORM: VkFormat = 10;
pub const VK_FORMAT_R8_USCALED: VkFormat = 11;
pub const VK_FORMAT_R8_SSCALED: VkFormat = 12;
pub const VK_FORMAT_R8_UINT: VkFormat = 13;
pub const VK_FORMAT_R8_SINT: VkFormat = 14;
pub const VK_FORMAT_R8_SRGB: VkFormat = 15;
pub const VK_FORMAT_R8G8_UNORM: VkFormat = 16;
pub const VK_FORMAT_R8G8_SNORM: VkFormat = 17;
pub const VK_FORMAT_R8G8_USCALED: VkFormat = 18;
pub const VK_FORMAT_R8G8_SSCALED: VkFormat = 19;
pub const VK_FORMAT_R8G8_UINT: VkFormat = 20;
pub const VK_FORMAT_R8G8_SINT: VkFormat = 21;
pub const VK_FORMAT_R8G8_SRGB: VkFormat = 22;
pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = 23;
pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = 24;
pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = 25;
pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = 26;
pub const VK_FORMAT_R8G8B8_UINT: VkFormat = 27;
pub const VK_FORMAT_R8G8B8_SINT: VkFormat = 28;
pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = 29;
pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = 30;
pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = 31;
pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = 32;
pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = 33;
pub const VK_FORMAT_B8G8R8_UINT: VkFormat = 34;
pub const VK_FORMAT_B8G8R8_SINT: VkFormat = 35;
pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = 36;
pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = 37;
pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = 38;
pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = 39;
pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = 40;
pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = 41;
pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = 42;
pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = 43;
pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = 44;
pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = 45;
pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = 46;
pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = 47;
pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = 48;
pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = 49;
pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = 50;
pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = 51;
pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = 52;
pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = 53;
pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = 54;
pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = 55;
pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = 56;
pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = 57;
pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = 58;
pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = 59;
pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = 60;
pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = 61;
pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = 62;
pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = 63;
pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = 64;
pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = 65;
pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = 66;
pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = 67;
pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = 68;
pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = 69;
pub const VK_FORMAT_R16_UNORM: VkFormat = 70;
pub const VK_FORMAT_R16_SNORM: VkFormat = 71;
pub const VK_FORMAT_R16_USCALED: VkFormat = 72;
pub const VK_FORMAT_R16_SSCALED: VkFormat = 73;
pub const VK_FORMAT_R16_UINT: VkFormat = 74;
pub const VK_FORMAT_R16_SINT: VkFormat = 75;
pub const VK_FORMAT_R16_SFLOAT: VkFormat = 76;
pub const VK_FORMAT_R16G16_UNORM: VkFormat = 77;
pub const VK_FORMAT_R16G16_SNORM: VkFormat = 78;
pub const VK_FORMAT_R16G16_USCALED: VkFormat = 79;
pub const VK_FORMAT_R16G16_SSCALED: VkFormat = 80;
pub const VK_FORMAT_R16G16_UINT: VkFormat = 81;
pub const VK_FORMAT_R16G16_SINT: VkFormat = 82;
pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = 83;
pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = 84;
pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = 85;
pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = 86;
pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = 87;
pub const VK_FORMAT_R16G16B16_UINT: VkFormat = 88;
pub const VK_FORMAT_R16G16B16_SINT: VkFormat = 89;
pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = 90;
pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = 91;
pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = 92;
pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = 93;
pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = 94;
pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = 95;
pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = 96;
pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = 97;
pub const VK_FORMAT_R32_UINT: VkFormat = 98;
pub const VK_FORMAT_R32_SINT: VkFormat = 99;
pub const VK_FORMAT_R32_SFLOAT: VkFormat = 100;
pub const VK_FORMAT_R32G32_UINT: VkFormat = 101;
pub const VK_FORMAT_R32G32_SINT: VkFormat = 102;
pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = 103;
pub const VK_FORMAT_R32G32B32_UINT: VkFormat = 104;
pub const VK_FORMAT_R32G32B32_SINT: VkFormat = 105;
pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = 106;
pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = 107;
pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = 108;
pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = 109;
pub const VK_FORMAT_R64_UINT: VkFormat = 110;
pub const VK_FORMAT_R64_SINT: VkFormat = 111;
pub const VK_FORMAT_R64_SFLOAT: VkFormat = 112;
pub const VK_FORMAT_R64G64_UINT: VkFormat = 113;
pub const VK_FORMAT_R64G64_SINT: VkFormat = 114;
pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = 115;
pub const VK_FORMAT_R64G64B64_UINT: VkFormat = 116;
pub const VK_FORMAT_R64G64B64_SINT: VkFormat = 117;
pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = 118;
pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = 119;
pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = 120;
pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = 121;
pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = 122;
pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = 123;
pub const VK_FORMAT_D16_UNORM: VkFormat = 124;
pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = 125;
pub const VK_FORMAT_D32_SFLOAT: VkFormat = 126;
pub const VK_FORMAT_S8_UINT: VkFormat = 127;
pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = 128;
pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = 129;
pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = 130;
pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = 131;
pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = 132;
pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = 133;
pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = 134;
pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = 135;
pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = 136;
pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = 137;
pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = 138;
pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = 139;
pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = 140;
pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = 141;
pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = 142;
pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;
pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;
pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = 145;
pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = 146;
pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = 147;
pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = 148;
pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = 149;
pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = 150;
pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = 151;
pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = 152;
pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = 153;
pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = 154;
pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = 155;
pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = 156;
pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = 157;
pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = 158;
pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = 159;
pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = 160;
pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = 161;
pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = 162;
pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = 163;
pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = 164;
pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = 165;
pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = 166;
pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = 167;
pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = 168;
pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = 169;
pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = 170;
pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = 171;
pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = 172;
pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = 173;
pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = 174;
pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = 175;
pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = 176;
pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = 177;
pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = 178;
pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = 179;
pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = 180;
pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = 181;
pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = 182;
pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = 183;
pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = 184;
pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 1000156000;
pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 1000156001;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 1000156002;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 1000156003;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 1000156004;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 1000156005;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 1000156006;
pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 1000156007;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 1000156008;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 1000156009;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 1000156010;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 1000156011;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156012;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156013;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156014;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156015;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156016;
pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 1000156017;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 1000156018;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 1000156019;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 1000156020;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 1000156021;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156022;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156023;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156024;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156025;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 1000156027;
pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 1000156028;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 1000156029;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 1000156030;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 1000156031;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 1000156032;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 1000156033;
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: VkFormat = 1000330000;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330001;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330002;
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: VkFormat = 1000330003;
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: VkFormat = 1000340000;
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: VkFormat = 1000340001;
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: VkFormat = 1000066000;
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: VkFormat = 1000066001;
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: VkFormat = 1000066002;
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: VkFormat = 1000066003;
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: VkFormat = 1000066004;
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: VkFormat = 1000066005;
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: VkFormat = 1000066006;
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: VkFormat = 1000066007;
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: VkFormat = 1000066008;
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: VkFormat = 1000066009;
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: VkFormat = 1000066010;
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: VkFormat = 1000066011;
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: VkFormat = 1000066012;
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: VkFormat = 1000066013;
pub const VK_FORMAT_PVRTC1_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054000;
pub const VK_FORMAT_PVRTC1_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054001;
pub const VK_FORMAT_PVRTC2_2BPP_UNORM_BLOCK_IMG: VkFormat = 1000054002;
pub const VK_FORMAT_PVRTC2_4BPP_UNORM_BLOCK_IMG: VkFormat = 1000054003;
pub const VK_FORMAT_PVRTC1_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054004;
pub const VK_FORMAT_PVRTC1_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054005;
pub const VK_FORMAT_PVRTC2_2BPP_SRGB_BLOCK_IMG: VkFormat = 1000054006;
pub const VK_FORMAT_PVRTC2_4BPP_SRGB_BLOCK_IMG: VkFormat = 1000054007;
pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK_EXT: VkFormat = 1000066000;
pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK_EXT: VkFormat = 1000066001;
pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066002;
pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066003;
pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066004;
pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066005;
pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066006;
pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK_EXT: VkFormat = 1000066007;
pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK_EXT: VkFormat = 1000066008;
pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK_EXT: VkFormat = 1000066009;
pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK_EXT: VkFormat = 1000066010;
pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK_EXT: VkFormat = 1000066011;
pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK_EXT: VkFormat = 1000066012;
pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK_EXT: VkFormat = 1000066013;
pub const VK_FORMAT_G8B8G8R8_422_UNORM_KHR: VkFormat = 1000156000;
pub const VK_FORMAT_B8G8R8G8_422_UNORM_KHR: VkFormat = 1000156001;
pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM_KHR: VkFormat = 1000156002;
pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM_KHR: VkFormat = 1000156003;
pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM_KHR: VkFormat = 1000156004;
pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM_KHR: VkFormat = 1000156005;
pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM_KHR: VkFormat = 1000156006;
pub const VK_FORMAT_R10X6_UNORM_PACK16_KHR: VkFormat = 1000156007;
pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16_KHR: VkFormat = 1000156008;
pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: VkFormat = 1000156009;
pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: VkFormat = 1000156010;
pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: VkFormat = 1000156011;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156012;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156013;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156014;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156015;
pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = 1000156016;
pub const VK_FORMAT_R12X4_UNORM_PACK16_KHR: VkFormat = 1000156017;
pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16_KHR: VkFormat = 1000156018;
pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: VkFormat = 1000156019;
pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: VkFormat = 1000156020;
pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: VkFormat = 1000156021;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156022;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: VkFormat = 1000156023;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156024;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: VkFormat = 1000156025;
pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: VkFormat = 1000156026;
pub const VK_FORMAT_G16B16G16R16_422_UNORM_KHR: VkFormat = 1000156027;
pub const VK_FORMAT_B16G16R16G16_422_UNORM_KHR: VkFormat = 1000156028;
pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM_KHR: VkFormat = 1000156029;
pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM_KHR: VkFormat = 1000156030;
pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM_KHR: VkFormat = 1000156031;
pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM_KHR: VkFormat = 1000156032;
pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM_KHR: VkFormat = 1000156033;
pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM_EXT: VkFormat = 1000330000;
pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: VkFormat = 1000330001;
pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: VkFormat = 1000330002;
pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM_EXT: VkFormat = 1000330003;
pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16_EXT: VkFormat = 1000340000;
pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16_EXT: VkFormat = 1000340001;
pub const VK_FORMAT_MAX_ENUM: VkFormat = 2147483647;

pub type VkImageTiling = u32;
pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = 0;
pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = 1;
pub const VK_IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT: VkImageTiling = 1000158000;
pub const VK_IMAGE_TILING_MAX_ENUM: VkImageTiling = 2147483647;

pub type VkImageType = u32;
pub const VK_IMAGE_TYPE_1D: VkImageType = 0;
pub const VK_IMAGE_TYPE_2D: VkImageType = 1;
pub const VK_IMAGE_TYPE_3D: VkImageType = 2;
pub const VK_IMAGE_TYPE_MAX_ENUM: VkImageType = 2147483647;

pub type VkPhysicalDeviceType = u32;
pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = 0;
pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = 1;
pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = 2;
pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = 3;
pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = 4;
pub const VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM: VkPhysicalDeviceType = 2147483647;

pub type VkQueryType = u32;
pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = 0;
pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = 1;
pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = 2;
pub const VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT: VkQueryType = 1000028004;
pub const VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR: VkQueryType = 1000116000;
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: VkQueryType = 1000150000;
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: VkQueryType = 1000150001;
pub const VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: VkQueryType = 1000165000;
pub const VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL: VkQueryType = 1000210000;
pub const VK_QUERY_TYPE_PRIMITIVES_GENERATED_EXT: VkQueryType = 1000382000;
pub const VK_QUERY_TYPE_MAX_ENUM: VkQueryType = 2147483647;

pub type VkSharingMode = u32;
pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;
pub const VK_SHARING_MODE_MAX_ENUM: VkSharingMode = 2147483647;

pub type VkComponentSwizzle = u32;
pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;
pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = 1;
pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = 2;
pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = 3;
pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = 4;
pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = 5;
pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = 6;
pub const VK_COMPONENT_SWIZZLE_MAX_ENUM: VkComponentSwizzle = 2147483647;

pub type VkImageViewType = u32;
pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = 0;
pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;
pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = 2;
pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = 3;
pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = 4;
pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = 5;
pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = 6;
pub const VK_IMAGE_VIEW_TYPE_MAX_ENUM: VkImageViewType = 2147483647;

pub type VkBlendFactor = u32;
pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = 0;
pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = 1;
pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = 2;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = 3;
pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = 4;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = 5;
pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = 6;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = 7;
pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = 8;
pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = 9;
pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = 10;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = 11;
pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = 12;
pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = 13;
pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = 14;
pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = 15;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = 16;
pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = 17;
pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = 18;
pub const VK_BLEND_FACTOR_MAX_ENUM: VkBlendFactor = 2147483647;

pub type VkBlendOp = u32;
pub const VK_BLEND_OP_ADD: VkBlendOp = 0;
pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = 1;
pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = 2;
pub const VK_BLEND_OP_MIN: VkBlendOp = 3;
pub const VK_BLEND_OP_MAX: VkBlendOp = 4;
pub const VK_BLEND_OP_ZERO_EXT: VkBlendOp = 1000148000;
pub const VK_BLEND_OP_SRC_EXT: VkBlendOp = 1000148001;
pub const VK_BLEND_OP_DST_EXT: VkBlendOp = 1000148002;
pub const VK_BLEND_OP_SRC_OVER_EXT: VkBlendOp = 1000148003;
pub const VK_BLEND_OP_DST_OVER_EXT: VkBlendOp = 1000148004;
pub const VK_BLEND_OP_SRC_IN_EXT: VkBlendOp = 1000148005;
pub const VK_BLEND_OP_DST_IN_EXT: VkBlendOp = 1000148006;
pub const VK_BLEND_OP_SRC_OUT_EXT: VkBlendOp = 1000148007;
pub const VK_BLEND_OP_DST_OUT_EXT: VkBlendOp = 1000148008;
pub const VK_BLEND_OP_SRC_ATOP_EXT: VkBlendOp = 1000148009;
pub const VK_BLEND_OP_DST_ATOP_EXT: VkBlendOp = 1000148010;
pub const VK_BLEND_OP_XOR_EXT: VkBlendOp = 1000148011;
pub const VK_BLEND_OP_MULTIPLY_EXT: VkBlendOp = 1000148012;
pub const VK_BLEND_OP_SCREEN_EXT: VkBlendOp = 1000148013;
pub const VK_BLEND_OP_OVERLAY_EXT: VkBlendOp = 1000148014;
pub const VK_BLEND_OP_DARKEN_EXT: VkBlendOp = 1000148015;
pub const VK_BLEND_OP_LIGHTEN_EXT: VkBlendOp = 1000148016;
pub const VK_BLEND_OP_COLORDODGE_EXT: VkBlendOp = 1000148017;
pub const VK_BLEND_OP_COLORBURN_EXT: VkBlendOp = 1000148018;
pub const VK_BLEND_OP_HARDLIGHT_EXT: VkBlendOp = 1000148019;
pub const VK_BLEND_OP_SOFTLIGHT_EXT: VkBlendOp = 1000148020;
pub const VK_BLEND_OP_DIFFERENCE_EXT: VkBlendOp = 1000148021;
pub const VK_BLEND_OP_EXCLUSION_EXT: VkBlendOp = 1000148022;
pub const VK_BLEND_OP_INVERT_EXT: VkBlendOp = 1000148023;
pub const VK_BLEND_OP_INVERT_RGB_EXT: VkBlendOp = 1000148024;
pub const VK_BLEND_OP_LINEARDODGE_EXT: VkBlendOp = 1000148025;
pub const VK_BLEND_OP_LINEARBURN_EXT: VkBlendOp = 1000148026;
pub const VK_BLEND_OP_VIVIDLIGHT_EXT: VkBlendOp = 1000148027;
pub const VK_BLEND_OP_LINEARLIGHT_EXT: VkBlendOp = 1000148028;
pub const VK_BLEND_OP_PINLIGHT_EXT: VkBlendOp = 1000148029;
pub const VK_BLEND_OP_HARDMIX_EXT: VkBlendOp = 1000148030;
pub const VK_BLEND_OP_HSL_HUE_EXT: VkBlendOp = 1000148031;
pub const VK_BLEND_OP_HSL_SATURATION_EXT: VkBlendOp = 1000148032;
pub const VK_BLEND_OP_HSL_COLOR_EXT: VkBlendOp = 1000148033;
pub const VK_BLEND_OP_HSL_LUMINOSITY_EXT: VkBlendOp = 1000148034;
pub const VK_BLEND_OP_PLUS_EXT: VkBlendOp = 1000148035;
pub const VK_BLEND_OP_PLUS_CLAMPED_EXT: VkBlendOp = 1000148036;
pub const VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT: VkBlendOp = 1000148037;
pub const VK_BLEND_OP_PLUS_DARKER_EXT: VkBlendOp = 1000148038;
pub const VK_BLEND_OP_MINUS_EXT: VkBlendOp = 1000148039;
pub const VK_BLEND_OP_MINUS_CLAMPED_EXT: VkBlendOp = 1000148040;
pub const VK_BLEND_OP_CONTRAST_EXT: VkBlendOp = 1000148041;
pub const VK_BLEND_OP_INVERT_OVG_EXT: VkBlendOp = 1000148042;
pub const VK_BLEND_OP_RED_EXT: VkBlendOp = 1000148043;
pub const VK_BLEND_OP_GREEN_EXT: VkBlendOp = 1000148044;
pub const VK_BLEND_OP_BLUE_EXT: VkBlendOp = 1000148045;
pub const VK_BLEND_OP_MAX_ENUM: VkBlendOp = 2147483647;

pub type VkCompareOp = u32;
pub const VK_COMPARE_OP_NEVER: VkCompareOp = 0;
pub const VK_COMPARE_OP_LESS: VkCompareOp = 1;
pub const VK_COMPARE_OP_EQUAL: VkCompareOp = 2;
pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = 3;
pub const VK_COMPARE_OP_GREATER: VkCompareOp = 4;
pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = 5;
pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = 6;
pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = 7;
pub const VK_COMPARE_OP_MAX_ENUM: VkCompareOp = 2147483647;

pub type VkDynamicState = u32;
pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = 0;
pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = 1;
pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = 2;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = 3;
pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = 4;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = 5;
pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = 6;
pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = 7;
pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = 8;
pub const VK_DYNAMIC_STATE_CULL_MODE: VkDynamicState = 1000267000;
pub const VK_DYNAMIC_STATE_FRONT_FACE: VkDynamicState = 1000267001;
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY: VkDynamicState = 1000267002;
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT: VkDynamicState = 1000267003;
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT: VkDynamicState = 1000267004;
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE: VkDynamicState = 1000267005;
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE: VkDynamicState = 1000267006;
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE: VkDynamicState = 1000267007;
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP: VkDynamicState = 1000267008;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE: VkDynamicState = 1000267009;
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE: VkDynamicState = 1000267010;
pub const VK_DYNAMIC_STATE_STENCIL_OP: VkDynamicState = 1000267011;
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE: VkDynamicState = 1000377001;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE: VkDynamicState = 1000377002;
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE: VkDynamicState = 1000377004;
pub const VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV: VkDynamicState = 1000087000;
pub const VK_DYNAMIC_STATE_DISCARD_RECTANGLE_EXT: VkDynamicState = 1000099000;
pub const VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT: VkDynamicState = 1000143000;
pub const VK_DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR: VkDynamicState = 1000347000;
pub const VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV: VkDynamicState = 1000164004;
pub const VK_DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV: VkDynamicState = 1000164006;
pub const VK_DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV: VkDynamicState = 1000205001;
pub const VK_DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR: VkDynamicState = 1000226000;
pub const VK_DYNAMIC_STATE_LINE_STIPPLE_EXT: VkDynamicState = 1000259000;
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_EXT: VkDynamicState = 1000352000;
pub const VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT: VkDynamicState = 1000377000;
pub const VK_DYNAMIC_STATE_LOGIC_OP_EXT: VkDynamicState = 1000377003;
pub const VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT: VkDynamicState = 1000381000;
pub const VK_DYNAMIC_STATE_CULL_MODE_EXT: VkDynamicState = 1000267000;
pub const VK_DYNAMIC_STATE_FRONT_FACE_EXT: VkDynamicState = 1000267001;
pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT: VkDynamicState = 1000267002;
pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT: VkDynamicState = 1000267003;
pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT: VkDynamicState = 1000267004;
pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT: VkDynamicState = 1000267005;
pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT: VkDynamicState = 1000267006;
pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT: VkDynamicState = 1000267007;
pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT: VkDynamicState = 1000267008;
pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT: VkDynamicState = 1000267009;
pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT: VkDynamicState = 1000267010;
pub const VK_DYNAMIC_STATE_STENCIL_OP_EXT: VkDynamicState = 1000267011;
pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT: VkDynamicState = 1000377001;
pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT: VkDynamicState = 1000377002;
pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT: VkDynamicState = 1000377004;
pub const VK_DYNAMIC_STATE_MAX_ENUM: VkDynamicState = 2147483647;

pub type VkFrontFace = u32;
pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = 0;
pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = 1;
pub const VK_FRONT_FACE_MAX_ENUM: VkFrontFace = 2147483647;

pub type VkVertexInputRate = u32;
pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = 0;
pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = 1;
pub const VK_VERTEX_INPUT_RATE_MAX_ENUM: VkVertexInputRate = 2147483647;

pub type VkPrimitiveTopology = u32;
pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = 0;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = 1;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = 2;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = 3;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = 4;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = 5;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 6;
pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 7;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 8;
pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 9;
pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = 10;
pub const VK_PRIMITIVE_TOPOLOGY_MAX_ENUM: VkPrimitiveTopology = 2147483647;

pub type VkPolygonMode = u32;
pub const VK_POLYGON_MODE_FILL: VkPolygonMode = 0;
pub const VK_POLYGON_MODE_LINE: VkPolygonMode = 1;
pub const VK_POLYGON_MODE_POINT: VkPolygonMode = 2;
pub const VK_POLYGON_MODE_FILL_RECTANGLE_NV: VkPolygonMode = 1000153000;
pub const VK_POLYGON_MODE_MAX_ENUM: VkPolygonMode = 2147483647;

pub type VkStencilOp = u32;
pub const VK_STENCIL_OP_KEEP: VkStencilOp = 0;
pub const VK_STENCIL_OP_ZERO: VkStencilOp = 1;
pub const VK_STENCIL_OP_REPLACE: VkStencilOp = 2;
pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = 3;
pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = 4;
pub const VK_STENCIL_OP_INVERT: VkStencilOp = 5;
pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = 6;
pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = 7;
pub const VK_STENCIL_OP_MAX_ENUM: VkStencilOp = 2147483647;

pub type VkLogicOp = u32;
pub const VK_LOGIC_OP_CLEAR: VkLogicOp = 0;
pub const VK_LOGIC_OP_AND: VkLogicOp = 1;
pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = 2;
pub const VK_LOGIC_OP_COPY: VkLogicOp = 3;
pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = 4;
pub const VK_LOGIC_OP_NO_OP: VkLogicOp = 5;
pub const VK_LOGIC_OP_XOR: VkLogicOp = 6;
pub const VK_LOGIC_OP_OR: VkLogicOp = 7;
pub const VK_LOGIC_OP_NOR: VkLogicOp = 8;
pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = 9;
pub const VK_LOGIC_OP_INVERT: VkLogicOp = 10;
pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = 11;
pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = 12;
pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = 13;
pub const VK_LOGIC_OP_NAND: VkLogicOp = 14;
pub const VK_LOGIC_OP_SET: VkLogicOp = 15;
pub const VK_LOGIC_OP_MAX_ENUM: VkLogicOp = 2147483647;

pub type VkBorderColor = u32;
pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = 0;
pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = 1;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = 2;
pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = 3;
pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = 4;
pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = 5;
pub const VK_BORDER_COLOR_FLOAT_CUSTOM_EXT: VkBorderColor = 1000287003;
pub const VK_BORDER_COLOR_INT_CUSTOM_EXT: VkBorderColor = 1000287004;
pub const VK_BORDER_COLOR_MAX_ENUM: VkBorderColor = 2147483647;

pub type VkFilter = u32;
pub const VK_FILTER_NEAREST: VkFilter = 0;
pub const VK_FILTER_LINEAR: VkFilter = 1;
pub const VK_FILTER_CUBIC_IMG: VkFilter = 1000015000;
pub const VK_FILTER_CUBIC_EXT: VkFilter = 1000015000;
pub const VK_FILTER_MAX_ENUM: VkFilter = 2147483647;

pub type VkSamplerAddressMode = u32;
pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = 0;
pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = 1;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = 2;
pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = 3;
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE: VkSamplerAddressMode = 4;
pub const VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR: VkSamplerAddressMode = 4;
pub const VK_SAMPLER_ADDRESS_MODE_MAX_ENUM: VkSamplerAddressMode = 2147483647;

pub type VkSamplerMipmapMode = u32;
pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;
pub const VK_SAMPLER_MIPMAP_MODE_MAX_ENUM: VkSamplerMipmapMode = 2147483647;

pub type VkDescriptorType = u32;
pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: VkDescriptorType = 1000138000;
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR: VkDescriptorType = 1000150000;
pub const VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV: VkDescriptorType = 1000165000;
pub const VK_DESCRIPTOR_TYPE_MUTABLE_VALVE: VkDescriptorType = 1000351000;
pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT: VkDescriptorType = 1000138000;
pub const VK_DESCRIPTOR_TYPE_MAX_ENUM: VkDescriptorType = 2147483647;

pub type VkAttachmentLoadOp = u32;
pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = 0;
pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = 1;
pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = 2;
pub const VK_ATTACHMENT_LOAD_OP_NONE_EXT: VkAttachmentLoadOp = 1000400000;
pub const VK_ATTACHMENT_LOAD_OP_MAX_ENUM: VkAttachmentLoadOp = 2147483647;

pub type VkAttachmentStoreOp = u32;
pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = 0;
pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = 1;
pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp = 1000301000;
pub const VK_ATTACHMENT_STORE_OP_NONE_KHR: VkAttachmentStoreOp = 1000301000;
pub const VK_ATTACHMENT_STORE_OP_NONE_QCOM: VkAttachmentStoreOp = 1000301000;
pub const VK_ATTACHMENT_STORE_OP_NONE_EXT: VkAttachmentStoreOp = 1000301000;
pub const VK_ATTACHMENT_STORE_OP_MAX_ENUM: VkAttachmentStoreOp = 2147483647;

pub type VkPipelineBindPoint = u32;
pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR: VkPipelineBindPoint = 1000165000;
pub const VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI: VkPipelineBindPoint = 1000369003;
pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_NV: VkPipelineBindPoint = 1000165000;
pub const VK_PIPELINE_BIND_POINT_MAX_ENUM: VkPipelineBindPoint = 2147483647;

pub type VkCommandBufferLevel = u32;
pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;
pub const VK_COMMAND_BUFFER_LEVEL_MAX_ENUM: VkCommandBufferLevel = 2147483647;

pub type VkIndexType = u32;
pub const VK_INDEX_TYPE_UINT16: VkIndexType = 0;
pub const VK_INDEX_TYPE_UINT32: VkIndexType = 1;
pub const VK_INDEX_TYPE_NONE_KHR: VkIndexType = 1000165000;
pub const VK_INDEX_TYPE_UINT8_EXT: VkIndexType = 1000265000;
pub const VK_INDEX_TYPE_NONE_NV: VkIndexType = 1000165000;
pub const VK_INDEX_TYPE_MAX_ENUM: VkIndexType = 2147483647;

pub type VkSubpassContents = u32;
pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = 0;
pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = 1;
pub const VK_SUBPASS_CONTENTS_MAX_ENUM: VkSubpassContents = 2147483647;

pub type VkAccessFlags = VkFlags;
pub type VkAccessFlagBits = VkFlags;
pub const VK_ACCESS_NONE_KHR: VkAccessFlagBits = 0;
pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits = 1;
pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlagBits = 2;
pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits = 4;
pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlagBits = 8;
pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits = 16;
pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlagBits = 32;
pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlagBits = 64;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits = 128;
pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 256;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits = 512;
pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits = 1024;
pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlagBits = 2048;
pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlagBits = 4096;
pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlagBits = 8192;
pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlagBits = 16384;
pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlagBits = 32768;
pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlagBits = 65536;
pub const VK_ACCESS_NONE: VkAccessFlagBits = 0;
pub const VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits = 33554432;
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits = 67108864;
pub const VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits = 134217728;
pub const VK_ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits = 1048576;
pub const VK_ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits = 524288;
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits = 2097152;
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits = 4194304;
pub const VK_ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits = 16777216;
pub const VK_ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits = 8388608;
pub const VK_ACCESS_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits = 131072;
pub const VK_ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits = 262144;
pub const VK_ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits = 8388608;
pub const VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits = 2097152;
pub const VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits = 4194304;
pub const VK_ACCESS_FLAG_BITS_MAX_ENUM: VkAccessFlagBits = 2147483647;

pub type VkImageAspectFlags = VkFlags;
pub type VkImageAspectFlagBits = VkFlags;
pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlagBits = 1;
pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlagBits = 2;
pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlagBits = 4;
pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlagBits = 8;
pub const VK_IMAGE_ASPECT_PLANE_0_BIT: VkImageAspectFlagBits = 16;
pub const VK_IMAGE_ASPECT_PLANE_1_BIT: VkImageAspectFlagBits = 32;
pub const VK_IMAGE_ASPECT_PLANE_2_BIT: VkImageAspectFlagBits = 64;
pub const VK_IMAGE_ASPECT_NONE: VkImageAspectFlagBits = 0;
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT: VkImageAspectFlagBits = 128;
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT: VkImageAspectFlagBits = 256;
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT: VkImageAspectFlagBits = 512;
pub const VK_IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT: VkImageAspectFlagBits = 1024;
pub const VK_IMAGE_ASPECT_PLANE_0_BIT_KHR: VkImageAspectFlagBits = 16;
pub const VK_IMAGE_ASPECT_PLANE_1_BIT_KHR: VkImageAspectFlagBits = 32;
pub const VK_IMAGE_ASPECT_PLANE_2_BIT_KHR: VkImageAspectFlagBits = 64;
pub const VK_IMAGE_ASPECT_NONE_KHR: VkImageAspectFlagBits = 0;
pub const VK_IMAGE_ASPECT_FLAG_BITS_MAX_ENUM: VkImageAspectFlagBits = 2147483647;

pub type VkFormatFeatureFlags = VkFlags;
pub type VkFormatFeatureFlagBits = VkFlags;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits = 1;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits = 2;
pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits = 4;
pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 8;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits = 16;
pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits = 32;
pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits = 64;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 128;
pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits = 256;
pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits = 512;
pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlagBits = 1024;
pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlagBits = 2048;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits = 4096;
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits = 16384;
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT: VkFormatFeatureFlagBits = 32768;
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 131072;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits = 262144;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits = 524288;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits = 1048576;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits = 2097152;
pub const VK_FORMAT_FEATURE_DISJOINT_BIT: VkFormatFeatureFlagBits = 4194304;
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits = 8388608;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits = 65536;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_IMG: VkFormatFeatureFlagBits = 8192;
pub const VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits = 536870912;
pub const VK_FORMAT_FEATURE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits = 16777216;
pub const VK_FORMAT_FEATURE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits = 1073741824;
pub const VK_FORMAT_FEATURE_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits = 16384;
pub const VK_FORMAT_FEATURE_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits = 32768;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_MINMAX_BIT_EXT: VkFormatFeatureFlagBits = 65536;
pub const VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = 131072;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits = 262144;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits = 524288;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits = 1048576;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits = 2097152;
pub const VK_FORMAT_FEATURE_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits = 4194304;
pub const VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits = 8388608;
pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits = 8192;
pub const VK_FORMAT_FEATURE_FLAG_BITS_MAX_ENUM: VkFormatFeatureFlagBits = 2147483647;

pub type VkImageCreateFlags = VkFlags;
pub type VkImageCreateFlagBits = VkFlags;
pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlagBits = 1;
pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlagBits = 2;
pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlagBits = 4;
pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlagBits = 8;
pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlagBits = 16;
pub const VK_IMAGE_CREATE_ALIAS_BIT: VkImageCreateFlagBits = 1024;
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT: VkImageCreateFlagBits = 64;
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT: VkImageCreateFlagBits = 32;
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT: VkImageCreateFlagBits = 128;
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT: VkImageCreateFlagBits = 256;
pub const VK_IMAGE_CREATE_PROTECTED_BIT: VkImageCreateFlagBits = 2048;
pub const VK_IMAGE_CREATE_DISJOINT_BIT: VkImageCreateFlagBits = 512;
pub const VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV: VkImageCreateFlagBits = 8192;
pub const VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT: VkImageCreateFlagBits = 4096;
pub const VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT: VkImageCreateFlagBits = 16384;
pub const VK_IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT: VkImageCreateFlagBits = 131072;
pub const VK_IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM: VkImageCreateFlagBits = 32768;
pub const VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkImageCreateFlagBits = 64;
pub const VK_IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = 32;
pub const VK_IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR: VkImageCreateFlagBits = 128;
pub const VK_IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR: VkImageCreateFlagBits = 256;
pub const VK_IMAGE_CREATE_DISJOINT_BIT_KHR: VkImageCreateFlagBits = 512;
pub const VK_IMAGE_CREATE_ALIAS_BIT_KHR: VkImageCreateFlagBits = 1024;
pub const VK_IMAGE_CREATE_FLAG_BITS_MAX_ENUM: VkImageCreateFlagBits = 2147483647;

pub type VkSampleCountFlags = VkFlags;
pub type VkSampleCountFlagBits = VkFlags;
pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlagBits = 1;
pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlagBits = 2;
pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlagBits = 4;
pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlagBits = 8;
pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlagBits = 16;
pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlagBits = 32;
pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlagBits = 64;
pub const VK_SAMPLE_COUNT_FLAG_BITS_MAX_ENUM: VkSampleCountFlagBits = 2147483647;

pub type VkImageUsageFlags = VkFlags;
pub type VkImageUsageFlagBits = VkFlags;
pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlagBits = 1;
pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlagBits = 2;
pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlagBits = 4;
pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlagBits = 8;
pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlagBits = 16;
pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlagBits = 32;
pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlagBits = 64;
pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlagBits = 128;
pub const VK_IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT: VkImageUsageFlagBits = 512;
pub const VK_IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkImageUsageFlagBits = 256;
pub const VK_IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI: VkImageUsageFlagBits = 262144;
pub const VK_IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV: VkImageUsageFlagBits = 256;
pub const VK_IMAGE_USAGE_FLAG_BITS_MAX_ENUM: VkImageUsageFlagBits = 2147483647;

pub type VkInstanceCreateFlags = VkFlags;
pub type VkInstanceCreateFlagBits = VkFlags;
pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR: VkInstanceCreateFlagBits = 1;
pub const VK_INSTANCE_CREATE_FLAG_BITS_MAX_ENUM: VkInstanceCreateFlagBits = 2147483647;

pub type VkMemoryHeapFlags = VkFlags;
pub type VkMemoryHeapFlagBits = VkFlags;
pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlagBits = 1;
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT: VkMemoryHeapFlagBits = 2;
pub const VK_MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR: VkMemoryHeapFlagBits = 2;
pub const VK_MEMORY_HEAP_FLAG_BITS_MAX_ENUM: VkMemoryHeapFlagBits = 2147483647;

pub type VkMemoryPropertyFlags = VkFlags;
pub type VkMemoryPropertyFlagBits = VkFlags;
pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlagBits = 1;
pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlagBits = 2;
pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlagBits = 4;
pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlagBits = 8;
pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlagBits = 16;
pub const VK_MEMORY_PROPERTY_PROTECTED_BIT: VkMemoryPropertyFlagBits = 32;
pub const VK_MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD: VkMemoryPropertyFlagBits = 64;
pub const VK_MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD: VkMemoryPropertyFlagBits = 128;
pub const VK_MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV: VkMemoryPropertyFlagBits = 256;
pub const VK_MEMORY_PROPERTY_FLAG_BITS_MAX_ENUM: VkMemoryPropertyFlagBits = 2147483647;

pub type VkQueueFlags = VkFlags;
pub type VkQueueFlagBits = VkFlags;
pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlagBits = 1;
pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlagBits = 2;
pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlagBits = 4;
pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlagBits = 8;
pub const VK_QUEUE_PROTECTED_BIT: VkQueueFlagBits = 16;
pub const VK_QUEUE_FLAG_BITS_MAX_ENUM: VkQueueFlagBits = 2147483647;

pub type VkDeviceCreateFlags = VkFlags;

pub type VkDeviceQueueCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlagBits = VkFlags;
pub const VK_DEVICE_QUEUE_CREATE_PROTECTED_BIT: VkDeviceQueueCreateFlagBits = 1;
pub const VK_DEVICE_QUEUE_CREATE_FLAG_BITS_MAX_ENUM: VkDeviceQueueCreateFlagBits = 2147483647;

pub type VkPipelineStageFlags = VkFlags;
pub type VkPipelineStageFlagBits = VkFlags;
pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits = 1;
pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits = 2;
pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlagBits = 4;
pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlagBits = 8;
pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits = 16;
pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits = 32;
pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits = 64;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits = 128;
pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 256;
pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits = 512;
pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits = 1024;
pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits = 2048;
pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlagBits = 4096;
pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits = 8192;
pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlagBits = 16384;
pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits = 32768;
pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlagBits = 65536;
pub const VK_PIPELINE_STAGE_NONE: VkPipelineStageFlagBits = 0;
pub const VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits = 16777216;
pub const VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits = 262144;
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR: VkPipelineStageFlagBits = 33554432;
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits = 2097152;
pub const VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits = 524288;
pub const VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits = 1048576;
pub const VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits = 8388608;
pub const VK_PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineStageFlagBits = 4194304;
pub const VK_PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits = 131072;
pub const VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits = 4194304;
pub const VK_PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits = 2097152;
pub const VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits = 33554432;
pub const VK_PIPELINE_STAGE_NONE_KHR: VkPipelineStageFlagBits = 0;
pub const VK_PIPELINE_STAGE_FLAG_BITS_MAX_ENUM: VkPipelineStageFlagBits = 2147483647;

pub type VkMemoryMapFlags = VkFlags;

pub type VkSparseMemoryBindFlags = VkFlags;
pub type VkSparseMemoryBindFlagBits = VkFlags;
pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlagBits = 1;
pub const VK_SPARSE_MEMORY_BIND_FLAG_BITS_MAX_ENUM: VkSparseMemoryBindFlagBits = 2147483647;

pub type VkSparseImageFormatFlags = VkFlags;
pub type VkSparseImageFormatFlagBits = VkFlags;
pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlagBits = 1;
pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlagBits = 2;
pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlagBits = 4;
pub const VK_SPARSE_IMAGE_FORMAT_FLAG_BITS_MAX_ENUM: VkSparseImageFormatFlagBits = 2147483647;

pub type VkFenceCreateFlags = VkFlags;
pub type VkFenceCreateFlagBits = VkFlags;
pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlagBits = 1;
pub const VK_FENCE_CREATE_FLAG_BITS_MAX_ENUM: VkFenceCreateFlagBits = 2147483647;

pub type VkSemaphoreCreateFlags = VkFlags;

pub type VkEventCreateFlags = VkFlags;
pub type VkEventCreateFlagBits = VkFlags;
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT: VkEventCreateFlagBits = 1;
pub const VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR: VkEventCreateFlagBits = 1;
pub const VK_EVENT_CREATE_FLAG_BITS_MAX_ENUM: VkEventCreateFlagBits = 2147483647;

pub type VkQueryPipelineStatisticFlags = VkFlags;
pub type VkQueryPipelineStatisticFlagBits = VkFlags;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlagBits = 1;
pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 2;
pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 4;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 8;
pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 16;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 32;
pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlagBits = 64;
pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 128;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlagBits = 256;
pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 512;
pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlagBits = 1024;
pub const VK_QUERY_PIPELINE_STATISTIC_FLAG_BITS_MAX_ENUM: VkQueryPipelineStatisticFlagBits = 2147483647;

pub type VkQueryPoolCreateFlags = VkFlags;

pub type VkQueryResultFlags = VkFlags;
pub type VkQueryResultFlagBits = VkFlags;
pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlagBits = 1;
pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlagBits = 2;
pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlagBits = 4;
pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlagBits = 8;
pub const VK_QUERY_RESULT_FLAG_BITS_MAX_ENUM: VkQueryResultFlagBits = 2147483647;

pub type VkBufferCreateFlags = VkFlags;
pub type VkBufferCreateFlagBits = VkFlags;
pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlagBits = 1;
pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlagBits = 2;
pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlagBits = 4;
pub const VK_BUFFER_CREATE_PROTECTED_BIT: VkBufferCreateFlagBits = 8;
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkBufferCreateFlagBits = 16;
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkBufferCreateFlagBits = 16;
pub const VK_BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkBufferCreateFlagBits = 16;
pub const VK_BUFFER_CREATE_FLAG_BITS_MAX_ENUM: VkBufferCreateFlagBits = 2147483647;

pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferUsageFlagBits = VkFlags;
pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlagBits = 1;
pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlagBits = 2;
pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 4;
pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlagBits = 8;
pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlagBits = 16;
pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlagBits = 32;
pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlagBits = 64;
pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlagBits = 128;
pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlagBits = 256;
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT: VkBufferUsageFlagBits = 131072;
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT: VkBufferUsageFlagBits = 2048;
pub const VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT: VkBufferUsageFlagBits = 4096;
pub const VK_BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT: VkBufferUsageFlagBits = 512;
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR: VkBufferUsageFlagBits = 524288;
pub const VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR: VkBufferUsageFlagBits = 1048576;
pub const VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR: VkBufferUsageFlagBits = 1024;
pub const VK_BUFFER_USAGE_RAY_TRACING_BIT_NV: VkBufferUsageFlagBits = 1024;
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT: VkBufferUsageFlagBits = 131072;
pub const VK_BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR: VkBufferUsageFlagBits = 131072;
pub const VK_BUFFER_USAGE_FLAG_BITS_MAX_ENUM: VkBufferUsageFlagBits = 2147483647;

pub type VkBufferViewCreateFlags = VkFlags;

pub type VkImageViewCreateFlags = VkFlags;
pub type VkImageViewCreateFlagBits = VkFlags;
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT: VkImageViewCreateFlagBits = 1;
pub const VK_IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT: VkImageViewCreateFlagBits = 2;
pub const VK_IMAGE_VIEW_CREATE_FLAG_BITS_MAX_ENUM: VkImageViewCreateFlagBits = 2147483647;

pub type VkShaderModuleCreateFlags = VkFlags;

pub type VkPipelineCacheCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlagBits = VkFlags;
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT: VkPipelineCacheCreateFlagBits = 1;
pub const VK_PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT: VkPipelineCacheCreateFlagBits = 1;
pub const VK_PIPELINE_CACHE_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineCacheCreateFlagBits = 2147483647;

pub type VkColorComponentFlags = VkFlags;
pub type VkColorComponentFlagBits = VkFlags;
pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlagBits = 1;
pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlagBits = 2;
pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlagBits = 4;
pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlagBits = 8;
pub const VK_COLOR_COMPONENT_FLAG_BITS_MAX_ENUM: VkColorComponentFlagBits = 2147483647;

pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineCreateFlagBits = VkFlags;
pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlagBits = 1;
pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlagBits = 2;
pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlagBits = 4;
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlagBits = 8;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_BIT: VkPipelineCreateFlagBits = 16;
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlagBits = 256;
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlagBits = 512;
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineCreateFlagBits = 2097152;
pub const VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: VkPipelineCreateFlagBits = 4194304;
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = 16384;
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = 32768;
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = 65536;
pub const VK_PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR: VkPipelineCreateFlagBits = 131072;
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR: VkPipelineCreateFlagBits = 4096;
pub const VK_PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR: VkPipelineCreateFlagBits = 8192;
pub const VK_PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR: VkPipelineCreateFlagBits = 524288;
pub const VK_PIPELINE_CREATE_DEFER_COMPILE_BIT_NV: VkPipelineCreateFlagBits = 32;
pub const VK_PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR: VkPipelineCreateFlagBits = 64;
pub const VK_PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR: VkPipelineCreateFlagBits = 128;
pub const VK_PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV: VkPipelineCreateFlagBits = 262144;
pub const VK_PIPELINE_CREATE_LIBRARY_BIT_KHR: VkPipelineCreateFlagBits = 2048;
pub const VK_PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT: VkPipelineCreateFlagBits = 8388608;
pub const VK_PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT: VkPipelineCreateFlagBits = 1024;
pub const VK_PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV: VkPipelineCreateFlagBits = 1048576;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE: VkPipelineCreateFlagBits = 16;
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineCreateFlagBits = 2097152;
pub const VK_PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT: VkPipelineCreateFlagBits = 4194304;
pub const VK_PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR: VkPipelineCreateFlagBits = 8;
pub const VK_PIPELINE_CREATE_DISPATCH_BASE_KHR: VkPipelineCreateFlagBits = 16;
pub const VK_PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT: VkPipelineCreateFlagBits = 256;
pub const VK_PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT: VkPipelineCreateFlagBits = 512;
pub const VK_PIPELINE_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineCreateFlagBits = 2147483647;

pub type VkPipelineShaderStageCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlagBits = VkFlags;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT: VkPipelineShaderStageCreateFlagBits = 1;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT: VkPipelineShaderStageCreateFlagBits = 2;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT: VkPipelineShaderStageCreateFlagBits = 1;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT: VkPipelineShaderStageCreateFlagBits = 2;
pub const VK_PIPELINE_SHADER_STAGE_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineShaderStageCreateFlagBits = 2147483647;

pub type VkShaderStageFlags = VkFlags;
pub type VkShaderStageFlagBits = VkFlags;
pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlagBits = 1;
pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlagBits = 2;
pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlagBits = 4;
pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlagBits = 8;
pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlagBits = 16;
pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlagBits = 32;
pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlagBits = 31;
pub const VK_SHADER_STAGE_ALL: VkShaderStageFlagBits = 2147483647;
pub const VK_SHADER_STAGE_RAYGEN_BIT_KHR: VkShaderStageFlagBits = 256;
pub const VK_SHADER_STAGE_ANY_HIT_BIT_KHR: VkShaderStageFlagBits = 512;
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_KHR: VkShaderStageFlagBits = 1024;
pub const VK_SHADER_STAGE_MISS_BIT_KHR: VkShaderStageFlagBits = 2048;
pub const VK_SHADER_STAGE_INTERSECTION_BIT_KHR: VkShaderStageFlagBits = 4096;
pub const VK_SHADER_STAGE_CALLABLE_BIT_KHR: VkShaderStageFlagBits = 8192;
pub const VK_SHADER_STAGE_TASK_BIT_NV: VkShaderStageFlagBits = 64;
pub const VK_SHADER_STAGE_MESH_BIT_NV: VkShaderStageFlagBits = 128;
pub const VK_SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI: VkShaderStageFlagBits = 16384;
pub const VK_SHADER_STAGE_RAYGEN_BIT_NV: VkShaderStageFlagBits = 256;
pub const VK_SHADER_STAGE_ANY_HIT_BIT_NV: VkShaderStageFlagBits = 512;
pub const VK_SHADER_STAGE_CLOSEST_HIT_BIT_NV: VkShaderStageFlagBits = 1024;
pub const VK_SHADER_STAGE_MISS_BIT_NV: VkShaderStageFlagBits = 2048;
pub const VK_SHADER_STAGE_INTERSECTION_BIT_NV: VkShaderStageFlagBits = 4096;
pub const VK_SHADER_STAGE_CALLABLE_BIT_NV: VkShaderStageFlagBits = 8192;
pub const VK_SHADER_STAGE_FLAG_BITS_MAX_ENUM: VkShaderStageFlagBits = 2147483647;

pub type VkCullModeFlags = VkFlags;
pub type VkCullModeFlagBits = VkFlags;
pub const VK_CULL_MODE_NONE: VkCullModeFlagBits = 0;
pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlagBits = 1;
pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlagBits = 2;
pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlagBits = 3;
pub const VK_CULL_MODE_FLAG_BITS_MAX_ENUM: VkCullModeFlagBits = 2147483647;

pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;

pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlagBits = VkFlags;
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = 1;
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: VkPipelineDepthStencilStateCreateFlagBits = 2;
pub const VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineDepthStencilStateCreateFlagBits = 2147483647;

pub type VkPipelineColorBlendStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlagBits = VkFlags;
pub const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM: VkPipelineColorBlendStateCreateFlagBits = 1;
pub const VK_PIPELINE_COLOR_BLEND_STATE_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineColorBlendStateCreateFlagBits = 2147483647;

pub type VkPipelineDynamicStateCreateFlags = VkFlags;

pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkPipelineLayoutCreateFlagBits = VkFlags;
pub const VK_PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT: VkPipelineLayoutCreateFlagBits = 2;
pub const VK_PIPELINE_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM: VkPipelineLayoutCreateFlagBits = 2147483647;

pub type VkSamplerCreateFlags = VkFlags;
pub type VkSamplerCreateFlagBits = VkFlags;
pub const VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT: VkSamplerCreateFlagBits = 1;
pub const VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT: VkSamplerCreateFlagBits = 2;
pub const VK_SAMPLER_CREATE_FLAG_BITS_MAX_ENUM: VkSamplerCreateFlagBits = 2147483647;

pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolCreateFlagBits = VkFlags;
pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlagBits = 1;
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT: VkDescriptorPoolCreateFlagBits = 2;
pub const VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE: VkDescriptorPoolCreateFlagBits = 4;
pub const VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorPoolCreateFlagBits = 2;
pub const VK_DESCRIPTOR_POOL_CREATE_FLAG_BITS_MAX_ENUM: VkDescriptorPoolCreateFlagBits = 2147483647;

pub type VkDescriptorPoolResetFlags = VkFlags;

pub type VkDescriptorSetLayoutCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlagBits = VkFlags;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT: VkDescriptorSetLayoutCreateFlagBits = 2;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR: VkDescriptorSetLayoutCreateFlagBits = 1;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE: VkDescriptorSetLayoutCreateFlagBits = 4;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT: VkDescriptorSetLayoutCreateFlagBits = 2;
pub const VK_DESCRIPTOR_SET_LAYOUT_CREATE_FLAG_BITS_MAX_ENUM: VkDescriptorSetLayoutCreateFlagBits = 2147483647;

pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkAttachmentDescriptionFlagBits = VkFlags;
pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlagBits = 1;
pub const VK_ATTACHMENT_DESCRIPTION_FLAG_BITS_MAX_ENUM: VkAttachmentDescriptionFlagBits = 2147483647;

pub type VkDependencyFlags = VkFlags;
pub type VkDependencyFlagBits = VkFlags;
pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlagBits = 1;
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT: VkDependencyFlagBits = 4;
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT: VkDependencyFlagBits = 2;
pub const VK_DEPENDENCY_VIEW_LOCAL_BIT_KHR: VkDependencyFlagBits = 2;
pub const VK_DEPENDENCY_DEVICE_GROUP_BIT_KHR: VkDependencyFlagBits = 4;
pub const VK_DEPENDENCY_FLAG_BITS_MAX_ENUM: VkDependencyFlagBits = 2147483647;

pub type VkFramebufferCreateFlags = VkFlags;
pub type VkFramebufferCreateFlagBits = VkFlags;
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT: VkFramebufferCreateFlagBits = 1;
pub const VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR: VkFramebufferCreateFlagBits = 1;
pub const VK_FRAMEBUFFER_CREATE_FLAG_BITS_MAX_ENUM: VkFramebufferCreateFlagBits = 2147483647;

pub type VkRenderPassCreateFlags = VkFlags;
pub type VkRenderPassCreateFlagBits = VkFlags;
pub const VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM: VkRenderPassCreateFlagBits = 2;
pub const VK_RENDER_PASS_CREATE_FLAG_BITS_MAX_ENUM: VkRenderPassCreateFlagBits = 2147483647;

pub type VkSubpassDescriptionFlags = VkFlags;
pub type VkSubpassDescriptionFlagBits = VkFlags;
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX: VkSubpassDescriptionFlagBits = 1;
pub const VK_SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX: VkSubpassDescriptionFlagBits = 2;
pub const VK_SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM: VkSubpassDescriptionFlagBits = 4;
pub const VK_SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM: VkSubpassDescriptionFlagBits = 8;
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = 16;
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = 32;
pub const VK_SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM: VkSubpassDescriptionFlagBits = 64;
pub const VK_SUBPASS_DESCRIPTION_FLAG_BITS_MAX_ENUM: VkSubpassDescriptionFlagBits = 2147483647;

pub type VkCommandPoolCreateFlags = VkFlags;
pub type VkCommandPoolCreateFlagBits = VkFlags;
pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlagBits = 1;
pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlagBits = 2;
pub const VK_COMMAND_POOL_CREATE_PROTECTED_BIT: VkCommandPoolCreateFlagBits = 4;
pub const VK_COMMAND_POOL_CREATE_FLAG_BITS_MAX_ENUM: VkCommandPoolCreateFlagBits = 2147483647;

pub type VkCommandPoolResetFlags = VkFlags;
pub type VkCommandPoolResetFlagBits = VkFlags;
pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlagBits = 1;
pub const VK_COMMAND_POOL_RESET_FLAG_BITS_MAX_ENUM: VkCommandPoolResetFlagBits = 2147483647;

pub type VkCommandBufferUsageFlags = VkFlags;
pub type VkCommandBufferUsageFlagBits = VkFlags;
pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlagBits = 1;
pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlagBits = 2;
pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlagBits = 4;
pub const VK_COMMAND_BUFFER_USAGE_FLAG_BITS_MAX_ENUM: VkCommandBufferUsageFlagBits = 2147483647;

pub type VkQueryControlFlags = VkFlags;
pub type VkQueryControlFlagBits = VkFlags;
pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlagBits = 1;
pub const VK_QUERY_CONTROL_FLAG_BITS_MAX_ENUM: VkQueryControlFlagBits = 2147483647;

pub type VkCommandBufferResetFlags = VkFlags;
pub type VkCommandBufferResetFlagBits = VkFlags;
pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlagBits = 1;
pub const VK_COMMAND_BUFFER_RESET_FLAG_BITS_MAX_ENUM: VkCommandBufferResetFlagBits = 2147483647;

pub type VkStencilFaceFlags = VkFlags;
pub type VkStencilFaceFlagBits = VkFlags;
pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlagBits = 1;
pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlagBits = 2;
pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlagBits = 3;
pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlagBits = 3;
pub const VK_STENCIL_FACE_FLAG_BITS_MAX_ENUM: VkStencilFaceFlagBits = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExtent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExtent3D {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkOffset2D {
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkOffset3D {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRect2D {
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBaseInStructure {
	pub sType: VkStructureType,
	pub pNext: *const VkBaseInStructure,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBaseOutStructure {
	pub sType: VkStructureType,
	pub pNext: *mut VkBaseOutStructure,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDispatchIndirectCommand {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrawIndexedIndirectCommand {
	pub indexCount: u32,
	pub instanceCount: u32,
	pub firstIndex: u32,
	pub vertexOffset: i32,
	pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrawIndirectCommand {
	pub vertexCount: u32,
	pub instanceCount: u32,
	pub firstVertex: u32,
	pub firstInstance: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageSubresourceRange {
	pub aspectMask: VkImageAspectFlags,
	pub baseMipLevel: u32,
	pub levelCount: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCacheHeaderVersionOne {
	pub headerSize: u32,
	pub headerVersion: VkPipelineCacheHeaderVersion,
	pub vendorID: u32,
	pub deviceID: u32,
	pub pipelineCacheUUID: [u8; 16],
}

pub type PFN_vkAllocationFunction = unsafe extern "C" fn(pUserData: *mut (), size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut ();

pub type PFN_vkFreeFunction = unsafe extern "C" fn(pUserData: *mut (), pMemory: *mut ());

pub type PFN_vkInternalAllocationNotification = unsafe extern "C" fn(pUserData: *mut (), size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub type PFN_vkInternalFreeNotification = unsafe extern "C" fn(pUserData: *mut (), size: usize, allocationType: VkInternalAllocationType, allocationScope: VkSystemAllocationScope);

pub type PFN_vkReallocationFunction = unsafe extern "C" fn(pUserData: *mut (), pOriginal: *mut (), size: usize, alignment: usize, allocationScope: VkSystemAllocationScope) -> *mut ();

pub type PFN_vkVoidFunction = *const ();

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkAllocationCallbacks {
	pub pUserData: *mut (),
	pub pfnAllocation: PFN_vkAllocationFunction,
	pub pfnReallocation: PFN_vkReallocationFunction,
	pub pfnFree: PFN_vkFreeFunction,
	pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
	pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkApplicationInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pApplicationName: *const i8,
	pub applicationVersion: u32,
	pub pEngineName: *const i8,
	pub engineVersion: u32,
	pub apiVersion: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFormatProperties {
	pub linearTilingFeatures: VkFormatFeatureFlags,
	pub optimalTilingFeatures: VkFormatFeatureFlags,
	pub bufferFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageFormatProperties {
	pub maxExtent: VkExtent3D,
	pub maxMipLevels: u32,
	pub maxArrayLayers: u32,
	pub sampleCounts: VkSampleCountFlags,
	pub maxResourceSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkInstanceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkInstanceCreateFlags,
	pub pApplicationInfo: *const VkApplicationInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const i8,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryHeap {
	pub size: VkDeviceSize,
	pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryType {
	pub propertyFlags: VkMemoryPropertyFlags,
	pub heapIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFeatures {
	pub robustBufferAccess: VkBool32,
	pub fullDrawIndexUint32: VkBool32,
	pub imageCubeArray: VkBool32,
	pub independentBlend: VkBool32,
	pub geometryShader: VkBool32,
	pub tessellationShader: VkBool32,
	pub sampleRateShading: VkBool32,
	pub dualSrcBlend: VkBool32,
	pub logicOp: VkBool32,
	pub multiDrawIndirect: VkBool32,
	pub drawIndirectFirstInstance: VkBool32,
	pub depthClamp: VkBool32,
	pub depthBiasClamp: VkBool32,
	pub fillModeNonSolid: VkBool32,
	pub depthBounds: VkBool32,
	pub wideLines: VkBool32,
	pub largePoints: VkBool32,
	pub alphaToOne: VkBool32,
	pub multiViewport: VkBool32,
	pub samplerAnisotropy: VkBool32,
	pub textureCompressionETC2: VkBool32,
	pub textureCompressionASTC_LDR: VkBool32,
	pub textureCompressionBC: VkBool32,
	pub occlusionQueryPrecise: VkBool32,
	pub pipelineStatisticsQuery: VkBool32,
	pub vertexPipelineStoresAndAtomics: VkBool32,
	pub fragmentStoresAndAtomics: VkBool32,
	pub shaderTessellationAndGeometryPointSize: VkBool32,
	pub shaderImageGatherExtended: VkBool32,
	pub shaderStorageImageExtendedFormats: VkBool32,
	pub shaderStorageImageMultisample: VkBool32,
	pub shaderStorageImageReadWithoutFormat: VkBool32,
	pub shaderStorageImageWriteWithoutFormat: VkBool32,
	pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
	pub shaderSampledImageArrayDynamicIndexing: VkBool32,
	pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageImageArrayDynamicIndexing: VkBool32,
	pub shaderClipDistance: VkBool32,
	pub shaderCullDistance: VkBool32,
	pub shaderFloat64: VkBool32,
	pub shaderInt64: VkBool32,
	pub shaderInt16: VkBool32,
	pub shaderResourceResidency: VkBool32,
	pub shaderResourceMinLod: VkBool32,
	pub sparseBinding: VkBool32,
	pub sparseResidencyBuffer: VkBool32,
	pub sparseResidencyImage2D: VkBool32,
	pub sparseResidencyImage3D: VkBool32,
	pub sparseResidency2Samples: VkBool32,
	pub sparseResidency4Samples: VkBool32,
	pub sparseResidency8Samples: VkBool32,
	pub sparseResidency16Samples: VkBool32,
	pub sparseResidencyAliased: VkBool32,
	pub variableMultisampleRate: VkBool32,
	pub inheritedQueries: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceLimits {
	pub maxImageDimension1D: u32,
	pub maxImageDimension2D: u32,
	pub maxImageDimension3D: u32,
	pub maxImageDimensionCube: u32,
	pub maxImageArrayLayers: u32,
	pub maxTexelBufferElements: u32,
	pub maxUniformBufferRange: u32,
	pub maxStorageBufferRange: u32,
	pub maxPushConstantsSize: u32,
	pub maxMemoryAllocationCount: u32,
	pub maxSamplerAllocationCount: u32,
	pub bufferImageGranularity: VkDeviceSize,
	pub sparseAddressSpaceSize: VkDeviceSize,
	pub maxBoundDescriptorSets: u32,
	pub maxPerStageDescriptorSamplers: u32,
	pub maxPerStageDescriptorUniformBuffers: u32,
	pub maxPerStageDescriptorStorageBuffers: u32,
	pub maxPerStageDescriptorSampledImages: u32,
	pub maxPerStageDescriptorStorageImages: u32,
	pub maxPerStageDescriptorInputAttachments: u32,
	pub maxPerStageResources: u32,
	pub maxDescriptorSetSamplers: u32,
	pub maxDescriptorSetUniformBuffers: u32,
	pub maxDescriptorSetUniformBuffersDynamic: u32,
	pub maxDescriptorSetStorageBuffers: u32,
	pub maxDescriptorSetStorageBuffersDynamic: u32,
	pub maxDescriptorSetSampledImages: u32,
	pub maxDescriptorSetStorageImages: u32,
	pub maxDescriptorSetInputAttachments: u32,
	pub maxVertexInputAttributes: u32,
	pub maxVertexInputBindings: u32,
	pub maxVertexInputAttributeOffset: u32,
	pub maxVertexInputBindingStride: u32,
	pub maxVertexOutputComponents: u32,
	pub maxTessellationGenerationLevel: u32,
	pub maxTessellationPatchSize: u32,
	pub maxTessellationControlPerVertexInputComponents: u32,
	pub maxTessellationControlPerVertexOutputComponents: u32,
	pub maxTessellationControlPerPatchOutputComponents: u32,
	pub maxTessellationControlTotalOutputComponents: u32,
	pub maxTessellationEvaluationInputComponents: u32,
	pub maxTessellationEvaluationOutputComponents: u32,
	pub maxGeometryShaderInvocations: u32,
	pub maxGeometryInputComponents: u32,
	pub maxGeometryOutputComponents: u32,
	pub maxGeometryOutputVertices: u32,
	pub maxGeometryTotalOutputComponents: u32,
	pub maxFragmentInputComponents: u32,
	pub maxFragmentOutputAttachments: u32,
	pub maxFragmentDualSrcAttachments: u32,
	pub maxFragmentCombinedOutputResources: u32,
	pub maxComputeSharedMemorySize: u32,
	pub maxComputeWorkGroupCount: [u32; 3],
	pub maxComputeWorkGroupInvocations: u32,
	pub maxComputeWorkGroupSize: [u32; 3],
	pub subPixelPrecisionBits: u32,
	pub subTexelPrecisionBits: u32,
	pub mipmapPrecisionBits: u32,
	pub maxDrawIndexedIndexValue: u32,
	pub maxDrawIndirectCount: u32,
	pub maxSamplerLodBias: f32,
	pub maxSamplerAnisotropy: f32,
	pub maxViewports: u32,
	pub maxViewportDimensions: [u32; 2],
	pub viewportBoundsRange: [f32; 2],
	pub viewportSubPixelBits: u32,
	pub minMemoryMapAlignment: usize,
	pub minTexelBufferOffsetAlignment: VkDeviceSize,
	pub minUniformBufferOffsetAlignment: VkDeviceSize,
	pub minStorageBufferOffsetAlignment: VkDeviceSize,
	pub minTexelOffset: i32,
	pub maxTexelOffset: u32,
	pub minTexelGatherOffset: i32,
	pub maxTexelGatherOffset: u32,
	pub minInterpolationOffset: f32,
	pub maxInterpolationOffset: f32,
	pub subPixelInterpolationOffsetBits: u32,
	pub maxFramebufferWidth: u32,
	pub maxFramebufferHeight: u32,
	pub maxFramebufferLayers: u32,
	pub framebufferColorSampleCounts: VkSampleCountFlags,
	pub framebufferDepthSampleCounts: VkSampleCountFlags,
	pub framebufferStencilSampleCounts: VkSampleCountFlags,
	pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
	pub maxColorAttachments: u32,
	pub sampledImageColorSampleCounts: VkSampleCountFlags,
	pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
	pub sampledImageDepthSampleCounts: VkSampleCountFlags,
	pub sampledImageStencilSampleCounts: VkSampleCountFlags,
	pub storageImageSampleCounts: VkSampleCountFlags,
	pub maxSampleMaskWords: u32,
	pub timestampComputeAndGraphics: VkBool32,
	pub timestampPeriod: f32,
	pub maxClipDistances: u32,
	pub maxCullDistances: u32,
	pub maxCombinedClipAndCullDistances: u32,
	pub discreteQueuePriorities: u32,
	pub pointSizeRange: [f32; 2],
	pub lineWidthRange: [f32; 2],
	pub pointSizeGranularity: f32,
	pub lineWidthGranularity: f32,
	pub strictLines: VkBool32,
	pub standardSampleLocations: VkBool32,
	pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
	pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
	pub nonCoherentAtomSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMemoryProperties {
	pub memoryTypeCount: u32,
	pub memoryTypes: [VkMemoryType; 32],
	pub memoryHeapCount: u32,
	pub memoryHeaps: [VkMemoryHeap; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSparseProperties {
	pub residencyStandard2DBlockShape: VkBool32,
	pub residencyStandard2DMultisampleBlockShape: VkBool32,
	pub residencyStandard3DBlockShape: VkBool32,
	pub residencyAlignedMipSize: VkBool32,
	pub residencyNonResidentStrict: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProperties {
	pub apiVersion: u32,
	pub driverVersion: u32,
	pub vendorID: u32,
	pub deviceID: u32,
	pub deviceType: VkPhysicalDeviceType,
	pub deviceName: [i8; 256],
	pub pipelineCacheUUID: [u8; 16],
	pub limits: VkPhysicalDeviceLimits,
	pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueueFamilyProperties {
	pub queueFlags: VkQueueFlags,
	pub queueCount: u32,
	pub timestampValidBits: u32,
	pub minImageTransferGranularity: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceQueueCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueCount: u32,
	pub pQueuePriorities: *const f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDeviceCreateFlags,
	pub queueCreateInfoCount: u32,
	pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const i8,
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const i8,
	pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExtensionProperties {
	pub extensionName: [i8; 256],
	pub specVersion: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkLayerProperties {
	pub layerName: [i8; 256],
	pub specVersion: u32,
	pub implementationVersion: u32,
	pub description: [i8; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub pWaitDstStageMask: *const VkPipelineStageFlags,
	pub commandBufferCount: u32,
	pub pCommandBuffers: *const VkCommandBuffer,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMappedMemoryRange {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub memory: VkDeviceMemory,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub allocationSize: VkDeviceSize,
	pub memoryTypeIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryRequirements {
	pub size: VkDeviceSize,
	pub alignment: VkDeviceSize,
	pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseMemoryBind {
	pub resourceOffset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseBufferMemoryBindInfo {
	pub buffer: VkBuffer,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageSubresource {
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub arrayLayer: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageMemoryBind {
	pub subresource: VkImageSubresource,
	pub offset: VkOffset3D,
	pub extent: VkExtent3D,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageMemoryBindInfo {
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseImageMemoryBind,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindSparseInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub bufferBindCount: u32,
	pub pBufferBinds: *const VkSparseBufferMemoryBindInfo,
	pub imageOpaqueBindCount: u32,
	pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo,
	pub imageBindCount: u32,
	pub pImageBinds: *const VkSparseImageMemoryBindInfo,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageFormatProperties {
	pub aspectMask: VkImageAspectFlags,
	pub imageGranularity: VkExtent3D,
	pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageMemoryRequirements {
	pub formatProperties: VkSparseImageFormatProperties,
	pub imageMipTailFirstLod: u32,
	pub imageMipTailSize: VkDeviceSize,
	pub imageMipTailOffset: VkDeviceSize,
	pub imageMipTailStride: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFenceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkEventCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkEventCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueryPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkQueryPoolCreateFlags,
	pub queryType: VkQueryType,
	pub queryCount: u32,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkBufferCreateFlags,
	pub size: VkDeviceSize,
	pub usage: VkBufferUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferViewCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkBufferViewCreateFlags,
	pub buffer: VkBuffer,
	pub format: VkFormat,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkImageCreateFlags,
	pub imageType: VkImageType,
	pub format: VkFormat,
	pub extent: VkExtent3D,
	pub mipLevels: u32,
	pub arrayLayers: u32,
	pub samples: VkSampleCountFlagBits,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
	pub initialLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubresourceLayout {
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub rowPitch: VkDeviceSize,
	pub arrayPitch: VkDeviceSize,
	pub depthPitch: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkComponentMapping {
	pub r: VkComponentSwizzle,
	pub g: VkComponentSwizzle,
	pub b: VkComponentSwizzle,
	pub a: VkComponentSwizzle,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkImageViewCreateFlags,
	pub image: VkImage,
	pub viewType: VkImageViewType,
	pub format: VkFormat,
	pub components: VkComponentMapping,
	pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShaderModuleCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkShaderModuleCreateFlags,
	pub codeSize: usize,
	pub pCode: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCacheCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCacheCreateFlags,
	pub initialDataSize: usize,
	pub pInitialData: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSpecializationMapEntry {
	pub constantID: u32,
	pub offset: u32,
	pub size: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSpecializationInfo {
	pub mapEntryCount: u32,
	pub pMapEntries: *const VkSpecializationMapEntry,
	pub dataSize: usize,
	pub pData: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineShaderStageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineShaderStageCreateFlags,
	pub stage: VkShaderStageFlagBits,
	pub module: VkShaderModule,
	pub pName: *const i8,
	pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkComputePipelineCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCreateFlags,
	pub stage: VkPipelineShaderStageCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkVertexInputBindingDescription {
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkVertexInputAttributeDescription {
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineVertexInputStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineVertexInputStateCreateFlags,
	pub vertexBindingDescriptionCount: u32,
	pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription,
	pub vertexAttributeDescriptionCount: u32,
	pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineInputAssemblyStateCreateFlags,
	pub topology: VkPrimitiveTopology,
	pub primitiveRestartEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineTessellationStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineTessellationStateCreateFlags,
	pub patchControlPoints: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkViewport {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub minDepth: f32,
	pub maxDepth: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineViewportStateCreateFlags,
	pub viewportCount: u32,
	pub pViewports: *const VkViewport,
	pub scissorCount: u32,
	pub pScissors: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineRasterizationStateCreateFlags,
	pub depthClampEnable: VkBool32,
	pub rasterizerDiscardEnable: VkBool32,
	pub polygonMode: VkPolygonMode,
	pub cullMode: VkCullModeFlags,
	pub frontFace: VkFrontFace,
	pub depthBiasEnable: VkBool32,
	pub depthBiasConstantFactor: f32,
	pub depthBiasClamp: f32,
	pub depthBiasSlopeFactor: f32,
	pub lineWidth: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineMultisampleStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineMultisampleStateCreateFlags,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub sampleShadingEnable: VkBool32,
	pub minSampleShading: f32,
	pub pSampleMask: *const VkSampleMask,
	pub alphaToCoverageEnable: VkBool32,
	pub alphaToOneEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkStencilOpState {
	pub failOp: VkStencilOp,
	pub passOp: VkStencilOp,
	pub depthFailOp: VkStencilOp,
	pub compareOp: VkCompareOp,
	pub compareMask: u32,
	pub writeMask: u32,
	pub reference: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineDepthStencilStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineDepthStencilStateCreateFlags,
	pub depthTestEnable: VkBool32,
	pub depthWriteEnable: VkBool32,
	pub depthCompareOp: VkCompareOp,
	pub depthBoundsTestEnable: VkBool32,
	pub stencilTestEnable: VkBool32,
	pub front: VkStencilOpState,
	pub back: VkStencilOpState,
	pub minDepthBounds: f32,
	pub maxDepthBounds: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineColorBlendAttachmentState {
	pub blendEnable: VkBool32,
	pub srcColorBlendFactor: VkBlendFactor,
	pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor,
	pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp,
	pub colorWriteMask: VkColorComponentFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineColorBlendStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineColorBlendStateCreateFlags,
	pub logicOpEnable: VkBool32,
	pub logicOp: VkLogicOp,
	pub attachmentCount: u32,
	pub pAttachments: *const VkPipelineColorBlendAttachmentState,
	pub blendConstants: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineDynamicStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineDynamicStateCreateFlags,
	pub dynamicStateCount: u32,
	pub pDynamicStates: *const VkDynamicState,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGraphicsPipelineCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
	pub pViewportState: *const VkPipelineViewportStateCreateInfo,
	pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
	pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
	pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
	pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPushConstantRange {
	pub stageFlags: VkShaderStageFlags,
	pub offset: u32,
	pub size: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineLayoutCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineLayoutCreateFlags,
	pub setLayoutCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout,
	pub pushConstantRangeCount: u32,
	pub pPushConstantRanges: *const VkPushConstantRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSamplerCreateFlags,
	pub magFilter: VkFilter,
	pub minFilter: VkFilter,
	pub mipmapMode: VkSamplerMipmapMode,
	pub addressModeU: VkSamplerAddressMode,
	pub addressModeV: VkSamplerAddressMode,
	pub addressModeW: VkSamplerAddressMode,
	pub mipLodBias: f32,
	pub anisotropyEnable: VkBool32,
	pub maxAnisotropy: f32,
	pub compareEnable: VkBool32,
	pub compareOp: VkCompareOp,
	pub minLod: f32,
	pub maxLod: f32,
	pub borderColor: VkBorderColor,
	pub unnormalizedCoordinates: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyDescriptorSet {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcSet: VkDescriptorSet,
	pub srcBinding: u32,
	pub srcArrayElement: u32,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorBufferInfo {
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorImageInfo {
	pub sampler: VkSampler,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorPoolSize {
	pub type_: VkDescriptorType,
	pub descriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDescriptorPoolCreateFlags,
	pub maxSets: u32,
	pub poolSizeCount: u32,
	pub pPoolSizes: *const VkDescriptorPoolSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub descriptorPool: VkDescriptorPool,
	pub descriptorSetCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutBinding {
	pub binding: u32,
	pub descriptorType: VkDescriptorType,
	pub descriptorCount: u32,
	pub stageFlags: VkShaderStageFlags,
	pub pImmutableSamplers: *const VkSampler,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDescriptorSetLayoutCreateFlags,
	pub bindingCount: u32,
	pub pBindings: *const VkDescriptorSetLayoutBinding,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkWriteDescriptorSet {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub pImageInfo: *const VkDescriptorImageInfo,
	pub pBufferInfo: *const VkDescriptorBufferInfo,
	pub pTexelBufferView: *const VkBufferView,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentDescription {
	pub flags: VkAttachmentDescriptionFlags,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentReference {
	pub attachment: u32,
	pub layout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFramebufferCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkFramebufferCreateFlags,
	pub renderPass: VkRenderPass,
	pub attachmentCount: u32,
	pub pAttachments: *const VkImageView,
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassDescription {
	pub flags: VkSubpassDescriptionFlags,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: *const VkAttachmentReference,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkAttachmentReference,
	pub pResolveAttachments: *const VkAttachmentReference,
	pub pDepthStencilAttachment: *const VkAttachmentReference,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassDependency {
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags,
	pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32,
	pub pAttachments: *const VkAttachmentDescription,
	pub subpassCount: u32,
	pub pSubpasses: *const VkSubpassDescription,
	pub dependencyCount: u32,
	pub pDependencies: *const VkSubpassDependency,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkCommandPoolCreateFlags,
	pub queueFamilyIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub commandPool: VkCommandPool,
	pub level: VkCommandBufferLevel,
	pub commandBufferCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferInheritanceInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusionQueryEnable: VkBool32,
	pub queryFlags: VkQueryControlFlags,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkCommandBufferUsageFlags,
	pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferCopy {
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageSubresourceLayers {
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferImageCopy {
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearColorValue {
	pub float32: [f32; 4],
	pub int32: [i32; 4],
	pub uint32: [u32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkClearDepthStencilValue {
	pub depth: f32,
	pub stencil: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkClearValue {
	pub color: VkClearColorValue,
	pub depthStencil: VkClearDepthStencilValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkClearAttachment {
	pub aspectMask: VkImageAspectFlags,
	pub colorAttachment: u32,
	pub clearValue: VkClearValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkClearRect {
	pub rect: VkRect2D,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageBlit {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageCopy {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageResolve {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub renderPass: VkRenderPass,
	pub framebuffer: VkFramebuffer,
	pub renderArea: VkRect2D,
	pub clearValueCount: u32,
	pub pClearValues: *const VkClearValue,
}

pub type PFN_vkCreateInstance = unsafe extern "C" fn(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;

pub type PFN_vkDestroyInstance = unsafe extern "C" fn(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "C" fn(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);

pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);

pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type_: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	pImageFormatProperties: *mut VkImageFormatProperties,
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);

pub type PFN_vkGetInstanceProcAddr = unsafe extern "C" fn(instance: VkInstance, pName: *const i8) -> PFN_vkVoidFunction;

pub type PFN_vkGetDeviceProcAddr = unsafe extern "C" fn(device: VkDevice, pName: *const i8) -> PFN_vkVoidFunction;

pub type PFN_vkCreateDevice =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;

pub type PFN_vkDestroyDevice = unsafe extern "C" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "C" fn(pLayerName: *const i8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

pub type PFN_vkEnumerateDeviceExtensionProperties =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pLayerName: *const i8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "C" fn(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

pub type PFN_vkGetDeviceQueue = unsafe extern "C" fn(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);

pub type PFN_vkQueueSubmit = unsafe extern "C" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;

pub type PFN_vkQueueWaitIdle = unsafe extern "C" fn(queue: VkQueue) -> VkResult;

pub type PFN_vkDeviceWaitIdle = unsafe extern "C" fn(device: VkDevice) -> VkResult;

pub type PFN_vkAllocateMemory = unsafe extern "C" fn(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;

pub type PFN_vkFreeMemory = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkMapMemory = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut ()) -> VkResult;

pub type PFN_vkUnmapMemory = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory);

pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "C" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "C" fn(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);

pub type PFN_vkBindBufferMemory = unsafe extern "C" fn(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

pub type PFN_vkBindImageMemory = unsafe extern "C" fn(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "C" fn(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);

pub type PFN_vkGetImageMemoryRequirements = unsafe extern "C" fn(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);

pub type PFN_vkGetImageSparseMemoryRequirements =
	unsafe extern "C" fn(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type_: VkImageType,
	samples: VkSampleCountFlagBits,
	usage: VkImageUsageFlags,
	tiling: VkImageTiling,
	pPropertyCount: *mut u32,
	pProperties: *mut VkSparseImageFormatProperties,
);

pub type PFN_vkQueueBindSparse = unsafe extern "C" fn(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;

pub type PFN_vkCreateFence = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

pub type PFN_vkDestroyFence = unsafe extern "C" fn(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkResetFences = unsafe extern "C" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;

pub type PFN_vkGetFenceStatus = unsafe extern "C" fn(device: VkDevice, fence: VkFence) -> VkResult;

pub type PFN_vkWaitForFences = unsafe extern "C" fn(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;

pub type PFN_vkCreateSemaphore = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;

pub type PFN_vkDestroySemaphore = unsafe extern "C" fn(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateEvent = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;

pub type PFN_vkDestroyEvent = unsafe extern "C" fn(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetEventStatus = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub type PFN_vkSetEvent = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub type PFN_vkResetEvent = unsafe extern "C" fn(device: VkDevice, event: VkEvent) -> VkResult;

pub type PFN_vkCreateQueryPool = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;

pub type PFN_vkDestroyQueryPool = unsafe extern "C" fn(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetQueryPoolResults =
	unsafe extern "C" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut (), stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;

pub type PFN_vkCreateBuffer = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;

pub type PFN_vkDestroyBuffer = unsafe extern "C" fn(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateBufferView = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;

pub type PFN_vkDestroyBufferView = unsafe extern "C" fn(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateImage = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;

pub type PFN_vkDestroyImage = unsafe extern "C" fn(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetImageSubresourceLayout = unsafe extern "C" fn(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);

pub type PFN_vkCreateImageView = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;

pub type PFN_vkDestroyImageView = unsafe extern "C" fn(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateShaderModule =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;

pub type PFN_vkDestroyShaderModule = unsafe extern "C" fn(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreatePipelineCache =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;

pub type PFN_vkDestroyPipelineCache = unsafe extern "C" fn(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetPipelineCacheData = unsafe extern "C" fn(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut ()) -> VkResult;

pub type PFN_vkMergePipelineCaches = unsafe extern "C" fn(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;

pub type PFN_vkCreateGraphicsPipelines = unsafe extern "C" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkGraphicsPipelineCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *mut VkPipeline,
) -> VkResult;

pub type PFN_vkCreateComputePipelines = unsafe extern "C" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkComputePipelineCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *mut VkPipeline,
) -> VkResult;

pub type PFN_vkDestroyPipeline = unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreatePipelineLayout =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;

pub type PFN_vkDestroyPipelineLayout = unsafe extern "C" fn(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateSampler = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;

pub type PFN_vkDestroySampler = unsafe extern "C" fn(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateDescriptorSetLayout =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;

pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "C" fn(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateDescriptorPool =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;

pub type PFN_vkDestroyDescriptorPool = unsafe extern "C" fn(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkResetDescriptorPool = unsafe extern "C" fn(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;

pub type PFN_vkAllocateDescriptorSets = unsafe extern "C" fn(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;

pub type PFN_vkFreeDescriptorSets = unsafe extern "C" fn(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;

pub type PFN_vkUpdateDescriptorSets =
	unsafe extern "C" fn(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);

pub type PFN_vkCreateFramebuffer =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;

pub type PFN_vkDestroyFramebuffer = unsafe extern "C" fn(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateRenderPass =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

pub type PFN_vkDestroyRenderPass = unsafe extern "C" fn(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetRenderAreaGranularity = unsafe extern "C" fn(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);

pub type PFN_vkCreateCommandPool =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;

pub type PFN_vkDestroyCommandPool = unsafe extern "C" fn(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkResetCommandPool = unsafe extern "C" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;

pub type PFN_vkAllocateCommandBuffers = unsafe extern "C" fn(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

pub type PFN_vkFreeCommandBuffers = unsafe extern "C" fn(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

pub type PFN_vkBeginCommandBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;

pub type PFN_vkEndCommandBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer) -> VkResult;

pub type PFN_vkResetCommandBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

pub type PFN_vkCmdBindPipeline = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);

pub type PFN_vkCmdSetViewport = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);

pub type PFN_vkCmdSetScissor = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);

pub type PFN_vkCmdSetLineWidth = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, lineWidth: f32);

pub type PFN_vkCmdSetDepthBias = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);

pub type PFN_vkCmdSetBlendConstants = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, blendConstants: *const f32);

pub type PFN_vkCmdSetDepthBounds = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);

pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);

pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);

pub type PFN_vkCmdSetStencilReference = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);

pub type PFN_vkCmdBindDescriptorSets = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	firstSet: u32,
	descriptorSetCount: u32,
	pDescriptorSets: *const VkDescriptorSet,
	dynamicOffsetCount: u32,
	pDynamicOffsets: *const u32,
);

pub type PFN_vkCmdBindIndexBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);

pub type PFN_vkCmdBindVertexBuffers = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);

pub type PFN_vkCmdDraw = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);

pub type PFN_vkCmdDrawIndexed = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);

pub type PFN_vkCmdDrawIndirect = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

pub type PFN_vkCmdDispatch = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

pub type PFN_vkCmdDispatchIndirect = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);

pub type PFN_vkCmdCopyBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);

pub type PFN_vkCmdCopyImage = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageCopy,
);

pub type PFN_vkCmdBlitImage = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageBlit,
	filter: VkFilter,
);

pub type PFN_vkCmdCopyBufferToImage =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);

pub type PFN_vkCmdCopyImageToBuffer =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

pub type PFN_vkCmdUpdateBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const ());

pub type PFN_vkCmdFillBuffer = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);

pub type PFN_vkCmdClearColorImage =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	image: VkImage,
	imageLayout: VkImageLayout,
	pDepthStencil: *const VkClearDepthStencilValue,
	rangeCount: u32,
	pRanges: *const VkImageSubresourceRange,
);

pub type PFN_vkCmdClearAttachments = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);

pub type PFN_vkCmdResolveImage = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	srcImage: VkImage,
	srcImageLayout: VkImageLayout,
	dstImage: VkImage,
	dstImageLayout: VkImageLayout,
	regionCount: u32,
	pRegions: *const VkImageResolve,
);

pub type PFN_vkCmdSetEvent = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

pub type PFN_vkCmdResetEvent = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

pub type PFN_vkCmdWaitEvents = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	eventCount: u32,
	pEvents: *const VkEvent,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *const VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *const VkImageMemoryBarrier,
);

pub type PFN_vkCmdPipelineBarrier = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	srcStageMask: VkPipelineStageFlags,
	dstStageMask: VkPipelineStageFlags,
	dependencyFlags: VkDependencyFlags,
	memoryBarrierCount: u32,
	pMemoryBarriers: *const VkMemoryBarrier,
	bufferMemoryBarrierCount: u32,
	pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
	imageMemoryBarrierCount: u32,
	pImageMemoryBarriers: *const VkImageMemoryBarrier,
);

pub type PFN_vkCmdBeginQuery = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);

pub type PFN_vkCmdEndQuery = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);

pub type PFN_vkCmdResetQueryPool = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

pub type PFN_vkCmdWriteTimestamp = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);

pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	queryPool: VkQueryPool,
	firstQuery: u32,
	queryCount: u32,
	dstBuffer: VkBuffer,
	dstOffset: VkDeviceSize,
	stride: VkDeviceSize,
	flags: VkQueryResultFlags,
);

pub type PFN_vkCmdPushConstants = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const ());

pub type PFN_vkCmdBeginRenderPass = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);

pub type PFN_vkCmdNextSubpass = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);

pub type PFN_vkCmdEndRenderPass = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

pub type PFN_vkCmdExecuteCommands = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;

	pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);

	pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

	pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);

	pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);

	pub fn vkGetPhysicalDeviceImageFormatProperties(
		physicalDevice: VkPhysicalDevice,
		format: VkFormat,
		type_: VkImageType,
		tiling: VkImageTiling,
		usage: VkImageUsageFlags,
		flags: VkImageCreateFlags,
		pImageFormatProperties: *mut VkImageFormatProperties,
	) -> VkResult;

	pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);

	pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);

	pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);

	pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const i8) -> PFN_vkVoidFunction;

	pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const i8) -> PFN_vkVoidFunction;

	pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;

	pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);

	pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const i8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

	pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const i8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;

	pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

	pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;

	pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);

	pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;

	pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;

	pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;

	pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;

	pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);

	pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut ()) -> VkResult;

	pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);

	pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

	pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;

	pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);

	pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

	pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;

	pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);

	pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);

	pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);

	pub fn vkGetPhysicalDeviceSparseImageFormatProperties(
		physicalDevice: VkPhysicalDevice,
		format: VkFormat,
		type_: VkImageType,
		samples: VkSampleCountFlagBits,
		usage: VkImageUsageFlags,
		tiling: VkImageTiling,
		pPropertyCount: *mut u32,
		pProperties: *mut VkSparseImageFormatProperties,
	);

	pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;

	pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

	pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);

	pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;

	pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;

	pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;

	pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;

	pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;

	pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;

	pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;

	pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;

	pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;

	pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetQueryPoolResults(
		device: VkDevice,
		queryPool: VkQueryPool,
		firstQuery: u32,
		queryCount: u32,
		dataSize: usize,
		pData: *mut (),
		stride: VkDeviceSize,
		flags: VkQueryResultFlags,
	) -> VkResult;

	pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;

	pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;

	pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;

	pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);

	pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;

	pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;

	pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;

	pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut ()) -> VkResult;

	pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;

	pub fn vkCreateGraphicsPipelines(
		device: VkDevice,
		pipelineCache: VkPipelineCache,
		createInfoCount: u32,
		pCreateInfos: *const VkGraphicsPipelineCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPipelines: *mut VkPipeline,
	) -> VkResult;

	pub fn vkCreateComputePipelines(
		device: VkDevice,
		pipelineCache: VkPipelineCache,
		createInfoCount: u32,
		pCreateInfos: *const VkComputePipelineCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPipelines: *mut VkPipeline,
	) -> VkResult;

	pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;

	pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;

	pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateDescriptorSetLayout(
		device: VkDevice,
		pCreateInfo: *const VkDescriptorSetLayoutCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pSetLayout: *mut VkDescriptorSetLayout,
	) -> VkResult;

	pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;

	pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);

	pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;

	pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;

	pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;

	pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);

	pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;

	pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

	pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);

	pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;

	pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);

	pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;

	pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;

	pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

	pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;

	pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;

	pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;

	pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);

	pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);

	pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);

	pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32);

	pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);

	pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: *const f32);

	pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);

	pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);

	pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);

	pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);

	pub fn vkCmdBindDescriptorSets(
		commandBuffer: VkCommandBuffer,
		pipelineBindPoint: VkPipelineBindPoint,
		layout: VkPipelineLayout,
		firstSet: u32,
		descriptorSetCount: u32,
		pDescriptorSets: *const VkDescriptorSet,
		dynamicOffsetCount: u32,
		pDynamicOffsets: *const u32,
	);

	pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);

	pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);

	pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);

	pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);

	pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

	pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

	pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

	pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);

	pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);

	pub fn vkCmdCopyImage(
		commandBuffer: VkCommandBuffer,
		srcImage: VkImage,
		srcImageLayout: VkImageLayout,
		dstImage: VkImage,
		dstImageLayout: VkImageLayout,
		regionCount: u32,
		pRegions: *const VkImageCopy,
	);

	pub fn vkCmdBlitImage(
		commandBuffer: VkCommandBuffer,
		srcImage: VkImage,
		srcImageLayout: VkImageLayout,
		dstImage: VkImage,
		dstImageLayout: VkImageLayout,
		regionCount: u32,
		pRegions: *const VkImageBlit,
		filter: VkFilter,
	);

	pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);

	pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);

	pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const ());

	pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);

	pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);

	pub fn vkCmdClearDepthStencilImage(
		commandBuffer: VkCommandBuffer,
		image: VkImage,
		imageLayout: VkImageLayout,
		pDepthStencil: *const VkClearDepthStencilValue,
		rangeCount: u32,
		pRanges: *const VkImageSubresourceRange,
	);

	pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);

	pub fn vkCmdResolveImage(
		commandBuffer: VkCommandBuffer,
		srcImage: VkImage,
		srcImageLayout: VkImageLayout,
		dstImage: VkImage,
		dstImageLayout: VkImageLayout,
		regionCount: u32,
		pRegions: *const VkImageResolve,
	);

	pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

	pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);

	pub fn vkCmdWaitEvents(
		commandBuffer: VkCommandBuffer,
		eventCount: u32,
		pEvents: *const VkEvent,
		srcStageMask: VkPipelineStageFlags,
		dstStageMask: VkPipelineStageFlags,
		memoryBarrierCount: u32,
		pMemoryBarriers: *const VkMemoryBarrier,
		bufferMemoryBarrierCount: u32,
		pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
		imageMemoryBarrierCount: u32,
		pImageMemoryBarriers: *const VkImageMemoryBarrier,
	);

	pub fn vkCmdPipelineBarrier(
		commandBuffer: VkCommandBuffer,
		srcStageMask: VkPipelineStageFlags,
		dstStageMask: VkPipelineStageFlags,
		dependencyFlags: VkDependencyFlags,
		memoryBarrierCount: u32,
		pMemoryBarriers: *const VkMemoryBarrier,
		bufferMemoryBarrierCount: u32,
		pBufferMemoryBarriers: *const VkBufferMemoryBarrier,
		imageMemoryBarrierCount: u32,
		pImageMemoryBarriers: *const VkImageMemoryBarrier,
	);

	pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);

	pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);

	pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

	pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);

	pub fn vkCmdCopyQueryPoolResults(
		commandBuffer: VkCommandBuffer,
		queryPool: VkQueryPool,
		firstQuery: u32,
		queryCount: u32,
		dstBuffer: VkBuffer,
		dstOffset: VkDeviceSize,
		stride: VkDeviceSize,
		flags: VkQueryResultFlags,
	);

	pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const ());

	pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);

	pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);

	pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);

	pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerYcbcrConversion_T {
	_unused: [u8; 0],
}

pub type VkSamplerYcbcrConversion = *mut VkSamplerYcbcrConversion_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorUpdateTemplate_T {
	_unused: [u8; 0],
}

pub type VkDescriptorUpdateTemplate = *mut VkDescriptorUpdateTemplate_T;

pub type VkPointClippingBehavior = u32;
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: VkPointClippingBehavior = 0;
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: VkPointClippingBehavior = 1;
pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES_KHR: VkPointClippingBehavior = 0;
pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY_KHR: VkPointClippingBehavior = 1;
pub const VK_POINT_CLIPPING_BEHAVIOR_MAX_ENUM: VkPointClippingBehavior = 2147483647;

pub type VkTessellationDomainOrigin = u32;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: VkTessellationDomainOrigin = 0;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: VkTessellationDomainOrigin = 1;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT_KHR: VkTessellationDomainOrigin = 0;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT_KHR: VkTessellationDomainOrigin = 1;
pub const VK_TESSELLATION_DOMAIN_ORIGIN_MAX_ENUM: VkTessellationDomainOrigin = 2147483647;

pub type VkSamplerYcbcrModelConversion = u32;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: VkSamplerYcbcrModelConversion = 0;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: VkSamplerYcbcrModelConversion = 1;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: VkSamplerYcbcrModelConversion = 2;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: VkSamplerYcbcrModelConversion = 3;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: VkSamplerYcbcrModelConversion = 4;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR: VkSamplerYcbcrModelConversion = 0;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR: VkSamplerYcbcrModelConversion = 1;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR: VkSamplerYcbcrModelConversion = 2;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR: VkSamplerYcbcrModelConversion = 3;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR: VkSamplerYcbcrModelConversion = 4;
pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_MAX_ENUM: VkSamplerYcbcrModelConversion = 2147483647;

pub type VkSamplerYcbcrRange = u32;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: VkSamplerYcbcrRange = 0;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: VkSamplerYcbcrRange = 1;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR: VkSamplerYcbcrRange = 0;
pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR: VkSamplerYcbcrRange = 1;
pub const VK_SAMPLER_YCBCR_RANGE_MAX_ENUM: VkSamplerYcbcrRange = 2147483647;

pub type VkChromaLocation = u32;
pub const VK_CHROMA_LOCATION_COSITED_EVEN: VkChromaLocation = 0;
pub const VK_CHROMA_LOCATION_MIDPOINT: VkChromaLocation = 1;
pub const VK_CHROMA_LOCATION_COSITED_EVEN_KHR: VkChromaLocation = 0;
pub const VK_CHROMA_LOCATION_MIDPOINT_KHR: VkChromaLocation = 1;
pub const VK_CHROMA_LOCATION_MAX_ENUM: VkChromaLocation = 2147483647;

pub type VkDescriptorUpdateTemplateType = u32;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: VkDescriptorUpdateTemplateType = 0;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR: VkDescriptorUpdateTemplateType = 1;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR: VkDescriptorUpdateTemplateType = 0;
pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_MAX_ENUM: VkDescriptorUpdateTemplateType = 2147483647;

pub type VkSubgroupFeatureFlags = VkFlags;
pub type VkSubgroupFeatureFlagBits = VkFlags;
pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlagBits = 1;
pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlagBits = 2;
pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlagBits = 4;
pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlagBits = 8;
pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlagBits = 16;
pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlagBits = 32;
pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlagBits = 64;
pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlagBits = 128;
pub const VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV: VkSubgroupFeatureFlagBits = 256;
pub const VK_SUBGROUP_FEATURE_FLAG_BITS_MAX_ENUM: VkSubgroupFeatureFlagBits = 2147483647;

pub type VkPeerMemoryFeatureFlags = VkFlags;
pub type VkPeerMemoryFeatureFlagBits = VkFlags;
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlagBits = 1;
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlagBits = 2;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlagBits = 4;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlagBits = 8;
pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits = 1;
pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits = 2;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT_KHR: VkPeerMemoryFeatureFlagBits = 4;
pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT_KHR: VkPeerMemoryFeatureFlagBits = 8;
pub const VK_PEER_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM: VkPeerMemoryFeatureFlagBits = 2147483647;

pub type VkMemoryAllocateFlags = VkFlags;
pub type VkMemoryAllocateFlagBits = VkFlags;
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlagBits = 1;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT: VkMemoryAllocateFlagBits = 2;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT: VkMemoryAllocateFlagBits = 4;
pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT_KHR: VkMemoryAllocateFlagBits = 1;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_BIT_KHR: VkMemoryAllocateFlagBits = 2;
pub const VK_MEMORY_ALLOCATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkMemoryAllocateFlagBits = 4;
pub const VK_MEMORY_ALLOCATE_FLAG_BITS_MAX_ENUM: VkMemoryAllocateFlagBits = 2147483647;

pub type VkCommandPoolTrimFlags = VkFlags;
pub type VkDescriptorUpdateTemplateCreateFlags = VkFlags;

pub type VkExternalMemoryHandleTypeFlags = VkFlags;
pub type VkExternalMemoryHandleTypeFlagBits = VkFlags;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlagBits = 16;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlagBits = 32;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlagBits = 64;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_DMA_BUF_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 512;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ANDROID_HARDWARE_BUFFER_BIT_ANDROID: VkExternalMemoryHandleTypeFlagBits = 1024;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_ALLOCATION_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 128;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_HOST_MAPPED_FOREIGN_MEMORY_BIT_EXT: VkExternalMemoryHandleTypeFlagBits = 256;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_ZIRCON_VMO_BIT_FUCHSIA: VkExternalMemoryHandleTypeFlagBits = 2048;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_RDMA_ADDRESS_BIT_NV: VkExternalMemoryHandleTypeFlagBits = 4096;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 16;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 32;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT_KHR: VkExternalMemoryHandleTypeFlagBits = 64;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM: VkExternalMemoryHandleTypeFlagBits = 2147483647;

pub type VkExternalMemoryFeatureFlags = VkFlags;
pub type VkExternalMemoryFeatureFlagBits = VkFlags;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlagBits = 1;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = 2;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlagBits = 4;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_KHR: VkExternalMemoryFeatureFlagBits = 1;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits = 2;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_KHR: VkExternalMemoryFeatureFlagBits = 4;
pub const VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM: VkExternalMemoryFeatureFlagBits = 2147483647;

pub type VkExternalFenceHandleTypeFlags = VkFlags;
pub type VkExternalFenceHandleTypeFlagBits = VkFlags;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalFenceHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalFenceHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalFenceHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_FLAG_BITS_MAX_ENUM: VkExternalFenceHandleTypeFlagBits = 2147483647;

pub type VkExternalFenceFeatureFlags = VkFlags;
pub type VkExternalFenceFeatureFlagBits = VkFlags;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlagBits = 1;
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlagBits = 2;
pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits = 1;
pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalFenceFeatureFlagBits = 2;
pub const VK_EXTERNAL_FENCE_FEATURE_FLAG_BITS_MAX_ENUM: VkExternalFenceFeatureFlagBits = 2147483647;

pub type VkFenceImportFlags = VkFlags;
pub type VkFenceImportFlagBits = VkFlags;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlagBits = 1;
pub const VK_FENCE_IMPORT_TEMPORARY_BIT_KHR: VkFenceImportFlagBits = 1;
pub const VK_FENCE_IMPORT_FLAG_BITS_MAX_ENUM: VkFenceImportFlagBits = 2147483647;

pub type VkSemaphoreImportFlags = VkFlags;
pub type VkSemaphoreImportFlagBits = VkFlags;
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlagBits = 1;
pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT_KHR: VkSemaphoreImportFlagBits = 1;
pub const VK_SEMAPHORE_IMPORT_FLAG_BITS_MAX_ENUM: VkSemaphoreImportFlagBits = 2147483647;

pub type VkExternalSemaphoreHandleTypeFlags = VkFlags;
pub type VkExternalSemaphoreHandleTypeFlagBits = VkFlags;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlagBits = 16;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_ZIRCON_EVENT_BIT_FUCHSIA: VkExternalSemaphoreHandleTypeFlagBits = 128;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = 1;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = 2;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = 4;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = 8;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT_KHR: VkExternalSemaphoreHandleTypeFlagBits = 16;
pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_FLAG_BITS_MAX_ENUM: VkExternalSemaphoreHandleTypeFlagBits = 2147483647;

pub type VkExternalSemaphoreFeatureFlags = VkFlags;
pub type VkExternalSemaphoreFeatureFlagBits = VkFlags;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = 1;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlagBits = 2;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBits = 1;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT_KHR: VkExternalSemaphoreFeatureFlagBits = 2;
pub const VK_EXTERNAL_SEMAPHORE_FEATURE_FLAG_BITS_MAX_ENUM: VkExternalSemaphoreFeatureFlagBits = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSubgroupProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub subgroupSize: u32,
	pub supportedStages: VkShaderStageFlags,
	pub supportedOperations: VkSubgroupFeatureFlags,
	pub quadOperationsInAllStages: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindBufferMemoryInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub buffer: VkBuffer,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindImageMemoryInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub image: VkImage,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevice16BitStorageFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryDedicatedRequirements {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub prefersDedicatedAllocation: VkBool32,
	pub requiresDedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryDedicatedAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub image: VkImage,
	pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryAllocateFlagsInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkMemoryAllocateFlags,
	pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupRenderPassBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceMask: u32,
	pub deviceRenderAreaCount: u32,
	pub pDeviceRenderAreas: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphoreDeviceIndices: *const u32,
	pub commandBufferCount: u32,
	pub pCommandBufferDeviceMasks: *const u32,
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphoreDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupBindSparseInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub resourceDeviceIndex: u32,
	pub memoryDeviceIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindImageMemoryDeviceGroupInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32,
	pub splitInstanceBindRegionCount: u32,
	pub pSplitInstanceBindRegions: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceGroupProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub physicalDeviceCount: u32,
	pub physicalDevices: [VkPhysicalDevice; 32],
	pub subsetAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupDeviceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub physicalDeviceCount: u32,
	pub pPhysicalDevices: *const VkPhysicalDevice,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferMemoryRequirementsInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageMemoryRequirementsInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub image: VkImage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub image: VkImage,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryRequirements2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryRequirements: VkMemoryRequirements,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageMemoryRequirements2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryRequirements: VkSparseImageMemoryRequirements,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFeatures2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub features: VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub properties: VkPhysicalDeviceProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFormatProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub formatProperties: VkFormatProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageFormatProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub imageFormatProperties: VkImageFormatProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub format: VkFormat,
	pub type_: VkImageType,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags,
	pub flags: VkImageCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueueFamilyProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub queueFamilyProperties: VkQueueFamilyProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMemoryProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryProperties: VkPhysicalDeviceMemoryProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSparseImageFormatProperties2 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub properties: VkSparseImageFormatProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub format: VkFormat,
	pub type_: VkImageType,
	pub samples: VkSampleCountFlagBits,
	pub usage: VkImageUsageFlags,
	pub tiling: VkImageTiling,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePointClippingProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pointClippingBehavior: VkPointClippingBehavior,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkInputAttachmentAspectReference {
	pub subpass: u32,
	pub inputAttachmentIndex: u32,
	pub aspectMask: VkImageAspectFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub aspectReferenceCount: u32,
	pub pAspectReferences: *const VkInputAttachmentAspectReference,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewUsageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub usage: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub domainOrigin: VkTessellationDomainOrigin,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassMultiviewCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub subpassCount: u32,
	pub pViewMasks: *const u32,
	pub dependencyCount: u32,
	pub pViewOffsets: *const i32,
	pub correlationMaskCount: u32,
	pub pCorrelationMasks: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMultiviewFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMultiviewProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32,
}

pub type VkPhysicalDeviceVariablePointerFeatures = VkPhysicalDeviceVariablePointersFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub protectedMemory: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub protectedNoFault: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceQueueInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkProtectedSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub protectedSubmit: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerYcbcrConversionCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub format: VkFormat,
	pub ycbcrModel: VkSamplerYcbcrModelConversion,
	pub ycbcrRange: VkSamplerYcbcrRange,
	pub components: VkComponentMapping,
	pub xChromaOffset: VkChromaLocation,
	pub yChromaOffset: VkChromaLocation,
	pub chromaFilter: VkFilter,
	pub forceExplicitReconstruction: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerYcbcrConversionInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub conversion: VkSamplerYcbcrConversion,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindImagePlaneMemoryInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub planeAspect: VkImageAspectFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImagePlaneMemoryRequirementsInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub planeAspect: VkImageAspectFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub samplerYcbcrConversion: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub combinedImageSamplerDescriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorUpdateTemplateEntry {
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub offset: usize,
	pub stride: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDescriptorUpdateTemplateCreateFlags,
	pub descriptorUpdateEntryCount: u32,
	pub pDescriptorUpdateEntries: *const VkDescriptorUpdateTemplateEntry,
	pub templateType: VkDescriptorUpdateTemplateType,
	pub descriptorSetLayout: VkDescriptorSetLayout,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipelineLayout: VkPipelineLayout,
	pub set: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalMemoryProperties {
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlags,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalImageFormatProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalBufferInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkBufferCreateFlags,
	pub usage: VkBufferUsageFlags,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalBufferProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub externalMemoryProperties: VkExternalMemoryProperties,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceIDProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceUUID: [u8; 16],
	pub driverUUID: [u8; 16],
	pub deviceLUID: [u8; 8],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalMemoryImageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalMemoryBufferCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExportMemoryAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalFenceInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleType: VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalFenceProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub exportFromImportedHandleTypes: VkExternalFenceHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalFenceHandleTypeFlags,
	pub externalFenceFeatures: VkExternalFenceFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExportFenceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalFenceHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExportSemaphoreCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalSemaphoreProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub exportFromImportedHandleTypes: VkExternalSemaphoreHandleTypeFlags,
	pub compatibleHandleTypes: VkExternalSemaphoreHandleTypeFlags,
	pub externalSemaphoreFeatures: VkExternalSemaphoreFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMaintenance3Properties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutSupport {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub supported: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderDrawParameters: VkBool32,
}

pub type VkPhysicalDeviceShaderDrawParameterFeatures = VkPhysicalDeviceShaderDrawParametersFeatures;
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "C" fn(pApiVersion: *mut u32) -> VkResult;

pub type PFN_vkBindBufferMemory2 = unsafe extern "C" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;

pub type PFN_vkBindImageMemory2 = unsafe extern "C" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;

pub type PFN_vkGetDeviceGroupPeerMemoryFeatures =
	unsafe extern "C" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);

pub type PFN_vkCmdSetDeviceMask = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);

pub type PFN_vkCmdDispatchBase = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

pub type PFN_vkEnumeratePhysicalDeviceGroups =
	unsafe extern "C" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;

pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "C" fn(
	device: VkDevice,
	pInfo: *const VkImageSparseMemoryRequirementsInfo2,
	pSparseMemoryRequirementCount: *mut u32,
	pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);

pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);

pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);

pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);

pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);

pub type PFN_vkTrimCommandPool = unsafe extern "C" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);

pub type PFN_vkGetDeviceQueue2 = unsafe extern "C" fn(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);

pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;

pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "C" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;

pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "C" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "C" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const ());

pub type PFN_vkGetPhysicalDeviceExternalBufferProperties =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);

pub type PFN_vkGetPhysicalDeviceExternalFenceProperties =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);

pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);

pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkEnumerateInstanceVersion(pApiVersion: *mut u32) -> VkResult;

	pub fn vkBindBufferMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;

	pub fn vkBindImageMemory2(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;

	pub fn vkGetDeviceGroupPeerMemoryFeatures(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);

	pub fn vkCmdSetDeviceMask(commandBuffer: VkCommandBuffer, deviceMask: u32);

	pub fn vkCmdDispatchBase(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

	pub fn vkEnumeratePhysicalDeviceGroups(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;

	pub fn vkGetImageMemoryRequirements2(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetBufferMemoryRequirements2(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetImageSparseMemoryRequirements2(
		device: VkDevice,
		pInfo: *const VkImageSparseMemoryRequirementsInfo2,
		pSparseMemoryRequirementCount: *mut u32,
		pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
	);

	pub fn vkGetPhysicalDeviceFeatures2(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);

	pub fn vkGetPhysicalDeviceProperties2(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);

	pub fn vkGetPhysicalDeviceFormatProperties2(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);

	pub fn vkGetPhysicalDeviceImageFormatProperties2(
		physicalDevice: VkPhysicalDevice,
		pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
		pImageFormatProperties: *mut VkImageFormatProperties2,
	) -> VkResult;

	pub fn vkGetPhysicalDeviceQueueFamilyProperties2(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);

	pub fn vkGetPhysicalDeviceMemoryProperties2(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);

	pub fn vkGetPhysicalDeviceSparseImageFormatProperties2(
		physicalDevice: VkPhysicalDevice,
		pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
		pPropertyCount: *mut u32,
		pProperties: *mut VkSparseImageFormatProperties2,
	);

	pub fn vkTrimCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);

	pub fn vkGetDeviceQueue2(device: VkDevice, pQueueInfo: *const VkDeviceQueueInfo2, pQueue: *mut VkQueue);

	pub fn vkCreateSamplerYcbcrConversion(
		device: VkDevice,
		pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pYcbcrConversion: *mut VkSamplerYcbcrConversion,
	) -> VkResult;

	pub fn vkDestroySamplerYcbcrConversion(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCreateDescriptorUpdateTemplate(
		device: VkDevice,
		pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
	) -> VkResult;

	pub fn vkDestroyDescriptorUpdateTemplate(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);

	pub fn vkUpdateDescriptorSetWithTemplate(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const ());

	pub fn vkGetPhysicalDeviceExternalBufferProperties(
		physicalDevice: VkPhysicalDevice,
		pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
		pExternalBufferProperties: *mut VkExternalBufferProperties,
	);

	pub fn vkGetPhysicalDeviceExternalFenceProperties(
		physicalDevice: VkPhysicalDevice,
		pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
		pExternalFenceProperties: *mut VkExternalFenceProperties,
	);

	pub fn vkGetPhysicalDeviceExternalSemaphoreProperties(
		physicalDevice: VkPhysicalDevice,
		pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
		pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
	);

	pub fn vkGetDescriptorSetLayoutSupport(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
}

pub type VkDriverId = u32;
pub const VK_DRIVER_ID_AMD_PROPRIETARY: VkDriverId = 1;
pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: VkDriverId = 2;
pub const VK_DRIVER_ID_MESA_RADV: VkDriverId = 3;
pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: VkDriverId = 4;
pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: VkDriverId = 5;
pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: VkDriverId = 6;
pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: VkDriverId = 7;
pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: VkDriverId = 8;
pub const VK_DRIVER_ID_ARM_PROPRIETARY: VkDriverId = 9;
pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: VkDriverId = 10;
pub const VK_DRIVER_ID_GGP_PROPRIETARY: VkDriverId = 11;
pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: VkDriverId = 12;
pub const VK_DRIVER_ID_MESA_LLVMPIPE: VkDriverId = 13;
pub const VK_DRIVER_ID_MOLTENVK: VkDriverId = 14;
pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: VkDriverId = 15;
pub const VK_DRIVER_ID_JUICE_PROPRIETARY: VkDriverId = 16;
pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: VkDriverId = 17;
pub const VK_DRIVER_ID_MESA_TURNIP: VkDriverId = 18;
pub const VK_DRIVER_ID_MESA_V3DV: VkDriverId = 19;
pub const VK_DRIVER_ID_MESA_PANVK: VkDriverId = 20;
pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: VkDriverId = 21;
pub const VK_DRIVER_ID_MESA_VENUS: VkDriverId = 22;
pub const VK_DRIVER_ID_AMD_PROPRIETARY_KHR: VkDriverId = 1;
pub const VK_DRIVER_ID_AMD_OPEN_SOURCE_KHR: VkDriverId = 2;
pub const VK_DRIVER_ID_MESA_RADV_KHR: VkDriverId = 3;
pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY_KHR: VkDriverId = 4;
pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS_KHR: VkDriverId = 5;
pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA_KHR: VkDriverId = 6;
pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY_KHR: VkDriverId = 7;
pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY_KHR: VkDriverId = 8;
pub const VK_DRIVER_ID_ARM_PROPRIETARY_KHR: VkDriverId = 9;
pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER_KHR: VkDriverId = 10;
pub const VK_DRIVER_ID_GGP_PROPRIETARY_KHR: VkDriverId = 11;
pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY_KHR: VkDriverId = 12;
pub const VK_DRIVER_ID_MAX_ENUM: VkDriverId = 2147483647;

pub type VkShaderFloatControlsIndependence = u32;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: VkShaderFloatControlsIndependence = 0;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: VkShaderFloatControlsIndependence = 1;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: VkShaderFloatControlsIndependence = 2;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR: VkShaderFloatControlsIndependence = 0;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR: VkShaderFloatControlsIndependence = 1;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR: VkShaderFloatControlsIndependence = 2;
pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_MAX_ENUM: VkShaderFloatControlsIndependence = 2147483647;

pub type VkSamplerReductionMode = u32;
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: VkSamplerReductionMode = 0;
pub const VK_SAMPLER_REDUCTION_MODE_MIN: VkSamplerReductionMode = 1;
pub const VK_SAMPLER_REDUCTION_MODE_MAX: VkSamplerReductionMode = 2;
pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT: VkSamplerReductionMode = 0;
pub const VK_SAMPLER_REDUCTION_MODE_MIN_EXT: VkSamplerReductionMode = 1;
pub const VK_SAMPLER_REDUCTION_MODE_MAX_EXT: VkSamplerReductionMode = 2;
pub const VK_SAMPLER_REDUCTION_MODE_MAX_ENUM: VkSamplerReductionMode = 2147483647;

pub type VkSemaphoreType = u32;
pub const VK_SEMAPHORE_TYPE_BINARY: VkSemaphoreType = 0;
pub const VK_SEMAPHORE_TYPE_TIMELINE: VkSemaphoreType = 1;
pub const VK_SEMAPHORE_TYPE_BINARY_KHR: VkSemaphoreType = 0;
pub const VK_SEMAPHORE_TYPE_TIMELINE_KHR: VkSemaphoreType = 1;
pub const VK_SEMAPHORE_TYPE_MAX_ENUM: VkSemaphoreType = 2147483647;

pub type VkResolveModeFlags = VkFlags;
pub type VkResolveModeFlagBits = VkFlags;
pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlagBits = 0;
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlagBits = 1;
pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlagBits = 2;
pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlagBits = 4;
pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlagBits = 8;
pub const VK_RESOLVE_MODE_NONE_KHR: VkResolveModeFlagBits = 0;
pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR: VkResolveModeFlagBits = 1;
pub const VK_RESOLVE_MODE_AVERAGE_BIT_KHR: VkResolveModeFlagBits = 2;
pub const VK_RESOLVE_MODE_MIN_BIT_KHR: VkResolveModeFlagBits = 4;
pub const VK_RESOLVE_MODE_MAX_BIT_KHR: VkResolveModeFlagBits = 8;
pub const VK_RESOLVE_MODE_FLAG_BITS_MAX_ENUM: VkResolveModeFlagBits = 2147483647;

pub type VkDescriptorBindingFlags = VkFlags;
pub type VkDescriptorBindingFlagBits = VkFlags;
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlagBits = 1;
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlagBits = 2;
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlagBits = 4;
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlagBits = 8;
pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT_EXT: VkDescriptorBindingFlagBits = 1;
pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT_EXT: VkDescriptorBindingFlagBits = 2;
pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT_EXT: VkDescriptorBindingFlagBits = 4;
pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT_EXT: VkDescriptorBindingFlagBits = 8;
pub const VK_DESCRIPTOR_BINDING_FLAG_BITS_MAX_ENUM: VkDescriptorBindingFlagBits = 2147483647;

pub type VkSemaphoreWaitFlags = VkFlags;
pub type VkSemaphoreWaitFlagBits = VkFlags;
pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlagBits = 1;
pub const VK_SEMAPHORE_WAIT_ANY_BIT_KHR: VkSemaphoreWaitFlagBits = 1;
pub const VK_SEMAPHORE_WAIT_FLAG_BITS_MAX_ENUM: VkSemaphoreWaitFlagBits = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan11Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub storageBuffer16BitAccess: VkBool32,
	pub uniformAndStorageBuffer16BitAccess: VkBool32,
	pub storagePushConstant16: VkBool32,
	pub storageInputOutput16: VkBool32,
	pub multiview: VkBool32,
	pub multiviewGeometryShader: VkBool32,
	pub multiviewTessellationShader: VkBool32,
	pub variablePointersStorageBuffer: VkBool32,
	pub variablePointers: VkBool32,
	pub protectedMemory: VkBool32,
	pub samplerYcbcrConversion: VkBool32,
	pub shaderDrawParameters: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan11Properties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceUUID: [u8; 16],
	pub driverUUID: [u8; 16],
	pub deviceLUID: [u8; 8],
	pub deviceNodeMask: u32,
	pub deviceLUIDValid: VkBool32,
	pub subgroupSize: u32,
	pub subgroupSupportedStages: VkShaderStageFlags,
	pub subgroupSupportedOperations: VkSubgroupFeatureFlags,
	pub subgroupQuadOperationsInAllStages: VkBool32,
	pub pointClippingBehavior: VkPointClippingBehavior,
	pub maxMultiviewViewCount: u32,
	pub maxMultiviewInstanceIndex: u32,
	pub protectedNoFault: VkBool32,
	pub maxPerSetDescriptors: u32,
	pub maxMemoryAllocationSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan12Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub samplerMirrorClampToEdge: VkBool32,
	pub drawIndirectCount: VkBool32,
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32,
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32,
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32,
	pub descriptorIndexing: VkBool32,
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32,
	pub samplerFilterMinmax: VkBool32,
	pub scalarBlockLayout: VkBool32,
	pub imagelessFramebuffer: VkBool32,
	pub uniformBufferStandardLayout: VkBool32,
	pub shaderSubgroupExtendedTypes: VkBool32,
	pub separateDepthStencilLayouts: VkBool32,
	pub hostQueryReset: VkBool32,
	pub timelineSemaphore: VkBool32,
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
	pub shaderOutputViewportIndex: VkBool32,
	pub shaderOutputLayer: VkBool32,
	pub subgroupBroadcastDynamicId: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkConformanceVersion {
	pub major: u8,
	pub minor: u8,
	pub subminor: u8,
	pub patch: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan12Properties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub driverID: VkDriverId,
	pub driverName: [i8; 256],
	pub driverInfo: [i8; 256],
	pub conformanceVersion: VkConformanceVersion,
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32,
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
	pub supportedDepthResolveModes: VkResolveModeFlags,
	pub supportedStencilResolveModes: VkResolveModeFlags,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32,
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32,
	pub maxTimelineSemaphoreValueDifference: u64,
	pub framebufferIntegerColorSampleCounts: VkSampleCountFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageFormatListCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub viewFormatCount: u32,
	pub pViewFormats: *const VkFormat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentDescription2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkAttachmentDescriptionFlags,
	pub format: VkFormat,
	pub samples: VkSampleCountFlagBits,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentReference2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub attachment: u32,
	pub layout: VkImageLayout,
	pub aspectMask: VkImageAspectFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassDescription2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSubpassDescriptionFlags,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub viewMask: u32,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: *const VkAttachmentReference2,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkAttachmentReference2,
	pub pResolveAttachments: *const VkAttachmentReference2,
	pub pDepthStencilAttachment: *const VkAttachmentReference2,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassDependency2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags,
	pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags,
	pub viewOffset: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassCreateInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32,
	pub pAttachments: *const VkAttachmentDescription2,
	pub subpassCount: u32,
	pub pSubpasses: *const VkSubpassDescription2,
	pub dependencyCount: u32,
	pub pDependencies: *const VkSubpassDependency2,
	pub correlatedViewMaskCount: u32,
	pub pCorrelatedViewMasks: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub contents: VkSubpassContents,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassEndInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevice8BitStorageFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub storageBuffer8BitAccess: VkBool32,
	pub uniformAndStorageBuffer8BitAccess: VkBool32,
	pub storagePushConstant8: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDriverProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub driverID: VkDriverId,
	pub driverName: [i8; 256],
	pub driverInfo: [i8; 256],
	pub conformanceVersion: VkConformanceVersion,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderBufferInt64Atomics: VkBool32,
	pub shaderSharedInt64Atomics: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderFloat16: VkBool32,
	pub shaderInt8: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFloatControlsProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub denormBehaviorIndependence: VkShaderFloatControlsIndependence,
	pub roundingModeIndependence: VkShaderFloatControlsIndependence,
	pub shaderSignedZeroInfNanPreserveFloat16: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat32: VkBool32,
	pub shaderSignedZeroInfNanPreserveFloat64: VkBool32,
	pub shaderDenormPreserveFloat16: VkBool32,
	pub shaderDenormPreserveFloat32: VkBool32,
	pub shaderDenormPreserveFloat64: VkBool32,
	pub shaderDenormFlushToZeroFloat16: VkBool32,
	pub shaderDenormFlushToZeroFloat32: VkBool32,
	pub shaderDenormFlushToZeroFloat64: VkBool32,
	pub shaderRoundingModeRTEFloat16: VkBool32,
	pub shaderRoundingModeRTEFloat32: VkBool32,
	pub shaderRoundingModeRTEFloat64: VkBool32,
	pub shaderRoundingModeRTZFloat16: VkBool32,
	pub shaderRoundingModeRTZFloat32: VkBool32,
	pub shaderRoundingModeRTZFloat64: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub bindingCount: u32,
	pub pBindingFlags: *const VkDescriptorBindingFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderInputAttachmentArrayDynamicIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayDynamicIndexing: VkBool32,
	pub shaderUniformBufferArrayNonUniformIndexing: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexing: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexing: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexing: VkBool32,
	pub shaderUniformTexelBufferArrayNonUniformIndexing: VkBool32,
	pub shaderStorageTexelBufferArrayNonUniformIndexing: VkBool32,
	pub descriptorBindingUniformBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingSampledImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageImageUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUniformTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingStorageTexelBufferUpdateAfterBind: VkBool32,
	pub descriptorBindingUpdateUnusedWhilePending: VkBool32,
	pub descriptorBindingPartiallyBound: VkBool32,
	pub descriptorBindingVariableDescriptorCount: VkBool32,
	pub runtimeDescriptorArray: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxUpdateAfterBindDescriptorsInAllPools: u32,
	pub shaderUniformBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderSampledImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageBufferArrayNonUniformIndexingNative: VkBool32,
	pub shaderStorageImageArrayNonUniformIndexingNative: VkBool32,
	pub shaderInputAttachmentArrayNonUniformIndexingNative: VkBool32,
	pub robustBufferAccessUpdateAfterBind: VkBool32,
	pub quadDivergentImplicitLod: VkBool32,
	pub maxPerStageDescriptorUpdateAfterBindSamplers: u32,
	pub maxPerStageDescriptorUpdateAfterBindUniformBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageBuffers: u32,
	pub maxPerStageDescriptorUpdateAfterBindSampledImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindStorageImages: u32,
	pub maxPerStageDescriptorUpdateAfterBindInputAttachments: u32,
	pub maxPerStageUpdateAfterBindResources: u32,
	pub maxDescriptorSetUpdateAfterBindSamplers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffers: u32,
	pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic: u32,
	pub maxDescriptorSetUpdateAfterBindSampledImages: u32,
	pub maxDescriptorSetUpdateAfterBindStorageImages: u32,
	pub maxDescriptorSetUpdateAfterBindInputAttachments: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub descriptorSetCount: u32,
	pub pDescriptorCounts: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxVariableDescriptorCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassDescriptionDepthStencilResolve {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub depthResolveMode: VkResolveModeFlagBits,
	pub stencilResolveMode: VkResolveModeFlagBits,
	pub pDepthStencilResolveAttachment: *const VkAttachmentReference2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub supportedDepthResolveModes: VkResolveModeFlags,
	pub supportedStencilResolveModes: VkResolveModeFlags,
	pub independentResolveNone: VkBool32,
	pub independentResolve: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub scalarBlockLayout: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageStencilUsageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub stencilUsage: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerReductionModeCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub reductionMode: VkSamplerReductionMode,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub filterMinmaxSingleComponentFormats: VkBool32,
	pub filterMinmaxImageComponentMapping: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub vulkanMemoryModel: VkBool32,
	pub vulkanMemoryModelDeviceScope: VkBool32,
	pub vulkanMemoryModelAvailabilityVisibilityChains: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub imagelessFramebuffer: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFramebufferAttachmentImageInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkImageCreateFlags,
	pub usage: VkImageUsageFlags,
	pub width: u32,
	pub height: u32,
	pub layerCount: u32,
	pub viewFormatCount: u32,
	pub pViewFormats: *const VkFormat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFramebufferAttachmentsCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub attachmentImageInfoCount: u32,
	pub pAttachmentImageInfos: *const VkFramebufferAttachmentImageInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassAttachmentBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub attachmentCount: u32,
	pub pAttachments: *const VkImageView,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub uniformBufferStandardLayout: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderSubgroupExtendedTypes: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub separateDepthStencilLayouts: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentReferenceStencilLayout {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub stencilLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentDescriptionStencilLayout {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub stencilInitialLayout: VkImageLayout,
	pub stencilFinalLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub hostQueryReset: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub timelineSemaphore: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxTimelineSemaphoreValueDifference: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreTypeCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub semaphoreType: VkSemaphoreType,
	pub initialValue: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkTimelineSemaphoreSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub waitSemaphoreValueCount: u32,
	pub pWaitSemaphoreValues: *const u64,
	pub signalSemaphoreValueCount: u32,
	pub pSignalSemaphoreValues: *const u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreWaitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSemaphoreWaitFlags,
	pub semaphoreCount: u32,
	pub pSemaphores: *const VkSemaphore,
	pub pValues: *const u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreSignalInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub semaphore: VkSemaphore,
	pub value: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferDeviceAddressInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub buffer: VkBuffer,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub opaqueCaptureAddress: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub opaqueCaptureAddress: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub memory: VkDeviceMemory,
}

pub type PFN_vkCmdDrawIndirectCount =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

pub type PFN_vkCmdDrawIndexedIndirectCount =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

pub type PFN_vkCreateRenderPass2 =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);

pub type PFN_vkCmdNextSubpass2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);

pub type PFN_vkCmdEndRenderPass2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);

pub type PFN_vkResetQueryPool = unsafe extern "C" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

pub type PFN_vkGetSemaphoreCounterValue = unsafe extern "C" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;

pub type PFN_vkWaitSemaphores = unsafe extern "C" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;

pub type PFN_vkSignalSemaphore = unsafe extern "C" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;

pub type PFN_vkGetBufferDeviceAddress = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

pub type PFN_vkGetBufferOpaqueCaptureAddress = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdDrawIndirectCount(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

	pub fn vkCmdDrawIndexedIndirectCount(
		commandBuffer: VkCommandBuffer,
		buffer: VkBuffer,
		offset: VkDeviceSize,
		countBuffer: VkBuffer,
		countBufferOffset: VkDeviceSize,
		maxDrawCount: u32,
		stride: u32,
	);

	pub fn vkCreateRenderPass2(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

	pub fn vkCmdBeginRenderPass2(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);

	pub fn vkCmdNextSubpass2(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);

	pub fn vkCmdEndRenderPass2(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);

	pub fn vkResetQueryPool(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

	pub fn vkGetSemaphoreCounterValue(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;

	pub fn vkWaitSemaphores(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;

	pub fn vkSignalSemaphore(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;

	pub fn vkGetBufferDeviceAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

	pub fn vkGetBufferOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;

	pub fn vkGetDeviceMemoryOpaqueCaptureAddress(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;
}

pub type VkFlags64 = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPrivateDataSlot_T {
	_unused: [u8; 0],
}

pub type VkPrivateDataSlot = *mut VkPrivateDataSlot_T;

pub type VkPipelineCreationFeedbackFlags = VkFlags;
pub type VkPipelineCreationFeedbackFlagBits = VkFlags;
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlagBits = 1;
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT: VkPipelineCreationFeedbackFlagBits = 2;
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT: VkPipelineCreationFeedbackFlagBits = 4;
pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT_EXT: VkPipelineCreationFeedbackFlagBits = 1;
pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT_EXT: VkPipelineCreationFeedbackFlagBits = 2;
pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT_EXT: VkPipelineCreationFeedbackFlagBits = 4;
pub const VK_PIPELINE_CREATION_FEEDBACK_FLAG_BITS_MAX_ENUM: VkPipelineCreationFeedbackFlagBits = 2147483647;

pub type VkToolPurposeFlags = VkFlags;
pub type VkToolPurposeFlagBits = VkFlags;
pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlagBits = 1;
pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlagBits = 2;
pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlagBits = 4;
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlagBits = 8;
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlagBits = 16;
pub const VK_TOOL_PURPOSE_DEBUG_REPORTING_BIT_EXT: VkToolPurposeFlagBits = 32;
pub const VK_TOOL_PURPOSE_DEBUG_MARKERS_BIT_EXT: VkToolPurposeFlagBits = 64;
pub const VK_TOOL_PURPOSE_VALIDATION_BIT_EXT: VkToolPurposeFlagBits = 1;
pub const VK_TOOL_PURPOSE_PROFILING_BIT_EXT: VkToolPurposeFlagBits = 2;
pub const VK_TOOL_PURPOSE_TRACING_BIT_EXT: VkToolPurposeFlagBits = 4;
pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT_EXT: VkToolPurposeFlagBits = 8;
pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT_EXT: VkToolPurposeFlagBits = 16;
pub const VK_TOOL_PURPOSE_FLAG_BITS_MAX_ENUM: VkToolPurposeFlagBits = 2147483647;

pub type VkPrivateDataSlotCreateFlags = VkFlags;
pub type VkPipelineStageFlags2 = VkFlags64;

pub type VkAccessFlags2 = VkFlags64;
pub type VkPipelineStageFlagBits2 = VkFlags64;
pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlagBits2 = 0;
pub const VK_PIPELINE_STAGE_2_NONE_KHR: VkPipelineStageFlagBits2 = 0;
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlagBits2 = 1;
pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 = 1;
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlagBits2 = 2;
pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR: VkPipelineStageFlagBits2 = 2;
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlagBits2 = 4;
pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = 4;
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlagBits2 = 8;
pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 8;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlagBits2 = 16;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 16;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlagBits2 = 32;
pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 32;
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlagBits2 = 64;
pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 64;
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlagBits2 = 128;
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 128;
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = 256;
pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 = 256;
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlagBits2 = 512;
pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR: VkPipelineStageFlagBits2 = 512;
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlagBits2 = 1024;
pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR: VkPipelineStageFlagBits2 = 1024;
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlagBits2 = 2048;
pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 2048;
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlagBits2 = 4096;
pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 = 4096;
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlagBits2 = 4096;
pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT_KHR: VkPipelineStageFlagBits2 = 4096;
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlagBits2 = 8192;
pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR: VkPipelineStageFlagBits2 = 8192;
pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlagBits2 = 16384;
pub const VK_PIPELINE_STAGE_2_HOST_BIT_KHR: VkPipelineStageFlagBits2 = 16384;
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlagBits2 = 32768;
pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR: VkPipelineStageFlagBits2 = 32768;
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlagBits2 = 65536;
pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR: VkPipelineStageFlagBits2 = 65536;
pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlagBits2 = 4294967296;
pub const VK_PIPELINE_STAGE_2_COPY_BIT_KHR: VkPipelineStageFlagBits2 = 4294967296;
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlagBits2 = 8589934592;
pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT_KHR: VkPipelineStageFlagBits2 = 8589934592;
pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlagBits2 = 17179869184;
pub const VK_PIPELINE_STAGE_2_BLIT_BIT_KHR: VkPipelineStageFlagBits2 = 17179869184;
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlagBits2 = 34359738368;
pub const VK_PIPELINE_STAGE_2_CLEAR_BIT_KHR: VkPipelineStageFlagBits2 = 34359738368;
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlagBits2 = 68719476736;
pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = 68719476736;
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlagBits2 = 137438953472;
pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR: VkPipelineStageFlagBits2 = 137438953472;
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlagBits2 = 274877906944;
pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR: VkPipelineStageFlagBits2 = 274877906944;
pub const VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT: VkPipelineStageFlagBits2 = 16777216;
pub const VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT: VkPipelineStageFlagBits2 = 262144;
pub const VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV: VkPipelineStageFlagBits2 = 131072;
pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkPipelineStageFlagBits2 = 4194304;
pub const VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV: VkPipelineStageFlagBits2 = 4194304;
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR: VkPipelineStageFlagBits2 = 33554432;
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR: VkPipelineStageFlagBits2 = 2097152;
pub const VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV: VkPipelineStageFlagBits2 = 2097152;
pub const VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV: VkPipelineStageFlagBits2 = 33554432;
pub const VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT: VkPipelineStageFlagBits2 = 8388608;
pub const VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV: VkPipelineStageFlagBits2 = 524288;
pub const VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV: VkPipelineStageFlagBits2 = 1048576;
pub const VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI: VkPipelineStageFlagBits2 = 549755813888;
pub const VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI: VkPipelineStageFlagBits2 = 1099511627776;

pub type VkAccessFlagBits2 = VkFlags64;
pub const VK_ACCESS_2_NONE: VkAccessFlagBits2 = 0;
pub const VK_ACCESS_2_NONE_KHR: VkAccessFlagBits2 = 0;
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlagBits2 = 1;
pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR: VkAccessFlagBits2 = 1;
pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlagBits2 = 2;
pub const VK_ACCESS_2_INDEX_READ_BIT_KHR: VkAccessFlagBits2 = 2;
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlagBits2 = 4;
pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR: VkAccessFlagBits2 = 4;
pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlagBits2 = 8;
pub const VK_ACCESS_2_UNIFORM_READ_BIT_KHR: VkAccessFlagBits2 = 8;
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 16;
pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = 16;
pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlagBits2 = 32;
pub const VK_ACCESS_2_SHADER_READ_BIT_KHR: VkAccessFlagBits2 = 32;
pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlagBits2 = 64;
pub const VK_ACCESS_2_SHADER_WRITE_BIT_KHR: VkAccessFlagBits2 = 64;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 128;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = 128;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = 256;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 = 256;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlagBits2 = 512;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = 512;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlagBits2 = 1024;
pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR: VkAccessFlagBits2 = 1024;
pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlagBits2 = 2048;
pub const VK_ACCESS_2_TRANSFER_READ_BIT_KHR: VkAccessFlagBits2 = 2048;
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlagBits2 = 4096;
pub const VK_ACCESS_2_TRANSFER_WRITE_BIT_KHR: VkAccessFlagBits2 = 4096;
pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlagBits2 = 8192;
pub const VK_ACCESS_2_HOST_READ_BIT_KHR: VkAccessFlagBits2 = 8192;
pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlagBits2 = 16384;
pub const VK_ACCESS_2_HOST_WRITE_BIT_KHR: VkAccessFlagBits2 = 16384;
pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlagBits2 = 32768;
pub const VK_ACCESS_2_MEMORY_READ_BIT_KHR: VkAccessFlagBits2 = 32768;
pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlagBits2 = 65536;
pub const VK_ACCESS_2_MEMORY_WRITE_BIT_KHR: VkAccessFlagBits2 = 65536;
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlagBits2 = 4294967296;
pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR: VkAccessFlagBits2 = 4294967296;
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlagBits2 = 8589934592;
pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT_KHR: VkAccessFlagBits2 = 8589934592;
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlagBits2 = 17179869184;
pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR: VkAccessFlagBits2 = 17179869184;
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT: VkAccessFlagBits2 = 33554432;
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT: VkAccessFlagBits2 = 67108864;
pub const VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT: VkAccessFlagBits2 = 134217728;
pub const VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT: VkAccessFlagBits2 = 1048576;
pub const VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV: VkAccessFlagBits2 = 131072;
pub const VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV: VkAccessFlagBits2 = 262144;
pub const VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR: VkAccessFlagBits2 = 8388608;
pub const VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV: VkAccessFlagBits2 = 8388608;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR: VkAccessFlagBits2 = 2097152;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR: VkAccessFlagBits2 = 4194304;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV: VkAccessFlagBits2 = 2097152;
pub const VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV: VkAccessFlagBits2 = 4194304;
pub const VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT: VkAccessFlagBits2 = 16777216;
pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT: VkAccessFlagBits2 = 524288;
pub const VK_ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI: VkAccessFlagBits2 = 549755813888;

pub type VkSubmitFlags = VkFlags;
pub type VkSubmitFlagBits = VkFlags;
pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlagBits = 1;
pub const VK_SUBMIT_PROTECTED_BIT_KHR: VkSubmitFlagBits = 1;
pub const VK_SUBMIT_FLAG_BITS_MAX_ENUM: VkSubmitFlagBits = 2147483647;

pub type VkRenderingFlags = VkFlags;
pub type VkRenderingFlagBits = VkFlags;
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlagBits = 1;
pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlagBits = 2;
pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlagBits = 4;
pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR: VkRenderingFlagBits = 1;
pub const VK_RENDERING_SUSPENDING_BIT_KHR: VkRenderingFlagBits = 2;
pub const VK_RENDERING_RESUMING_BIT_KHR: VkRenderingFlagBits = 4;
pub const VK_RENDERING_FLAG_BITS_MAX_ENUM: VkRenderingFlagBits = 2147483647;

pub type VkFormatFeatureFlags2 = VkFlags64;
pub type VkFormatFeatureFlagBits2 = VkFlags64;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlagBits2 = 1;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 = 1;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlagBits2 = 2;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT_KHR: VkFormatFeatureFlagBits2 = 2;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlagBits2 = 4;
pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT_KHR: VkFormatFeatureFlagBits2 = 4;
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = 8;
pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = 8;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlagBits2 = 16;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = 16;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlagBits2 = 32;
pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT_KHR: VkFormatFeatureFlagBits2 = 32;
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlagBits2 = 64;
pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = 64;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = 128;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = 128;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlagBits2 = 256;
pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT_KHR: VkFormatFeatureFlagBits2 = 256;
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlagBits2 = 512;
pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = 512;
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlagBits2 = 1024;
pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT_KHR: VkFormatFeatureFlagBits2 = 1024;
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlagBits2 = 2048;
pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT_KHR: VkFormatFeatureFlagBits2 = 2048;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlagBits2 = 4096;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT_KHR: VkFormatFeatureFlagBits2 = 4096;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT: VkFormatFeatureFlagBits2 = 8192;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT: VkFormatFeatureFlagBits2 = 8192;
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlagBits2 = 16384;
pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT_KHR: VkFormatFeatureFlagBits2 = 16384;
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlagBits2 = 32768;
pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT_KHR: VkFormatFeatureFlagBits2 = 32768;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlagBits2 = 65536;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT_KHR: VkFormatFeatureFlagBits2 = 65536;
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = 131072;
pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 = 131072;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlagBits2 = 262144;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT_KHR: VkFormatFeatureFlagBits2 = 262144;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlagBits2 = 524288;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT_KHR: VkFormatFeatureFlagBits2 = 524288;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlagBits2 = 1048576;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT_KHR: VkFormatFeatureFlagBits2 = 1048576;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlagBits2 = 2097152;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT_KHR: VkFormatFeatureFlagBits2 = 2097152;
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlagBits2 = 4194304;
pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT_KHR: VkFormatFeatureFlagBits2 = 4194304;
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlagBits2 = 8388608;
pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT_KHR: VkFormatFeatureFlagBits2 = 8388608;
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = 2147483648;
pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT_KHR: VkFormatFeatureFlagBits2 = 2147483648;
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlagBits2 = 4294967296;
pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT_KHR: VkFormatFeatureFlagBits2 = 4294967296;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlagBits2 = 8589934592;
pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT_KHR: VkFormatFeatureFlagBits2 = 8589934592;
pub const VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR: VkFormatFeatureFlagBits2 = 536870912;
pub const VK_FORMAT_FEATURE_2_FRAGMENT_DENSITY_MAP_BIT_EXT: VkFormatFeatureFlagBits2 = 16777216;
pub const VK_FORMAT_FEATURE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR: VkFormatFeatureFlagBits2 = 1073741824;
pub const VK_FORMAT_FEATURE_2_LINEAR_COLOR_ATTACHMENT_BIT_NV: VkFormatFeatureFlagBits2 = 274877906944;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan13Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub robustImageAccess: VkBool32,
	pub inlineUniformBlock: VkBool32,
	pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
	pub pipelineCreationCacheControl: VkBool32,
	pub privateData: VkBool32,
	pub shaderDemoteToHelperInvocation: VkBool32,
	pub shaderTerminateInvocation: VkBool32,
	pub subgroupSizeControl: VkBool32,
	pub computeFullSubgroups: VkBool32,
	pub synchronization2: VkBool32,
	pub textureCompressionASTC_HDR: VkBool32,
	pub shaderZeroInitializeWorkgroupMemory: VkBool32,
	pub dynamicRendering: VkBool32,
	pub shaderIntegerDotProduct: VkBool32,
	pub maintenance4: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVulkan13Properties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minSubgroupSize: u32,
	pub maxSubgroupSize: u32,
	pub maxComputeWorkgroupSubgroups: u32,
	pub requiredSubgroupSizeStages: VkShaderStageFlags,
	pub maxInlineUniformBlockSize: u32,
	pub maxPerStageDescriptorInlineUniformBlocks: u32,
	pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
	pub maxDescriptorSetInlineUniformBlocks: u32,
	pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
	pub maxInlineUniformTotalSize: u32,
	pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct8BitSignedAccelerated: VkBool32,
	pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct16BitSignedAccelerated: VkBool32,
	pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct32BitSignedAccelerated: VkBool32,
	pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct64BitSignedAccelerated: VkBool32,
	pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
	pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub maxBufferSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCreationFeedback {
	pub flags: VkPipelineCreationFeedbackFlags,
	pub duration: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCreationFeedbackCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pPipelineCreationFeedback: *mut VkPipelineCreationFeedback,
	pub pipelineStageCreationFeedbackCount: u32,
	pub pPipelineStageCreationFeedbacks: *mut VkPipelineCreationFeedback,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderTerminateInvocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceToolProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub name: [i8; 256],
	pub version: [i8; 256],
	pub purposes: VkToolPurposeFlags,
	pub description: [i8; 256],
	pub layer: [i8; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderDemoteToHelperInvocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePrivateDataFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub privateData: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDevicePrivateDataCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub privateDataSlotRequestCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPrivateDataSlotCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPrivateDataSlotCreateFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pipelineCreationCacheControl: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryBarrier2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferMemoryBarrier2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageMemoryBarrier2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcStageMask: VkPipelineStageFlags2,
	pub srcAccessMask: VkAccessFlags2,
	pub dstStageMask: VkPipelineStageFlags2,
	pub dstAccessMask: VkAccessFlags2,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDependencyInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dependencyFlags: VkDependencyFlags,
	pub memoryBarrierCount: u32,
	pub pMemoryBarriers: *const VkMemoryBarrier2,
	pub bufferMemoryBarrierCount: u32,
	pub pBufferMemoryBarriers: *const VkBufferMemoryBarrier2,
	pub imageMemoryBarrierCount: u32,
	pub pImageMemoryBarriers: *const VkImageMemoryBarrier2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub semaphore: VkSemaphore,
	pub value: u64,
	pub stageMask: VkPipelineStageFlags2,
	pub deviceIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub commandBuffer: VkCommandBuffer,
	pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubmitInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSubmitFlags,
	pub waitSemaphoreInfoCount: u32,
	pub pWaitSemaphoreInfos: *const VkSemaphoreSubmitInfo,
	pub commandBufferInfoCount: u32,
	pub pCommandBufferInfos: *const VkCommandBufferSubmitInfo,
	pub signalSemaphoreInfoCount: u32,
	pub pSignalSemaphoreInfos: *const VkSemaphoreSubmitInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSynchronization2Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub synchronization2: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderZeroInitializeWorkgroupMemory: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub robustImageAccess: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferCopy2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyBufferInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcBuffer: VkBuffer,
	pub dstBuffer: VkBuffer,
	pub regionCount: u32,
	pub pRegions: *const VkBufferCopy2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageCopy2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyImageInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageCopy2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferImageCopy2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyBufferToImageInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcBuffer: VkBuffer,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkBufferImageCopy2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyImageToBufferInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstBuffer: VkBuffer,
	pub regionCount: u32,
	pub pRegions: *const VkBufferImageCopy2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageBlit2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBlitImageInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageBlit2,
	pub filter: VkFilter,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageResolve2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkResolveImageInfo2 {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcImage: VkImage,
	pub srcImageLayout: VkImageLayout,
	pub dstImage: VkImage,
	pub dstImageLayout: VkImageLayout,
	pub regionCount: u32,
	pub pRegions: *const VkImageResolve2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub subgroupSizeControl: VkBool32,
	pub computeFullSubgroups: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minSubgroupSize: u32,
	pub maxSubgroupSize: u32,
	pub maxComputeWorkgroupSubgroups: u32,
	pub requiredSubgroupSizeStages: VkShaderStageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub requiredSubgroupSize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub inlineUniformBlock: VkBool32,
	pub descriptorBindingInlineUniformBlockUpdateAfterBind: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxInlineUniformBlockSize: u32,
	pub maxPerStageDescriptorInlineUniformBlocks: u32,
	pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks: u32,
	pub maxDescriptorSetInlineUniformBlocks: u32,
	pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dataSize: u32,
	pub pData: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub maxInlineUniformBlockBindings: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub textureCompressionASTC_HDR: VkBool32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkRenderingAttachmentInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
	pub resolveMode: VkResolveModeFlagBits,
	pub resolveImageView: VkImageView,
	pub resolveImageLayout: VkImageLayout,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub clearValue: VkClearValue,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderingInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkRenderingFlags,
	pub renderArea: VkRect2D,
	pub layerCount: u32,
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkRenderingAttachmentInfo,
	pub pDepthAttachment: *const VkRenderingAttachmentInfo,
	pub pStencilAttachment: *const VkRenderingAttachmentInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRenderingCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachmentFormats: *const VkFormat,
	pub depthAttachmentFormat: VkFormat,
	pub stencilAttachmentFormat: VkFormat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub dynamicRendering: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferInheritanceRenderingInfo {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkRenderingFlags,
	pub viewMask: u32,
	pub colorAttachmentCount: u32,
	pub pColorAttachmentFormats: *const VkFormat,
	pub depthAttachmentFormat: VkFormat,
	pub stencilAttachmentFormat: VkFormat,
	pub rasterizationSamples: VkSampleCountFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderIntegerDotProduct: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub integerDotProduct8BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct8BitSignedAccelerated: VkBool32,
	pub integerDotProduct8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProduct4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct16BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct16BitSignedAccelerated: VkBool32,
	pub integerDotProduct16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct32BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct32BitSignedAccelerated: VkBool32,
	pub integerDotProduct32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProduct64BitUnsignedAccelerated: VkBool32,
	pub integerDotProduct64BitSignedAccelerated: VkBool32,
	pub integerDotProduct64BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitSignedAccelerated: VkBool32,
	pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub storageTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub storageTexelBufferOffsetSingleTexelAlignment: VkBool32,
	pub uniformTexelBufferOffsetAlignmentBytes: VkDeviceSize,
	pub uniformTexelBufferOffsetSingleTexelAlignment: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFormatProperties3 {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub linearTilingFeatures: VkFormatFeatureFlags2,
	pub optimalTilingFeatures: VkFormatFeatureFlags2,
	pub bufferFeatures: VkFormatFeatureFlags2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMaintenance4Features {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maintenance4: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMaintenance4Properties {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxBufferSize: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceBufferMemoryRequirements {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pCreateInfo: *const VkBufferCreateInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceImageMemoryRequirements {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pCreateInfo: *const VkImageCreateInfo,
	pub planeAspect: VkImageAspectFlagBits,
}

pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;

pub type PFN_vkCreatePrivateDataSlot =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;

pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "C" fn(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkSetPrivateData = unsafe extern "C" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;

pub type PFN_vkGetPrivateData = unsafe extern "C" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);

pub type PFN_vkCmdSetEvent2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);

pub type PFN_vkCmdResetEvent2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);

pub type PFN_vkCmdWaitEvents2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);

pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);

pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);

pub type PFN_vkQueueSubmit2 = unsafe extern "C" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;

pub type PFN_vkCmdCopyBuffer2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);

pub type PFN_vkCmdCopyImage2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);

pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);

pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);

pub type PFN_vkCmdBlitImage2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);

pub type PFN_vkCmdResolveImage2 = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);

pub type PFN_vkCmdBeginRendering = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);

pub type PFN_vkCmdEndRendering = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

pub type PFN_vkCmdSetCullMode = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);

pub type PFN_vkCmdSetFrontFace = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);

pub type PFN_vkCmdSetPrimitiveTopology = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);

pub type PFN_vkCmdSetViewportWithCount = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);

pub type PFN_vkCmdSetScissorWithCount = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);

pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *const VkBuffer,
	pOffsets: *const VkDeviceSize,
	pSizes: *const VkDeviceSize,
	pStrides: *const VkDeviceSize,
);

pub type PFN_vkCmdSetDepthTestEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);

pub type PFN_vkCmdSetDepthWriteEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);

pub type PFN_vkCmdSetDepthCompareOp = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);

pub type PFN_vkCmdSetDepthBoundsTestEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);

pub type PFN_vkCmdSetStencilTestEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);

pub type PFN_vkCmdSetStencilOp =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);

pub type PFN_vkCmdSetRasterizerDiscardEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);

pub type PFN_vkCmdSetDepthBiasEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);

pub type PFN_vkCmdSetPrimitiveRestartEnable = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);

pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetDeviceImageSparseMemoryRequirements =
	unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceToolProperties(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;

	pub fn vkCreatePrivateDataSlot(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;

	pub fn vkDestroyPrivateDataSlot(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);

	pub fn vkSetPrivateData(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;

	pub fn vkGetPrivateData(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);

	pub fn vkCmdSetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);

	pub fn vkCmdResetEvent2(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);

	pub fn vkCmdWaitEvents2(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);

	pub fn vkCmdPipelineBarrier2(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);

	pub fn vkCmdWriteTimestamp2(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);

	pub fn vkQueueSubmit2(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;

	pub fn vkCmdCopyBuffer2(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);

	pub fn vkCmdCopyImage2(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);

	pub fn vkCmdCopyBufferToImage2(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);

	pub fn vkCmdCopyImageToBuffer2(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);

	pub fn vkCmdBlitImage2(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);

	pub fn vkCmdResolveImage2(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);

	pub fn vkCmdBeginRendering(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);

	pub fn vkCmdEndRendering(commandBuffer: VkCommandBuffer);

	pub fn vkCmdSetCullMode(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);

	pub fn vkCmdSetFrontFace(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);

	pub fn vkCmdSetPrimitiveTopology(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);

	pub fn vkCmdSetViewportWithCount(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);

	pub fn vkCmdSetScissorWithCount(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);

	pub fn vkCmdBindVertexBuffers2(
		commandBuffer: VkCommandBuffer,
		firstBinding: u32,
		bindingCount: u32,
		pBuffers: *const VkBuffer,
		pOffsets: *const VkDeviceSize,
		pSizes: *const VkDeviceSize,
		pStrides: *const VkDeviceSize,
	);

	pub fn vkCmdSetDepthTestEnable(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);

	pub fn vkCmdSetDepthWriteEnable(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);

	pub fn vkCmdSetDepthCompareOp(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);

	pub fn vkCmdSetDepthBoundsTestEnable(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);

	pub fn vkCmdSetStencilTestEnable(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);

	pub fn vkCmdSetStencilOp(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);

	pub fn vkCmdSetRasterizerDiscardEnable(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);

	pub fn vkCmdSetDepthBiasEnable(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);

	pub fn vkCmdSetPrimitiveRestartEnable(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);

	pub fn vkGetDeviceBufferMemoryRequirements(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetDeviceImageMemoryRequirements(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetDeviceImageSparseMemoryRequirements(
		device: VkDevice,
		pInfo: *const VkDeviceImageMemoryRequirements,
		pSparseMemoryRequirementCount: *mut u32,
		pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceKHR_T {
	_unused: [u8; 0],
}

pub type VkSurfaceKHR = *mut VkSurfaceKHR_T;

pub type VkPresentModeKHR = u32;
pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = 0;
pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = 1;
pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;
pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = 3;
pub const VK_PRESENT_MODE_SHARED_DEMAND_REFRESH_KHR: VkPresentModeKHR = 1000111000;
pub const VK_PRESENT_MODE_SHARED_CONTINUOUS_REFRESH_KHR: VkPresentModeKHR = 1000111001;
pub const VK_PRESENT_MODE_MAX_ENUM_KHR: VkPresentModeKHR = 2147483647;

pub type VkColorSpaceKHR = u32;
pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
pub const VK_COLOR_SPACE_DISPLAY_P3_NONLINEAR_EXT: VkColorSpaceKHR = 1000104001;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_LINEAR_EXT: VkColorSpaceKHR = 1000104002;
pub const VK_COLOR_SPACE_DISPLAY_P3_LINEAR_EXT: VkColorSpaceKHR = 1000104003;
pub const VK_COLOR_SPACE_DCI_P3_NONLINEAR_EXT: VkColorSpaceKHR = 1000104004;
pub const VK_COLOR_SPACE_BT709_LINEAR_EXT: VkColorSpaceKHR = 1000104005;
pub const VK_COLOR_SPACE_BT709_NONLINEAR_EXT: VkColorSpaceKHR = 1000104006;
pub const VK_COLOR_SPACE_BT2020_LINEAR_EXT: VkColorSpaceKHR = 1000104007;
pub const VK_COLOR_SPACE_HDR10_ST2084_EXT: VkColorSpaceKHR = 1000104008;
pub const VK_COLOR_SPACE_DOLBYVISION_EXT: VkColorSpaceKHR = 1000104009;
pub const VK_COLOR_SPACE_HDR10_HLG_EXT: VkColorSpaceKHR = 1000104010;
pub const VK_COLOR_SPACE_ADOBERGB_LINEAR_EXT: VkColorSpaceKHR = 1000104011;
pub const VK_COLOR_SPACE_ADOBERGB_NONLINEAR_EXT: VkColorSpaceKHR = 1000104012;
pub const VK_COLOR_SPACE_PASS_THROUGH_EXT: VkColorSpaceKHR = 1000104013;
pub const VK_COLOR_SPACE_EXTENDED_SRGB_NONLINEAR_EXT: VkColorSpaceKHR = 1000104014;
pub const VK_COLOR_SPACE_DISPLAY_NATIVE_AMD: VkColorSpaceKHR = 1000213000;
pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
pub const VK_COLOR_SPACE_DCI_P3_LINEAR_EXT: VkColorSpaceKHR = 1000104003;
pub const VK_COLOR_SPACE_MAX_ENUM_KHR: VkColorSpaceKHR = 2147483647;

pub type VkSurfaceTransformFlagsKHR = VkFlags;
pub type VkSurfaceTransformFlagBitsKHR = VkFlags;
pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 1;
pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 2;
pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 4;
pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 8;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 16;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 32;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 64;
pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 128;
pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagBitsKHR = 256;
pub const VK_SURFACE_TRANSFORM_FLAG_BITS_MAX_ENUM_KHR: VkSurfaceTransformFlagBitsKHR = 2147483647;

pub type VkCompositeAlphaFlagsKHR = VkFlags;
pub type VkCompositeAlphaFlagBitsKHR = VkFlags;
pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 1;
pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 2;
pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 4;
pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagBitsKHR = 8;
pub const VK_COMPOSITE_ALPHA_FLAG_BITS_MAX_ENUM_KHR: VkCompositeAlphaFlagBitsKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceCapabilitiesKHR {
	pub minImageCount: u32,
	pub maxImageCount: u32,
	pub currentExtent: VkExtent2D,
	pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D,
	pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagBitsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceFormatKHR {
	pub format: VkFormat,
	pub colorSpace: VkColorSpaceKHR,
}

pub type PFN_vkDestroySurfaceKHR = unsafe extern "C" fn(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceFormatsKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfacePresentModesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkDestroySurfaceKHR(instance: VkInstance, surface: VkSurfaceKHR, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetPhysicalDeviceSurfaceSupportKHR(physicalDevice: VkPhysicalDevice, queueFamilyIndex: u32, surface: VkSurfaceKHR, pSupported: *mut VkBool32) -> VkResult;

	pub fn vkGetPhysicalDeviceSurfaceCapabilitiesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilitiesKHR) -> VkResult;

	pub fn vkGetPhysicalDeviceSurfaceFormatsKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormatKHR) -> VkResult;

	pub fn vkGetPhysicalDeviceSurfacePresentModesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pPresentModeCount: *mut u32, pPresentModes: *mut VkPresentModeKHR) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSwapchainKHR_T {
	_unused: [u8; 0],
}

pub type VkSwapchainKHR = *mut VkSwapchainKHR_T;

pub type VkSwapchainCreateFlagsKHR = VkFlags;
pub type VkSwapchainCreateFlagBitsKHR = VkFlags;
pub const VK_SWAPCHAIN_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR: VkSwapchainCreateFlagBitsKHR = 1;
pub const VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR: VkSwapchainCreateFlagBitsKHR = 2;
pub const VK_SWAPCHAIN_CREATE_MUTABLE_FORMAT_BIT_KHR: VkSwapchainCreateFlagBitsKHR = 4;
pub const VK_SWAPCHAIN_CREATE_FLAG_BITS_MAX_ENUM_KHR: VkSwapchainCreateFlagBitsKHR = 2147483647;

pub type VkDeviceGroupPresentModeFlagsKHR = VkFlags;
pub type VkDeviceGroupPresentModeFlagBitsKHR = VkFlags;
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = 1;
pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = 2;
pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = 4;
pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: VkDeviceGroupPresentModeFlagBitsKHR = 8;
pub const VK_DEVICE_GROUP_PRESENT_MODE_FLAG_BITS_MAX_ENUM_KHR: VkDeviceGroupPresentModeFlagBitsKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSwapchainCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkSwapchainCreateFlagsKHR,
	pub surface: VkSurfaceKHR,
	pub minImageCount: u32,
	pub imageFormat: VkFormat,
	pub imageColorSpace: VkColorSpaceKHR,
	pub imageExtent: VkExtent2D,
	pub imageArrayLayers: u32,
	pub imageUsage: VkImageUsageFlags,
	pub imageSharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
	pub preTransform: VkSurfaceTransformFlagBitsKHR,
	pub compositeAlpha: VkCompositeAlphaFlagBitsKHR,
	pub presentMode: VkPresentModeKHR,
	pub clipped: VkBool32,
	pub oldSwapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore,
	pub swapchainCount: u32,
	pub pSwapchains: *const VkSwapchainKHR,
	pub pImageIndices: *const u32,
	pub pResults: *mut VkResult,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageSwapchainCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchain: VkSwapchainKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindImageMemorySwapchainInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchain: VkSwapchainKHR,
	pub imageIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAcquireNextImageInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchain: VkSwapchainKHR,
	pub timeout: u64,
	pub semaphore: VkSemaphore,
	pub fence: VkFence,
	pub deviceMask: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub presentMask: [u32; 32],
	pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupPresentInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchainCount: u32,
	pub pDeviceMasks: *const u32,
	pub mode: VkDeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub modes: VkDeviceGroupPresentModeFlagsKHR,
}

pub type PFN_vkCreateSwapchainKHR =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;

pub type PFN_vkDestroySwapchainKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetSwapchainImagesKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;

pub type PFN_vkAcquireNextImageKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;

pub type PFN_vkQueuePresentKHR = unsafe extern "C" fn(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;

pub type PFN_vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "C" fn(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult;

pub type PFN_vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "C" fn(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;

pub type PFN_vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;

pub type PFN_vkAcquireNextImage2KHR = unsafe extern "C" fn(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateSwapchainKHR(device: VkDevice, pCreateInfo: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchain: *mut VkSwapchainKHR) -> VkResult;

	pub fn vkDestroySwapchainKHR(device: VkDevice, swapchain: VkSwapchainKHR, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetSwapchainImagesKHR(device: VkDevice, swapchain: VkSwapchainKHR, pSwapchainImageCount: *mut u32, pSwapchainImages: *mut VkImage) -> VkResult;

	pub fn vkAcquireNextImageKHR(device: VkDevice, swapchain: VkSwapchainKHR, timeout: u64, semaphore: VkSemaphore, fence: VkFence, pImageIndex: *mut u32) -> VkResult;

	pub fn vkQueuePresentKHR(queue: VkQueue, pPresentInfo: *const VkPresentInfoKHR) -> VkResult;

	pub fn vkGetDeviceGroupPresentCapabilitiesKHR(device: VkDevice, pDeviceGroupPresentCapabilities: *mut VkDeviceGroupPresentCapabilitiesKHR) -> VkResult;

	pub fn vkGetDeviceGroupSurfacePresentModesKHR(device: VkDevice, surface: VkSurfaceKHR, pModes: *mut VkDeviceGroupPresentModeFlagsKHR) -> VkResult;

	pub fn vkGetPhysicalDevicePresentRectanglesKHR(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pRectCount: *mut u32, pRects: *mut VkRect2D) -> VkResult;

	pub fn vkAcquireNextImage2KHR(device: VkDevice, pAcquireInfo: *const VkAcquireNextImageInfoKHR, pImageIndex: *mut u32) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayKHR_T {
	_unused: [u8; 0],
}

pub type VkDisplayKHR = *mut VkDisplayKHR_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayModeKHR_T {
	_unused: [u8; 0],
}

pub type VkDisplayModeKHR = *mut VkDisplayModeKHR_T;

pub type VkDisplayModeCreateFlagsKHR = VkFlags;

pub type VkDisplayPlaneAlphaFlagsKHR = VkFlags;
pub type VkDisplayPlaneAlphaFlagBitsKHR = u32;
pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 1;
pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 2;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 4;
pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 8;
pub const VK_DISPLAY_PLANE_ALPHA_FLAG_BITS_MAX_ENUM_KHR: VkDisplayPlaneAlphaFlagBitsKHR = 2147483647;

pub type VkDisplaySurfaceCreateFlagsKHR = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayModeParametersKHR {
	pub visibleRegion: VkExtent2D,
	pub refreshRate: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayModeCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDisplayModeCreateFlagsKHR,
	pub parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayModePropertiesKHR {
	pub displayMode: VkDisplayModeKHR,
	pub parameters: VkDisplayModeParametersKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPlaneCapabilitiesKHR {
	pub supportedAlpha: VkDisplayPlaneAlphaFlagsKHR,
	pub minSrcPosition: VkOffset2D,
	pub maxSrcPosition: VkOffset2D,
	pub minSrcExtent: VkExtent2D,
	pub maxSrcExtent: VkExtent2D,
	pub minDstPosition: VkOffset2D,
	pub maxDstPosition: VkOffset2D,
	pub minDstExtent: VkExtent2D,
	pub maxDstExtent: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPlanePropertiesKHR {
	pub currentDisplay: VkDisplayKHR,
	pub currentStackIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPropertiesKHR {
	pub display: VkDisplayKHR,
	pub displayName: *const i8,
	pub physicalDimensions: VkExtent2D,
	pub physicalResolution: VkExtent2D,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub planeReorderPossible: VkBool32,
	pub persistentContent: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplaySurfaceCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDisplaySurfaceCreateFlagsKHR,
	pub displayMode: VkDisplayModeKHR,
	pub planeIndex: u32,
	pub planeStackIndex: u32,
	pub transform: VkSurfaceTransformFlagBitsKHR,
	pub globalAlpha: f32,
	pub alphaMode: VkDisplayPlaneAlphaFlagBitsKHR,
	pub imageExtent: VkExtent2D,
}

pub type PFN_vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;

pub type PFN_vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;

pub type PFN_vkGetDisplayModePropertiesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;

pub type PFN_vkCreateDisplayModeKHR = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice,
	display: VkDisplayKHR,
	pCreateInfo: *const VkDisplayModeCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pMode: *mut VkDisplayModeKHR,
) -> VkResult;

pub type PFN_vkGetDisplayPlaneCapabilitiesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;

pub type PFN_vkCreateDisplayPlaneSurfaceKHR =
	unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceDisplayPropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPropertiesKHR) -> VkResult;

	pub fn vkGetPhysicalDeviceDisplayPlanePropertiesKHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlanePropertiesKHR) -> VkResult;

	pub fn vkGetDisplayPlaneSupportedDisplaysKHR(physicalDevice: VkPhysicalDevice, planeIndex: u32, pDisplayCount: *mut u32, pDisplays: *mut VkDisplayKHR) -> VkResult;

	pub fn vkGetDisplayModePropertiesKHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModePropertiesKHR) -> VkResult;

	pub fn vkCreateDisplayModeKHR(
		physicalDevice: VkPhysicalDevice,
		display: VkDisplayKHR,
		pCreateInfo: *const VkDisplayModeCreateInfoKHR,
		pAllocator: *const VkAllocationCallbacks,
		pMode: *mut VkDisplayModeKHR,
	) -> VkResult;

	pub fn vkGetDisplayPlaneCapabilitiesKHR(physicalDevice: VkPhysicalDevice, mode: VkDisplayModeKHR, planeIndex: u32, pCapabilities: *mut VkDisplayPlaneCapabilitiesKHR) -> VkResult;

	pub fn vkCreateDisplayPlaneSurfaceKHR(instance: VkInstance, pCreateInfo: *const VkDisplaySurfaceCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPresentInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcRect: VkRect2D,
	pub dstRect: VkRect2D,
	pub persistent: VkBool32,
}

pub type PFN_vkCreateSharedSwapchainsKHR =
	unsafe extern "C" fn(device: VkDevice, swapchainCount: u32, pCreateInfos: *const VkSwapchainCreateInfoKHR, pAllocator: *const VkAllocationCallbacks, pSwapchains: *mut VkSwapchainKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateSharedSwapchainsKHR(
		device: VkDevice,
		swapchainCount: u32,
		pCreateInfos: *const VkSwapchainCreateInfoKHR,
		pAllocator: *const VkAllocationCallbacks,
		pSwapchains: *mut VkSwapchainKHR,
	) -> VkResult;
}

pub type VkRenderingFlagsKHR = VkRenderingFlags;
pub use VkRenderingFlagBits as VkRenderingFlagBitsKHR;
pub type VkRenderingInfoKHR = VkRenderingInfo;
pub type VkRenderingAttachmentInfoKHR = VkRenderingAttachmentInfo;
pub type VkPipelineRenderingCreateInfoKHR = VkPipelineRenderingCreateInfo;
pub type VkPhysicalDeviceDynamicRenderingFeaturesKHR = VkPhysicalDeviceDynamicRenderingFeatures;
pub type VkCommandBufferInheritanceRenderingInfoKHR = VkCommandBufferInheritanceRenderingInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
	pub shadingRateAttachmentTexelSize: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentSampleCountInfoAMD {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub colorAttachmentCount: u32,
	pub pColorAttachmentSamples: *const VkSampleCountFlagBits,
	pub depthStencilAttachmentSamples: VkSampleCountFlagBits,
}

pub type VkAttachmentSampleCountInfoNV = VkAttachmentSampleCountInfoAMD;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub perViewAttributes: VkBool32,
	pub perViewAttributesPositionXOnly: VkBool32,
}

pub type PFN_vkCmdBeginRenderingKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);

pub type PFN_vkCmdEndRenderingKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdBeginRenderingKHR(commandBuffer: VkCommandBuffer, pRenderingInfo: *const VkRenderingInfo);

	pub fn vkCmdEndRenderingKHR(commandBuffer: VkCommandBuffer);
}

pub type VkRenderPassMultiviewCreateInfoKHR = VkRenderPassMultiviewCreateInfo;
pub type VkPhysicalDeviceMultiviewFeaturesKHR = VkPhysicalDeviceMultiviewFeatures;
pub type VkPhysicalDeviceMultiviewPropertiesKHR = VkPhysicalDeviceMultiviewProperties;
pub type VkPhysicalDeviceFeatures2KHR = VkPhysicalDeviceFeatures2;
pub type VkPhysicalDeviceProperties2KHR = VkPhysicalDeviceProperties2;
pub type VkFormatProperties2KHR = VkFormatProperties2;
pub type VkImageFormatProperties2KHR = VkImageFormatProperties2;
pub type VkPhysicalDeviceImageFormatInfo2KHR = VkPhysicalDeviceImageFormatInfo2;
pub type VkQueueFamilyProperties2KHR = VkQueueFamilyProperties2;
pub type VkPhysicalDeviceMemoryProperties2KHR = VkPhysicalDeviceMemoryProperties2;
pub type VkSparseImageFormatProperties2KHR = VkSparseImageFormatProperties2;
pub type VkPhysicalDeviceSparseImageFormatInfo2KHR = VkPhysicalDeviceSparseImageFormatInfo2;

pub type PFN_vkGetPhysicalDeviceFeatures2KHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);

pub type PFN_vkGetPhysicalDeviceProperties2KHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);

pub type PFN_vkGetPhysicalDeviceFormatProperties2KHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);

pub type PFN_vkGetPhysicalDeviceImageFormatProperties2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2, pImageFormatProperties: *mut VkImageFormatProperties2) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);

pub type PFN_vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);

pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties2);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceFeatures2KHR(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures2);

	pub fn vkGetPhysicalDeviceProperties2KHR(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties2);

	pub fn vkGetPhysicalDeviceFormatProperties2KHR(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties2);

	pub fn vkGetPhysicalDeviceImageFormatProperties2KHR(
		physicalDevice: VkPhysicalDevice,
		pImageFormatInfo: *const VkPhysicalDeviceImageFormatInfo2,
		pImageFormatProperties: *mut VkImageFormatProperties2,
	) -> VkResult;

	pub fn vkGetPhysicalDeviceQueueFamilyProperties2KHR(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties2);

	pub fn vkGetPhysicalDeviceMemoryProperties2KHR(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties2);

	pub fn vkGetPhysicalDeviceSparseImageFormatProperties2KHR(
		physicalDevice: VkPhysicalDevice,
		pFormatInfo: *const VkPhysicalDeviceSparseImageFormatInfo2,
		pPropertyCount: *mut u32,
		pProperties: *mut VkSparseImageFormatProperties2,
	);
}

pub type VkPeerMemoryFeatureFlagsKHR = VkPeerMemoryFeatureFlags;
pub use VkPeerMemoryFeatureFlagBits as VkPeerMemoryFeatureFlagBitsKHR;
pub type VkMemoryAllocateFlagsKHR = VkMemoryAllocateFlags;
pub use VkMemoryAllocateFlagBits as VkMemoryAllocateFlagBitsKHR;
pub type VkMemoryAllocateFlagsInfoKHR = VkMemoryAllocateFlagsInfo;
pub type VkDeviceGroupRenderPassBeginInfoKHR = VkDeviceGroupRenderPassBeginInfo;
pub type VkDeviceGroupCommandBufferBeginInfoKHR = VkDeviceGroupCommandBufferBeginInfo;
pub type VkDeviceGroupSubmitInfoKHR = VkDeviceGroupSubmitInfo;
pub type VkDeviceGroupBindSparseInfoKHR = VkDeviceGroupBindSparseInfo;
pub type VkBindBufferMemoryDeviceGroupInfoKHR = VkBindBufferMemoryDeviceGroupInfo;
pub type VkBindImageMemoryDeviceGroupInfoKHR = VkBindImageMemoryDeviceGroupInfo;

pub type PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR =
	unsafe extern "C" fn(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);

pub type PFN_vkCmdSetDeviceMaskKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, deviceMask: u32);

pub type PFN_vkCmdDispatchBaseKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetDeviceGroupPeerMemoryFeaturesKHR(device: VkDevice, heapIndex: u32, localDeviceIndex: u32, remoteDeviceIndex: u32, pPeerMemoryFeatures: *mut VkPeerMemoryFeatureFlags);

	pub fn vkCmdSetDeviceMaskKHR(commandBuffer: VkCommandBuffer, deviceMask: u32);

	pub fn vkCmdDispatchBaseKHR(commandBuffer: VkCommandBuffer, baseGroupX: u32, baseGroupY: u32, baseGroupZ: u32, groupCountX: u32, groupCountY: u32, groupCountZ: u32);
}

pub type VkCommandPoolTrimFlagsKHR = VkCommandPoolTrimFlags;

pub type PFN_vkTrimCommandPoolKHR = unsafe extern "C" fn(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkTrimCommandPoolKHR(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolTrimFlags);
}

pub type VkPhysicalDeviceGroupPropertiesKHR = VkPhysicalDeviceGroupProperties;
pub type VkDeviceGroupDeviceCreateInfoKHR = VkDeviceGroupDeviceCreateInfo;

pub type PFN_vkEnumeratePhysicalDeviceGroupsKHR =
	unsafe extern "C" fn(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkEnumeratePhysicalDeviceGroupsKHR(instance: VkInstance, pPhysicalDeviceGroupCount: *mut u32, pPhysicalDeviceGroupProperties: *mut VkPhysicalDeviceGroupProperties) -> VkResult;
}

pub type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlags;

pub use VkExternalMemoryHandleTypeFlagBits as VkExternalMemoryHandleTypeFlagBitsKHR;

pub type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlags;

pub use VkExternalMemoryFeatureFlagBits as VkExternalMemoryFeatureFlagBitsKHR;

pub type VkExternalMemoryPropertiesKHR = VkExternalMemoryProperties;
pub type VkPhysicalDeviceExternalImageFormatInfoKHR = VkPhysicalDeviceExternalImageFormatInfo;
pub type VkExternalImageFormatPropertiesKHR = VkExternalImageFormatProperties;
pub type VkPhysicalDeviceExternalBufferInfoKHR = VkPhysicalDeviceExternalBufferInfo;
pub type VkExternalBufferPropertiesKHR = VkExternalBufferProperties;
pub type VkPhysicalDeviceIDPropertiesKHR = VkPhysicalDeviceIDProperties;

pub type PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo, pExternalBufferProperties: *mut VkExternalBufferProperties);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceExternalBufferPropertiesKHR(
		physicalDevice: VkPhysicalDevice,
		pExternalBufferInfo: *const VkPhysicalDeviceExternalBufferInfo,
		pExternalBufferProperties: *mut VkExternalBufferProperties,
	);
}

pub type VkExternalMemoryImageCreateInfoKHR = VkExternalMemoryImageCreateInfo;
pub type VkExternalMemoryBufferCreateInfoKHR = VkExternalMemoryBufferCreateInfo;
pub type VkExportMemoryAllocateInfoKHR = VkExportMemoryAllocateInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImportMemoryFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub fd: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryFdPropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryGetFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

pub type PFN_vkGetMemoryFdKHR = unsafe extern "C" fn(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut i32) -> VkResult;

pub type PFN_vkGetMemoryFdPropertiesKHR =
	unsafe extern "C" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, fd: i32, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetMemoryFdKHR(device: VkDevice, pGetFdInfo: *const VkMemoryGetFdInfoKHR, pFd: *mut i32) -> VkResult;

	pub fn vkGetMemoryFdPropertiesKHR(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, fd: i32, pMemoryFdProperties: *mut VkMemoryFdPropertiesKHR) -> VkResult;
}

pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlags;
pub use VkExternalSemaphoreHandleTypeFlagBits as VkExternalSemaphoreHandleTypeFlagBitsKHR;
pub type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlags;
pub use VkExternalSemaphoreFeatureFlagBits as VkExternalSemaphoreFeatureFlagBitsKHR;
pub type VkPhysicalDeviceExternalSemaphoreInfoKHR = VkPhysicalDeviceExternalSemaphoreInfo;
pub type VkExternalSemaphorePropertiesKHR = VkExternalSemaphoreProperties;

pub type PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo, pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceExternalSemaphorePropertiesKHR(
		physicalDevice: VkPhysicalDevice,
		pExternalSemaphoreInfo: *const VkPhysicalDeviceExternalSemaphoreInfo,
		pExternalSemaphoreProperties: *mut VkExternalSemaphoreProperties,
	);
}

pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlags;
pub use VkSemaphoreImportFlagBits as VkSemaphoreImportFlagBitsKHR;
pub type VkExportSemaphoreCreateInfoKHR = VkExportSemaphoreCreateInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImportSemaphoreFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub semaphore: VkSemaphore,
	pub flags: VkSemaphoreImportFlags,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
	pub fd: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSemaphoreGetFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub semaphore: VkSemaphore,
	pub handleType: VkExternalSemaphoreHandleTypeFlagBits,
}

pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "C" fn(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult;

pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "C" fn(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut i32) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkImportSemaphoreFdKHR(device: VkDevice, pImportSemaphoreFdInfo: *const VkImportSemaphoreFdInfoKHR) -> VkResult;

	pub fn vkGetSemaphoreFdKHR(device: VkDevice, pGetFdInfo: *const VkSemaphoreGetFdInfoKHR, pFd: *mut i32) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxPushDescriptors: u32,
}

pub type PFN_vkCmdPushDescriptorSetKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	pipelineBindPoint: VkPipelineBindPoint,
	layout: VkPipelineLayout,
	set: u32,
	descriptorWriteCount: u32,
	pDescriptorWrites: *const VkWriteDescriptorSet,
);

pub type PFN_vkCmdPushDescriptorSetWithTemplateKHR =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, layout: VkPipelineLayout, set: u32, pData: *const ());

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdPushDescriptorSetKHR(
		commandBuffer: VkCommandBuffer,
		pipelineBindPoint: VkPipelineBindPoint,
		layout: VkPipelineLayout,
		set: u32,
		descriptorWriteCount: u32,
		pDescriptorWrites: *const VkWriteDescriptorSet,
	);

	pub fn vkCmdPushDescriptorSetWithTemplateKHR(commandBuffer: VkCommandBuffer, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, layout: VkPipelineLayout, set: u32, pData: *const ());
}

pub type VkPhysicalDeviceShaderFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
pub type VkPhysicalDeviceFloat16Int8FeaturesKHR = VkPhysicalDeviceShaderFloat16Int8Features;
pub type VkPhysicalDevice16BitStorageFeaturesKHR = VkPhysicalDevice16BitStorageFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRectLayerKHR {
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
	pub layer: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentRegionKHR {
	pub rectangleCount: u32,
	pub pRectangles: *const VkRectLayerKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentRegionsKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchainCount: u32,
	pub pRegions: *const VkPresentRegionKHR,
}

pub type VkDescriptorUpdateTemplateKHR = VkDescriptorUpdateTemplate;
pub use VkDescriptorUpdateTemplateType as VkDescriptorUpdateTemplateTypeKHR;
pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags;
pub type VkDescriptorUpdateTemplateEntryKHR = VkDescriptorUpdateTemplateEntry;
pub type VkDescriptorUpdateTemplateCreateInfoKHR = VkDescriptorUpdateTemplateCreateInfo;

pub type PFN_vkCreateDescriptorUpdateTemplateKHR = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
) -> VkResult;

pub type PFN_vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "C" fn(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "C" fn(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const ());

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateDescriptorUpdateTemplateKHR(
		device: VkDevice,
		pCreateInfo: *const VkDescriptorUpdateTemplateCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pDescriptorUpdateTemplate: *mut VkDescriptorUpdateTemplate,
	) -> VkResult;

	pub fn vkDestroyDescriptorUpdateTemplateKHR(device: VkDevice, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pAllocator: *const VkAllocationCallbacks);

	pub fn vkUpdateDescriptorSetWithTemplateKHR(device: VkDevice, descriptorSet: VkDescriptorSet, descriptorUpdateTemplate: VkDescriptorUpdateTemplate, pData: *const ());
}

pub type VkPhysicalDeviceImagelessFramebufferFeaturesKHR = VkPhysicalDeviceImagelessFramebufferFeatures;
pub type VkFramebufferAttachmentsCreateInfoKHR = VkFramebufferAttachmentsCreateInfo;
pub type VkFramebufferAttachmentImageInfoKHR = VkFramebufferAttachmentImageInfo;
pub type VkRenderPassAttachmentBeginInfoKHR = VkRenderPassAttachmentBeginInfo;
pub type VkRenderPassCreateInfo2KHR = VkRenderPassCreateInfo2;
pub type VkAttachmentDescription2KHR = VkAttachmentDescription2;
pub type VkAttachmentReference2KHR = VkAttachmentReference2;
pub type VkSubpassDescription2KHR = VkSubpassDescription2;
pub type VkSubpassDependency2KHR = VkSubpassDependency2;
pub type VkSubpassBeginInfoKHR = VkSubpassBeginInfo;
pub type VkSubpassEndInfoKHR = VkSubpassEndInfo;

pub type PFN_vkCreateRenderPass2KHR =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

pub type PFN_vkCmdBeginRenderPass2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);

pub type PFN_vkCmdNextSubpass2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);

pub type PFN_vkCmdEndRenderPass2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateRenderPass2KHR(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo2, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;

	pub fn vkCmdBeginRenderPass2KHR(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, pSubpassBeginInfo: *const VkSubpassBeginInfo);

	pub fn vkCmdNextSubpass2KHR(commandBuffer: VkCommandBuffer, pSubpassBeginInfo: *const VkSubpassBeginInfo, pSubpassEndInfo: *const VkSubpassEndInfo);

	pub fn vkCmdEndRenderPass2KHR(commandBuffer: VkCommandBuffer, pSubpassEndInfo: *const VkSubpassEndInfo);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub sharedPresentSupportedUsageFlags: VkImageUsageFlags,
}

pub type PFN_vkGetSwapchainStatusKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetSwapchainStatusKHR(device: VkDevice, swapchain: VkSwapchainKHR) -> VkResult;
}

pub type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlags;
pub use VkExternalFenceHandleTypeFlagBits as VkExternalFenceHandleTypeFlagBitsKHR;
pub type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlags;
pub use VkExternalFenceFeatureFlagBits as VkExternalFenceFeatureFlagBitsKHR;
pub type VkPhysicalDeviceExternalFenceInfoKHR = VkPhysicalDeviceExternalFenceInfo;
pub type VkExternalFencePropertiesKHR = VkExternalFenceProperties;

pub type PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo, pExternalFenceProperties: *mut VkExternalFenceProperties);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceExternalFencePropertiesKHR(
		physicalDevice: VkPhysicalDevice,
		pExternalFenceInfo: *const VkPhysicalDeviceExternalFenceInfo,
		pExternalFenceProperties: *mut VkExternalFenceProperties,
	);
}

pub type VkFenceImportFlagsKHR = VkFenceImportFlags;
pub use VkFenceImportFlagBits as VkFenceImportFlagBitsKHR;
pub type VkExportFenceCreateInfoKHR = VkExportFenceCreateInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImportFenceFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub fence: VkFence,
	pub flags: VkFenceImportFlags,
	pub handleType: VkExternalFenceHandleTypeFlagBits,
	pub fd: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFenceGetFdInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub fence: VkFence,
	pub handleType: VkExternalFenceHandleTypeFlagBits,
}

pub type PFN_vkImportFenceFdKHR = unsafe extern "C" fn(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;

pub type PFN_vkGetFenceFdKHR = unsafe extern "C" fn(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut i32) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkImportFenceFdKHR(device: VkDevice, pImportFenceFdInfo: *const VkImportFenceFdInfoKHR) -> VkResult;

	pub fn vkGetFenceFdKHR(device: VkDevice, pGetFdInfo: *const VkFenceGetFdInfoKHR, pFd: *mut i32) -> VkResult;
}

pub type VkPerformanceCounterUnitKHR = u32;
pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: VkPerformanceCounterUnitKHR = 0;
pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: VkPerformanceCounterUnitKHR = 1;
pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: VkPerformanceCounterUnitKHR = 2;
pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: VkPerformanceCounterUnitKHR = 3;
pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: VkPerformanceCounterUnitKHR = 4;
pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: VkPerformanceCounterUnitKHR = 5;
pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: VkPerformanceCounterUnitKHR = 6;
pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: VkPerformanceCounterUnitKHR = 7;
pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: VkPerformanceCounterUnitKHR = 8;
pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: VkPerformanceCounterUnitKHR = 9;
pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: VkPerformanceCounterUnitKHR = 10;
pub const VK_PERFORMANCE_COUNTER_UNIT_MAX_ENUM_KHR: VkPerformanceCounterUnitKHR = 2147483647;

pub type VkPerformanceCounterScopeKHR = u32;
pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR = 0;
pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR = 1;
pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR = 2;
pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR = 0;
pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR = 1;
pub const VK_QUERY_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR = 2;
pub const VK_PERFORMANCE_COUNTER_SCOPE_MAX_ENUM_KHR: VkPerformanceCounterScopeKHR = 2147483647;

pub type VkPerformanceCounterStorageKHR = u32;
pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: VkPerformanceCounterStorageKHR = 0;
pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: VkPerformanceCounterStorageKHR = 1;
pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: VkPerformanceCounterStorageKHR = 2;
pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: VkPerformanceCounterStorageKHR = 3;
pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: VkPerformanceCounterStorageKHR = 4;
pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: VkPerformanceCounterStorageKHR = 5;
pub const VK_PERFORMANCE_COUNTER_STORAGE_MAX_ENUM_KHR: VkPerformanceCounterStorageKHR = 2147483647;

pub type VkPerformanceCounterDescriptionFlagsKHR = VkFlags;
pub type VkPerformanceCounterDescriptionFlagBitsKHR = u32;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = 1;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = 2;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = 1;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = 2;
pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_FLAG_BITS_MAX_ENUM_KHR: VkPerformanceCounterDescriptionFlagBitsKHR = 2147483647;

pub type VkAcquireProfilingLockFlagsKHR = VkFlags;
pub type VkAcquireProfilingLockFlagBitsKHR = u32;
pub const VK_ACQUIRE_PROFILING_LOCK_FLAG_BITS_MAX_ENUM_KHR: VkAcquireProfilingLockFlagBitsKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub performanceCounterQueryPools: VkBool32,
	pub performanceCounterMultipleQueryPools: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub allowCommandBufferQueryCopies: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceCounterKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub unit: VkPerformanceCounterUnitKHR,
	pub scope: VkPerformanceCounterScopeKHR,
	pub storage: VkPerformanceCounterStorageKHR,
	pub uuid: [u8; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceCounterDescriptionKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub flags: VkPerformanceCounterDescriptionFlagsKHR,
	pub name: [i8; 256],
	pub category: [i8; 256],
	pub description: [i8; 256],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub queueFamilyIndex: u32,
	pub counterIndexCount: u32,
	pub pCounterIndices: *const u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkPerformanceCounterResultKHR {
	pub int32: i32,
	pub int64: i64,
	pub uint32: u32,
	pub uint64: u64,
	pub float32: f32,
	pub float64: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAcquireProfilingLockInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkAcquireProfilingLockFlagsKHR,
	pub timeout: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceQuerySubmitInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub counterPassIndex: u32,
}

pub type PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice,
	queueFamilyIndex: u32,
	pCounterCount: *mut u32,
	pCounters: *mut VkPerformanceCounterKHR,
	pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
) -> VkResult;

pub type PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR, pNumPasses: *mut u32);

pub type PFN_vkAcquireProfilingLockKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkAcquireProfilingLockInfoKHR) -> VkResult;

pub type PFN_vkReleaseProfilingLockKHR = unsafe extern "C" fn(device: VkDevice);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR(
		physicalDevice: VkPhysicalDevice,
		queueFamilyIndex: u32,
		pCounterCount: *mut u32,
		pCounters: *mut VkPerformanceCounterKHR,
		pCounterDescriptions: *mut VkPerformanceCounterDescriptionKHR,
	) -> VkResult;

	pub fn vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR(physicalDevice: VkPhysicalDevice, pPerformanceQueryCreateInfo: *const VkQueryPoolPerformanceCreateInfoKHR, pNumPasses: *mut u32);

	pub fn vkAcquireProfilingLockKHR(device: VkDevice, pInfo: *const VkAcquireProfilingLockInfoKHR) -> VkResult;

	pub fn vkReleaseProfilingLockKHR(device: VkDevice);
}

pub use VkPointClippingBehavior as VkPointClippingBehaviorKHR;
pub use VkTessellationDomainOrigin as VkTessellationDomainOriginKHR;
pub type VkPhysicalDevicePointClippingPropertiesKHR = VkPhysicalDevicePointClippingProperties;
pub type VkRenderPassInputAttachmentAspectCreateInfoKHR = VkRenderPassInputAttachmentAspectCreateInfo;
pub type VkInputAttachmentAspectReferenceKHR = VkInputAttachmentAspectReference;
pub type VkImageViewUsageCreateInfoKHR = VkImageViewUsageCreateInfo;
pub type VkPipelineTessellationDomainOriginStateCreateInfoKHR = VkPipelineTessellationDomainOriginStateCreateInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub surface: VkSurfaceKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceCapabilities2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub surfaceCapabilities: VkSurfaceCapabilitiesKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceFormat2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub surfaceFormat: VkSurfaceFormatKHR,
}

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR) -> VkResult;

pub type PFN_vkGetPhysicalDeviceSurfaceFormats2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR, pSurfaceFormatCount: *mut u32, pSurfaceFormats: *mut VkSurfaceFormat2KHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceSurfaceCapabilities2KHR(
		physicalDevice: VkPhysicalDevice,
		pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
		pSurfaceCapabilities: *mut VkSurfaceCapabilities2KHR,
	) -> VkResult;

	pub fn vkGetPhysicalDeviceSurfaceFormats2KHR(
		physicalDevice: VkPhysicalDevice,
		pSurfaceInfo: *const VkPhysicalDeviceSurfaceInfo2KHR,
		pSurfaceFormatCount: *mut u32,
		pSurfaceFormats: *mut VkSurfaceFormat2KHR,
	) -> VkResult;
}

pub type VkPhysicalDeviceVariablePointerFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;
pub type VkPhysicalDeviceVariablePointersFeaturesKHR = VkPhysicalDeviceVariablePointersFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayProperties2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub displayProperties: VkDisplayPropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPlaneProperties2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub displayPlaneProperties: VkDisplayPlanePropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayModeProperties2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub displayModeProperties: VkDisplayModePropertiesKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPlaneInfo2KHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub mode: VkDisplayModeKHR,
	pub planeIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPlaneCapabilities2KHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub capabilities: VkDisplayPlaneCapabilitiesKHR,
}

pub type PFN_vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayProperties2KHR) -> VkResult;

pub type PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlaneProperties2KHR) -> VkResult;

pub type PFN_vkGetDisplayModeProperties2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModeProperties2KHR) -> VkResult;

pub type PFN_vkGetDisplayPlaneCapabilities2KHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR, pCapabilities: *mut VkDisplayPlaneCapabilities2KHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceDisplayProperties2KHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayProperties2KHR) -> VkResult;

	pub fn vkGetPhysicalDeviceDisplayPlaneProperties2KHR(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkDisplayPlaneProperties2KHR) -> VkResult;

	pub fn vkGetDisplayModeProperties2KHR(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR, pPropertyCount: *mut u32, pProperties: *mut VkDisplayModeProperties2KHR) -> VkResult;

	pub fn vkGetDisplayPlaneCapabilities2KHR(physicalDevice: VkPhysicalDevice, pDisplayPlaneInfo: *const VkDisplayPlaneInfo2KHR, pCapabilities: *mut VkDisplayPlaneCapabilities2KHR) -> VkResult;
}

pub type VkMemoryDedicatedRequirementsKHR = VkMemoryDedicatedRequirements;
pub type VkMemoryDedicatedAllocateInfoKHR = VkMemoryDedicatedAllocateInfo;
pub type VkBufferMemoryRequirementsInfo2KHR = VkBufferMemoryRequirementsInfo2;
pub type VkImageMemoryRequirementsInfo2KHR = VkImageMemoryRequirementsInfo2;
pub type VkImageSparseMemoryRequirementsInfo2KHR = VkImageSparseMemoryRequirementsInfo2;
pub type VkMemoryRequirements2KHR = VkMemoryRequirements2;
pub type VkSparseImageMemoryRequirements2KHR = VkSparseImageMemoryRequirements2;

pub type PFN_vkGetImageMemoryRequirements2KHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetBufferMemoryRequirements2KHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetImageSparseMemoryRequirements2KHR = unsafe extern "C" fn(
	device: VkDevice,
	pInfo: *const VkImageSparseMemoryRequirementsInfo2,
	pSparseMemoryRequirementCount: *mut u32,
	pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetImageMemoryRequirements2KHR(device: VkDevice, pInfo: *const VkImageMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetBufferMemoryRequirements2KHR(device: VkDevice, pInfo: *const VkBufferMemoryRequirementsInfo2, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetImageSparseMemoryRequirements2KHR(
		device: VkDevice,
		pInfo: *const VkImageSparseMemoryRequirementsInfo2,
		pSparseMemoryRequirementCount: *mut u32,
		pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
	);
}

pub type VkImageFormatListCreateInfoKHR = VkImageFormatListCreateInfo;
pub type VkSamplerYcbcrConversionKHR = VkSamplerYcbcrConversion;

pub use VkChromaLocation as VkChromaLocationKHR;
pub use VkSamplerYcbcrModelConversion as VkSamplerYcbcrModelConversionKHR;
pub use VkSamplerYcbcrRange as VkSamplerYcbcrRangeKHR;

pub type VkSamplerYcbcrConversionCreateInfoKHR = VkSamplerYcbcrConversionCreateInfo;
pub type VkSamplerYcbcrConversionInfoKHR = VkSamplerYcbcrConversionInfo;
pub type VkBindImagePlaneMemoryInfoKHR = VkBindImagePlaneMemoryInfo;
pub type VkImagePlaneMemoryRequirementsInfoKHR = VkImagePlaneMemoryRequirementsInfo;
pub type VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR = VkPhysicalDeviceSamplerYcbcrConversionFeatures;
pub type VkSamplerYcbcrConversionImageFormatPropertiesKHR = VkSamplerYcbcrConversionImageFormatProperties;

pub type PFN_vkCreateSamplerYcbcrConversionKHR = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
	pAllocator: *const VkAllocationCallbacks,
	pYcbcrConversion: *mut VkSamplerYcbcrConversion,
) -> VkResult;

pub type PFN_vkDestroySamplerYcbcrConversionKHR = unsafe extern "C" fn(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateSamplerYcbcrConversionKHR(
		device: VkDevice,
		pCreateInfo: *const VkSamplerYcbcrConversionCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pYcbcrConversion: *mut VkSamplerYcbcrConversion,
	) -> VkResult;

	pub fn vkDestroySamplerYcbcrConversionKHR(device: VkDevice, ycbcrConversion: VkSamplerYcbcrConversion, pAllocator: *const VkAllocationCallbacks);
}

pub type VkBindBufferMemoryInfoKHR = VkBindBufferMemoryInfo;
pub type VkBindImageMemoryInfoKHR = VkBindImageMemoryInfo;

pub type PFN_vkBindBufferMemory2KHR = unsafe extern "C" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;

pub type PFN_vkBindImageMemory2KHR = unsafe extern "C" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkBindBufferMemory2KHR(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindBufferMemoryInfo) -> VkResult;

	pub fn vkBindImageMemory2KHR(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindImageMemoryInfo) -> VkResult;
}

pub type VkPhysicalDeviceMaintenance3PropertiesKHR = VkPhysicalDeviceMaintenance3Properties;
pub type VkDescriptorSetLayoutSupportKHR = VkDescriptorSetLayoutSupport;

pub type PFN_vkGetDescriptorSetLayoutSupportKHR = unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetDescriptorSetLayoutSupportKHR(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pSupport: *mut VkDescriptorSetLayoutSupport);
}

pub type PFN_vkCmdDrawIndirectCountKHR =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

pub type PFN_vkCmdDrawIndexedIndirectCountKHR =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdDrawIndirectCountKHR(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

	pub fn vkCmdDrawIndexedIndirectCountKHR(
		commandBuffer: VkCommandBuffer,
		buffer: VkBuffer,
		offset: VkDeviceSize,
		countBuffer: VkBuffer,
		countBufferOffset: VkDeviceSize,
		maxDrawCount: u32,
		stride: u32,
	);
}

pub type VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR = VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures;
pub type VkPhysicalDevice8BitStorageFeaturesKHR = VkPhysicalDevice8BitStorageFeatures;
pub type VkPhysicalDeviceShaderAtomicInt64FeaturesKHR = VkPhysicalDeviceShaderAtomicInt64Features;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderSubgroupClock: VkBool32,
	pub shaderDeviceClock: VkBool32,
}

pub type VkQueueGlobalPriorityKHR = u32;
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR: VkQueueGlobalPriorityKHR = 128;
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR: VkQueueGlobalPriorityKHR = 256;
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR: VkQueueGlobalPriorityKHR = 512;
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR: VkQueueGlobalPriorityKHR = 1024;
pub const VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT: VkQueueGlobalPriorityKHR = 128;
pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT: VkQueueGlobalPriorityKHR = 256;
pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT: VkQueueGlobalPriorityKHR = 512;
pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT: VkQueueGlobalPriorityKHR = 1024;
pub const VK_QUEUE_GLOBAL_PRIORITY_MAX_ENUM_KHR: VkQueueGlobalPriorityKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub globalPriority: VkQueueGlobalPriorityKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub globalPriorityQuery: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub priorityCount: u32,
	pub priorities: [VkQueueGlobalPriorityKHR; 16],
}

pub use VkDriverId as VkDriverIdKHR;

pub type VkConformanceVersionKHR = VkConformanceVersion;
pub type VkPhysicalDeviceDriverPropertiesKHR = VkPhysicalDeviceDriverProperties;

pub use VkShaderFloatControlsIndependence as VkShaderFloatControlsIndependenceKHR;

pub type VkPhysicalDeviceFloatControlsPropertiesKHR = VkPhysicalDeviceFloatControlsProperties;

pub use VkResolveModeFlagBits as VkResolveModeFlagBitsKHR;

pub type VkResolveModeFlagsKHR = VkResolveModeFlags;
pub type VkSubpassDescriptionDepthStencilResolveKHR = VkSubpassDescriptionDepthStencilResolve;
pub type VkPhysicalDeviceDepthStencilResolvePropertiesKHR = VkPhysicalDeviceDepthStencilResolveProperties;

pub use VkSemaphoreType as VkSemaphoreTypeKHR;
pub use VkSemaphoreWaitFlagBits as VkSemaphoreWaitFlagBitsKHR;

pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags;
pub type VkPhysicalDeviceTimelineSemaphoreFeaturesKHR = VkPhysicalDeviceTimelineSemaphoreFeatures;
pub type VkPhysicalDeviceTimelineSemaphorePropertiesKHR = VkPhysicalDeviceTimelineSemaphoreProperties;
pub type VkSemaphoreTypeCreateInfoKHR = VkSemaphoreTypeCreateInfo;
pub type VkTimelineSemaphoreSubmitInfoKHR = VkTimelineSemaphoreSubmitInfo;
pub type VkSemaphoreWaitInfoKHR = VkSemaphoreWaitInfo;
pub type VkSemaphoreSignalInfoKHR = VkSemaphoreSignalInfo;

pub type PFN_vkGetSemaphoreCounterValueKHR = unsafe extern "C" fn(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;

pub type PFN_vkWaitSemaphoresKHR = unsafe extern "C" fn(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;

pub type PFN_vkSignalSemaphoreKHR = unsafe extern "C" fn(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetSemaphoreCounterValueKHR(device: VkDevice, semaphore: VkSemaphore, pValue: *mut u64) -> VkResult;

	pub fn vkWaitSemaphoresKHR(device: VkDevice, pWaitInfo: *const VkSemaphoreWaitInfo, timeout: u64) -> VkResult;

	pub fn vkSignalSemaphoreKHR(device: VkDevice, pSignalInfo: *const VkSemaphoreSignalInfo) -> VkResult;
}

pub type VkPhysicalDeviceVulkanMemoryModelFeaturesKHR = VkPhysicalDeviceVulkanMemoryModelFeatures;
pub type VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR = VkPhysicalDeviceShaderTerminateInvocationFeatures;

pub type VkFragmentShadingRateCombinerOpKHR = u32;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: VkFragmentShadingRateCombinerOpKHR = 0;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: VkFragmentShadingRateCombinerOpKHR = 1;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: VkFragmentShadingRateCombinerOpKHR = 2;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: VkFragmentShadingRateCombinerOpKHR = 3;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: VkFragmentShadingRateCombinerOpKHR = 4;
pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_ENUM_KHR: VkFragmentShadingRateCombinerOpKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pFragmentShadingRateAttachment: *const VkAttachmentReference2,
	pub shadingRateAttachmentTexelSize: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub fragmentSize: VkExtent2D,
	pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pipelineFragmentShadingRate: VkBool32,
	pub primitiveFragmentShadingRate: VkBool32,
	pub attachmentFragmentShadingRate: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minFragmentShadingRateAttachmentTexelSize: VkExtent2D,
	pub maxFragmentShadingRateAttachmentTexelSize: VkExtent2D,
	pub maxFragmentShadingRateAttachmentTexelSizeAspectRatio: u32,
	pub primitiveFragmentShadingRateWithMultipleViewports: VkBool32,
	pub layeredShadingRateAttachments: VkBool32,
	pub fragmentShadingRateNonTrivialCombinerOps: VkBool32,
	pub maxFragmentSize: VkExtent2D,
	pub maxFragmentSizeAspectRatio: u32,
	pub maxFragmentShadingRateCoverageSamples: u32,
	pub maxFragmentShadingRateRasterizationSamples: VkSampleCountFlagBits,
	pub fragmentShadingRateWithShaderDepthStencilWrites: VkBool32,
	pub fragmentShadingRateWithSampleMask: VkBool32,
	pub fragmentShadingRateWithShaderSampleMask: VkBool32,
	pub fragmentShadingRateWithConservativeRasterization: VkBool32,
	pub fragmentShadingRateWithFragmentShaderInterlock: VkBool32,
	pub fragmentShadingRateWithCustomSampleLocations: VkBool32,
	pub fragmentShadingRateStrictMultiplyCombiner: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub sampleCounts: VkSampleCountFlags,
	pub fragmentSize: VkExtent2D,
}

pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pFragmentShadingRateCount: *mut u32, pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR) -> VkResult;

pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pFragmentSize: *const VkExtent2D, combinerOps: *const VkFragmentShadingRateCombinerOpKHR);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceFragmentShadingRatesKHR(
		physicalDevice: VkPhysicalDevice,
		pFragmentShadingRateCount: *mut u32,
		pFragmentShadingRates: *mut VkPhysicalDeviceFragmentShadingRateKHR,
	) -> VkResult;

	pub fn vkCmdSetFragmentShadingRateKHR(commandBuffer: VkCommandBuffer, pFragmentSize: *const VkExtent2D, combinerOps: *const VkFragmentShadingRateCombinerOpKHR);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub supportsProtected: VkBool32,
}

pub type VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures;
pub type VkAttachmentReferenceStencilLayoutKHR = VkAttachmentReferenceStencilLayout;
pub type VkAttachmentDescriptionStencilLayoutKHR = VkAttachmentDescriptionStencilLayout;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub presentWait: VkBool32,
}

pub type PFN_vkWaitForPresentKHR = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, presentId: u64, timeout: u64) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkWaitForPresentKHR(device: VkDevice, swapchain: VkSwapchainKHR, presentId: u64, timeout: u64) -> VkResult;
}

pub type VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR = VkPhysicalDeviceUniformBufferStandardLayoutFeatures;
pub type VkPhysicalDeviceBufferDeviceAddressFeaturesKHR = VkPhysicalDeviceBufferDeviceAddressFeatures;
pub type VkBufferDeviceAddressInfoKHR = VkBufferDeviceAddressInfo;
pub type VkBufferOpaqueCaptureAddressCreateInfoKHR = VkBufferOpaqueCaptureAddressCreateInfo;
pub type VkMemoryOpaqueCaptureAddressAllocateInfoKHR = VkMemoryOpaqueCaptureAddressAllocateInfo;
pub type VkDeviceMemoryOpaqueCaptureAddressInfoKHR = VkDeviceMemoryOpaqueCaptureAddressInfo;

pub type PFN_vkGetBufferDeviceAddressKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

pub type PFN_vkGetBufferOpaqueCaptureAddressKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;

pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetBufferDeviceAddressKHR(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

	pub fn vkGetBufferOpaqueCaptureAddressKHR(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> u64;

	pub fn vkGetDeviceMemoryOpaqueCaptureAddressKHR(device: VkDevice, pInfo: *const VkDeviceMemoryOpaqueCaptureAddressInfo) -> u64;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeferredOperationKHR_T {
	_unused: [u8; 0],
}

pub type VkDeferredOperationKHR = *mut VkDeferredOperationKHR_T;

pub type PFN_vkCreateDeferredOperationKHR = unsafe extern "C" fn(device: VkDevice, pAllocator: *const VkAllocationCallbacks, pDeferredOperation: *mut VkDeferredOperationKHR) -> VkResult;

pub type PFN_vkDestroyDeferredOperationKHR = unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetDeferredOperationMaxConcurrencyKHR = unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> u32;

pub type PFN_vkGetDeferredOperationResultKHR = unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;

pub type PFN_vkDeferredOperationJoinKHR = unsafe extern "C" fn(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateDeferredOperationKHR(device: VkDevice, pAllocator: *const VkAllocationCallbacks, pDeferredOperation: *mut VkDeferredOperationKHR) -> VkResult;

	pub fn vkDestroyDeferredOperationKHR(device: VkDevice, operation: VkDeferredOperationKHR, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetDeferredOperationMaxConcurrencyKHR(device: VkDevice, operation: VkDeferredOperationKHR) -> u32;

	pub fn vkGetDeferredOperationResultKHR(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;

	pub fn vkDeferredOperationJoinKHR(device: VkDevice, operation: VkDeferredOperationKHR) -> VkResult;
}

pub type VkPipelineExecutableStatisticFormatKHR = u32;
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: VkPipelineExecutableStatisticFormatKHR = 0;
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: VkPipelineExecutableStatisticFormatKHR = 1;
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: VkPipelineExecutableStatisticFormatKHR = 2;
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: VkPipelineExecutableStatisticFormatKHR = 3;
pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_MAX_ENUM_KHR: VkPipelineExecutableStatisticFormatKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pipelineExecutableInfo: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pipeline: VkPipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineExecutablePropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub stages: VkShaderStageFlags,
	pub name: [i8; 256],
	pub description: [i8; 256],
	pub subgroupSize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineExecutableInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pipeline: VkPipeline,
	pub executableIndex: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkPipelineExecutableStatisticValueKHR {
	pub b32: VkBool32,
	pub i64_: i64,
	pub u64_: u64,
	pub f64_: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPipelineExecutableStatisticKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub name: [i8; 256],
	pub description: [i8; 256],
	pub format: VkPipelineExecutableStatisticFormatKHR,
	pub value: VkPipelineExecutableStatisticValueKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub name: [i8; 256],
	pub description: [i8; 256],
	pub isText: VkBool32,
	pub dataSize: usize,
	pub pData: *mut (),
}

pub type PFN_vkGetPipelineExecutablePropertiesKHR =
	unsafe extern "C" fn(device: VkDevice, pPipelineInfo: *const VkPipelineInfoKHR, pExecutableCount: *mut u32, pProperties: *mut VkPipelineExecutablePropertiesKHR) -> VkResult;

pub type PFN_vkGetPipelineExecutableStatisticsKHR =
	unsafe extern "C" fn(device: VkDevice, pExecutableInfo: *const VkPipelineExecutableInfoKHR, pStatisticCount: *mut u32, pStatistics: *mut VkPipelineExecutableStatisticKHR) -> VkResult;

pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "C" fn(
	device: VkDevice,
	pExecutableInfo: *const VkPipelineExecutableInfoKHR,
	pInternalRepresentationCount: *mut u32,
	pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPipelineExecutablePropertiesKHR(device: VkDevice, pPipelineInfo: *const VkPipelineInfoKHR, pExecutableCount: *mut u32, pProperties: *mut VkPipelineExecutablePropertiesKHR)
	-> VkResult;

	pub fn vkGetPipelineExecutableStatisticsKHR(
		device: VkDevice,
		pExecutableInfo: *const VkPipelineExecutableInfoKHR,
		pStatisticCount: *mut u32,
		pStatistics: *mut VkPipelineExecutableStatisticKHR,
	) -> VkResult;

	pub fn vkGetPipelineExecutableInternalRepresentationsKHR(
		device: VkDevice,
		pExecutableInfo: *const VkPipelineExecutableInfoKHR,
		pInternalRepresentationCount: *mut u32,
		pInternalRepresentations: *mut VkPipelineExecutableInternalRepresentationKHR,
	) -> VkResult;
}

pub type VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR = VkPhysicalDeviceShaderIntegerDotProductFeatures;
pub type VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR = VkPhysicalDeviceShaderIntegerDotProductProperties;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineLibraryCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub libraryCount: u32,
	pub pLibraries: *const VkPipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentIdKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchainCount: u32,
	pub pPresentIds: *const u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub presentId: VkBool32,
}

pub type VkPipelineStageFlags2KHR = VkPipelineStageFlags2;
pub type VkPipelineStageFlagBits2KHR = VkPipelineStageFlagBits2;
pub type VkAccessFlags2KHR = VkAccessFlags2;
pub type VkAccessFlagBits2KHR = VkAccessFlagBits2;

pub use VkSubmitFlagBits as VkSubmitFlagBitsKHR;

pub type VkSubmitFlagsKHR = VkSubmitFlags;
pub type VkMemoryBarrier2KHR = VkMemoryBarrier2;
pub type VkBufferMemoryBarrier2KHR = VkBufferMemoryBarrier2;
pub type VkImageMemoryBarrier2KHR = VkImageMemoryBarrier2;
pub type VkDependencyInfoKHR = VkDependencyInfo;
pub type VkSubmitInfo2KHR = VkSubmitInfo2;
pub type VkSemaphoreSubmitInfoKHR = VkSemaphoreSubmitInfo;
pub type VkCommandBufferSubmitInfoKHR = VkCommandBufferSubmitInfo;
pub type VkPhysicalDeviceSynchronization2FeaturesKHR = VkPhysicalDeviceSynchronization2Features;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueueFamilyCheckpointProperties2NV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub checkpointExecutionStageMask: VkPipelineStageFlags2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCheckpointData2NV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub stage: VkPipelineStageFlags2,
	pub pCheckpointMarker: *mut (),
}

pub type PFN_vkCmdSetEvent2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);

pub type PFN_vkCmdResetEvent2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);

pub type PFN_vkCmdWaitEvents2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);

pub type PFN_vkCmdPipelineBarrier2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);

pub type PFN_vkCmdWriteTimestamp2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);

pub type PFN_vkQueueSubmit2KHR = unsafe extern "C" fn(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;

pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32);

pub type PFN_vkGetQueueCheckpointData2NV = unsafe extern "C" fn(queue: VkQueue, pCheckpointDataCount: *mut u32, pCheckpointData: *mut VkCheckpointData2NV);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetEvent2KHR(commandBuffer: VkCommandBuffer, event: VkEvent, pDependencyInfo: *const VkDependencyInfo);

	pub fn vkCmdResetEvent2KHR(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags2);

	pub fn vkCmdWaitEvents2KHR(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, pDependencyInfos: *const VkDependencyInfo);

	pub fn vkCmdPipelineBarrier2KHR(commandBuffer: VkCommandBuffer, pDependencyInfo: *const VkDependencyInfo);

	pub fn vkCmdWriteTimestamp2KHR(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, queryPool: VkQueryPool, query: u32);

	pub fn vkQueueSubmit2KHR(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo2, fence: VkFence) -> VkResult;

	pub fn vkCmdWriteBufferMarker2AMD(commandBuffer: VkCommandBuffer, stage: VkPipelineStageFlags2, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32);

	pub fn vkGetQueueCheckpointData2NV(queue: VkQueue, pCheckpointDataCount: *mut u32, pCheckpointData: *mut VkCheckpointData2NV);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderSubgroupUniformControlFlow: VkBool32,
}

pub type VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR = VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub workgroupMemoryExplicitLayout: VkBool32,
	pub workgroupMemoryExplicitLayoutScalarBlockLayout: VkBool32,
	pub workgroupMemoryExplicitLayout8BitAccess: VkBool32,
	pub workgroupMemoryExplicitLayout16BitAccess: VkBool32,
}

pub type VkCopyBufferInfo2KHR = VkCopyBufferInfo2;
pub type VkCopyImageInfo2KHR = VkCopyImageInfo2;
pub type VkCopyBufferToImageInfo2KHR = VkCopyBufferToImageInfo2;
pub type VkCopyImageToBufferInfo2KHR = VkCopyImageToBufferInfo2;
pub type VkBlitImageInfo2KHR = VkBlitImageInfo2;
pub type VkResolveImageInfo2KHR = VkResolveImageInfo2;
pub type VkBufferCopy2KHR = VkBufferCopy2;
pub type VkImageCopy2KHR = VkImageCopy2;
pub type VkImageBlit2KHR = VkImageBlit2;
pub type VkBufferImageCopy2KHR = VkBufferImageCopy2;
pub type VkImageResolve2KHR = VkImageResolve2;

pub type PFN_vkCmdCopyBuffer2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);

pub type PFN_vkCmdCopyImage2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);

pub type PFN_vkCmdCopyBufferToImage2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);

pub type PFN_vkCmdCopyImageToBuffer2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);

pub type PFN_vkCmdBlitImage2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);

pub type PFN_vkCmdResolveImage2KHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdCopyBuffer2KHR(commandBuffer: VkCommandBuffer, pCopyBufferInfo: *const VkCopyBufferInfo2);

	pub fn vkCmdCopyImage2KHR(commandBuffer: VkCommandBuffer, pCopyImageInfo: *const VkCopyImageInfo2);

	pub fn vkCmdCopyBufferToImage2KHR(commandBuffer: VkCommandBuffer, pCopyBufferToImageInfo: *const VkCopyBufferToImageInfo2);

	pub fn vkCmdCopyImageToBuffer2KHR(commandBuffer: VkCommandBuffer, pCopyImageToBufferInfo: *const VkCopyImageToBufferInfo2);

	pub fn vkCmdBlitImage2KHR(commandBuffer: VkCommandBuffer, pBlitImageInfo: *const VkBlitImageInfo2);

	pub fn vkCmdResolveImage2KHR(commandBuffer: VkCommandBuffer, pResolveImageInfo: *const VkResolveImageInfo2);
}

pub type VkFormatFeatureFlags2KHR = VkFormatFeatureFlags2;
pub type VkFormatFeatureFlagBits2KHR = VkFormatFeatureFlagBits2;
pub type VkFormatProperties3KHR = VkFormatProperties3;
pub type VkPhysicalDeviceMaintenance4FeaturesKHR = VkPhysicalDeviceMaintenance4Features;
pub type VkPhysicalDeviceMaintenance4PropertiesKHR = VkPhysicalDeviceMaintenance4Properties;
pub type VkDeviceBufferMemoryRequirementsKHR = VkDeviceBufferMemoryRequirements;
pub type VkDeviceImageMemoryRequirementsKHR = VkDeviceImageMemoryRequirements;

pub type PFN_vkGetDeviceBufferMemoryRequirementsKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetDeviceImageMemoryRequirementsKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkGetDeviceImageSparseMemoryRequirementsKHR =
	unsafe extern "C" fn(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetDeviceBufferMemoryRequirementsKHR(device: VkDevice, pInfo: *const VkDeviceBufferMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetDeviceImageMemoryRequirementsKHR(device: VkDevice, pInfo: *const VkDeviceImageMemoryRequirements, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkGetDeviceImageSparseMemoryRequirementsKHR(
		device: VkDevice,
		pInfo: *const VkDeviceImageMemoryRequirements,
		pSparseMemoryRequirementCount: *mut u32,
		pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements2,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugReportCallbackEXT_T {
	_unused: [u8; 0],
}

pub type VkDebugReportCallbackEXT = *mut VkDebugReportCallbackEXT_T;

pub type VkDebugReportObjectTypeEXT = u32;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: VkDebugReportObjectTypeEXT = 0;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: VkDebugReportObjectTypeEXT = 1;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: VkDebugReportObjectTypeEXT = 2;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: VkDebugReportObjectTypeEXT = 3;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: VkDebugReportObjectTypeEXT = 4;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: VkDebugReportObjectTypeEXT = 5;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: VkDebugReportObjectTypeEXT = 6;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: VkDebugReportObjectTypeEXT = 7;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: VkDebugReportObjectTypeEXT = 8;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: VkDebugReportObjectTypeEXT = 9;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: VkDebugReportObjectTypeEXT = 10;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: VkDebugReportObjectTypeEXT = 11;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: VkDebugReportObjectTypeEXT = 12;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: VkDebugReportObjectTypeEXT = 13;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: VkDebugReportObjectTypeEXT = 14;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: VkDebugReportObjectTypeEXT = 15;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: VkDebugReportObjectTypeEXT = 16;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 17;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: VkDebugReportObjectTypeEXT = 18;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: VkDebugReportObjectTypeEXT = 19;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 20;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: VkDebugReportObjectTypeEXT = 21;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: VkDebugReportObjectTypeEXT = 22;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: VkDebugReportObjectTypeEXT = 23;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: VkDebugReportObjectTypeEXT = 24;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: VkDebugReportObjectTypeEXT = 25;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: VkDebugReportObjectTypeEXT = 26;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: VkDebugReportObjectTypeEXT = 27;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: VkDebugReportObjectTypeEXT = 28;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: VkDebugReportObjectTypeEXT = 29;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: VkDebugReportObjectTypeEXT = 30;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: VkDebugReportObjectTypeEXT = 33;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT: VkDebugReportObjectTypeEXT = 1000156000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT: VkDebugReportObjectTypeEXT = 1000085000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT: VkDebugReportObjectTypeEXT = 1000029000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT: VkDebugReportObjectTypeEXT = 1000029001;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT: VkDebugReportObjectTypeEXT = 1000150000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT: VkDebugReportObjectTypeEXT = 1000165000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT: VkDebugReportObjectTypeEXT = 1000366000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT = 28;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT = 33;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT: VkDebugReportObjectTypeEXT = 1000085000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT: VkDebugReportObjectTypeEXT = 1000156000;
pub const VK_DEBUG_REPORT_OBJECT_TYPE_MAX_ENUM_EXT: VkDebugReportObjectTypeEXT = 2147483647;

pub type VkDebugReportFlagsEXT = VkFlags;
pub type VkDebugReportFlagBitsEXT = u32;
pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagBitsEXT = 1;
pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = 2;
pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagBitsEXT = 4;
pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagBitsEXT = 8;
pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagBitsEXT = 16;
pub const VK_DEBUG_REPORT_FLAG_BITS_MAX_ENUM_EXT: VkDebugReportFlagBitsEXT = 2147483647;

pub type PFN_vkDebugReportCallbackEXT = unsafe extern "C" fn(
	flags: VkDebugReportFlagsEXT,
	objectType: VkDebugReportObjectTypeEXT,
	object: u64,
	location: usize,
	messageCode: i32,
	pLayerPrefix: *const i8,
	pMessage: *const i8,
	pUserData: *mut (),
) -> VkBool32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugReportCallbackCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDebugReportFlagsEXT,
	pub pfnCallback: PFN_vkDebugReportCallbackEXT,
	pub pUserData: *mut (),
}

pub type PFN_vkCreateDebugReportCallbackEXT =
	unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pCallback: *mut VkDebugReportCallbackEXT) -> VkResult;

pub type PFN_vkDestroyDebugReportCallbackEXT = unsafe extern "C" fn(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkDebugReportMessageEXT = unsafe extern "C" fn(
	instance: VkInstance,
	flags: VkDebugReportFlagsEXT,
	objectType: VkDebugReportObjectTypeEXT,
	object: u64,
	location: usize,
	messageCode: i32,
	pLayerPrefix: *const i8,
	pMessage: *const i8,
);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateDebugReportCallbackEXT(
		instance: VkInstance,
		pCreateInfo: *const VkDebugReportCallbackCreateInfoEXT,
		pAllocator: *const VkAllocationCallbacks,
		pCallback: *mut VkDebugReportCallbackEXT,
	) -> VkResult;

	pub fn vkDestroyDebugReportCallbackEXT(instance: VkInstance, callback: VkDebugReportCallbackEXT, pAllocator: *const VkAllocationCallbacks);

	pub fn vkDebugReportMessageEXT(
		instance: VkInstance,
		flags: VkDebugReportFlagsEXT,
		objectType: VkDebugReportObjectTypeEXT,
		object: u64,
		location: usize,
		messageCode: i32,
		pLayerPrefix: *const i8,
		pMessage: *const i8,
	);
}

pub type VkRasterizationOrderAMD = u32;
pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD = 0;
pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD = 1;
pub const VK_RASTERIZATION_ORDER_MAX_ENUM_AMD: VkRasterizationOrderAMD = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub rasterizationOrder: VkRasterizationOrderAMD,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugMarkerObjectNameInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub objectType: VkDebugReportObjectTypeEXT,
	pub object: u64,
	pub pObjectName: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugMarkerObjectTagInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub objectType: VkDebugReportObjectTypeEXT,
	pub object: u64,
	pub tagName: u64,
	pub tagSize: usize,
	pub pTag: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugMarkerMarkerInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pMarkerName: *const i8,
	pub color: [f32; 4],
}

pub type PFN_vkDebugMarkerSetObjectTagEXT = unsafe extern "C" fn(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;

pub type PFN_vkDebugMarkerSetObjectNameEXT = unsafe extern "C" fn(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;

pub type PFN_vkCmdDebugMarkerBeginEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

pub type PFN_vkCmdDebugMarkerEndEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

pub type PFN_vkCmdDebugMarkerInsertEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkDebugMarkerSetObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugMarkerObjectTagInfoEXT) -> VkResult;

	pub fn vkDebugMarkerSetObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugMarkerObjectNameInfoEXT) -> VkResult;

	pub fn vkCmdDebugMarkerBeginEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);

	pub fn vkCmdDebugMarkerEndEXT(commandBuffer: VkCommandBuffer);

	pub fn vkCmdDebugMarkerInsertEXT(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkDebugMarkerMarkerInfoEXT);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dedicatedAllocation: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub image: VkImage,
	pub buffer: VkBuffer,
}

pub type VkPipelineRasterizationStateStreamCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub transformFeedback: VkBool32,
	pub geometryStreams: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxTransformFeedbackStreams: u32,
	pub maxTransformFeedbackBuffers: u32,
	pub maxTransformFeedbackBufferSize: VkDeviceSize,
	pub maxTransformFeedbackStreamDataSize: u32,
	pub maxTransformFeedbackBufferDataSize: u32,
	pub maxTransformFeedbackBufferDataStride: u32,
	pub transformFeedbackQueries: VkBool32,
	pub transformFeedbackStreamsLinesTriangles: VkBool32,
	pub transformFeedbackRasterizationStreamSelect: VkBool32,
	pub transformFeedbackDraw: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineRasterizationStateStreamCreateFlagsEXT,
	pub rasterizationStream: u32,
}

pub type PFN_vkCmdBindTransformFeedbackBuffersEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize, pSizes: *const VkDeviceSize);

pub type PFN_vkCmdBeginTransformFeedbackEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstCounterBuffer: u32, counterBufferCount: u32, pCounterBuffers: *const VkBuffer, pCounterBufferOffsets: *const VkDeviceSize);

pub type PFN_vkCmdEndTransformFeedbackEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstCounterBuffer: u32, counterBufferCount: u32, pCounterBuffers: *const VkBuffer, pCounterBufferOffsets: *const VkDeviceSize);

pub type PFN_vkCmdBeginQueryIndexedEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags, index: u32);

pub type PFN_vkCmdEndQueryIndexedEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, index: u32);

pub type PFN_vkCmdDrawIndirectByteCountEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, instanceCount: u32, firstInstance: u32, counterBuffer: VkBuffer, counterBufferOffset: VkDeviceSize, counterOffset: u32, vertexStride: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdBindTransformFeedbackBuffersEXT(
		commandBuffer: VkCommandBuffer,
		firstBinding: u32,
		bindingCount: u32,
		pBuffers: *const VkBuffer,
		pOffsets: *const VkDeviceSize,
		pSizes: *const VkDeviceSize,
	);

	pub fn vkCmdBeginTransformFeedbackEXT(
		commandBuffer: VkCommandBuffer,
		firstCounterBuffer: u32,
		counterBufferCount: u32,
		pCounterBuffers: *const VkBuffer,
		pCounterBufferOffsets: *const VkDeviceSize,
	);

	pub fn vkCmdEndTransformFeedbackEXT(commandBuffer: VkCommandBuffer, firstCounterBuffer: u32, counterBufferCount: u32, pCounterBuffers: *const VkBuffer, pCounterBufferOffsets: *const VkDeviceSize);

	pub fn vkCmdBeginQueryIndexedEXT(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags, index: u32);

	pub fn vkCmdEndQueryIndexedEXT(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, index: u32);

	pub fn vkCmdDrawIndirectByteCountEXT(
		commandBuffer: VkCommandBuffer,
		instanceCount: u32,
		firstInstance: u32,
		counterBuffer: VkBuffer,
		counterBufferOffset: VkDeviceSize,
		counterOffset: u32,
		vertexStride: u32,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCuModuleNVX_T {
	_unused: [u8; 0],
}

pub type VkCuModuleNVX = *mut VkCuModuleNVX_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCuFunctionNVX_T {
	_unused: [u8; 0],
}

pub type VkCuFunctionNVX = *mut VkCuFunctionNVX_T;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCuModuleCreateInfoNVX {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub dataSize: usize,
	pub pData: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCuFunctionCreateInfoNVX {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub module: VkCuModuleNVX,
	pub pName: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCuLaunchInfoNVX {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub function: VkCuFunctionNVX,
	pub gridDimX: u32,
	pub gridDimY: u32,
	pub gridDimZ: u32,
	pub blockDimX: u32,
	pub blockDimY: u32,
	pub blockDimZ: u32,
	pub sharedMemBytes: u32,
	pub paramCount: usize,
	pub pParams: *const *const (),
	pub extraCount: usize,
	pub pExtras: *const *const (),
}

pub type PFN_vkCreateCuModuleNVX =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkCuModuleCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pModule: *mut VkCuModuleNVX) -> VkResult;

pub type PFN_vkCreateCuFunctionNVX =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkCuFunctionCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pFunction: *mut VkCuFunctionNVX) -> VkResult;

pub type PFN_vkDestroyCuModuleNVX = unsafe extern "C" fn(device: VkDevice, module: VkCuModuleNVX, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkDestroyCuFunctionNVX = unsafe extern "C" fn(device: VkDevice, function: VkCuFunctionNVX, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCmdCuLaunchKernelNVX = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLaunchInfo: *const VkCuLaunchInfoNVX);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateCuModuleNVX(device: VkDevice, pCreateInfo: *const VkCuModuleCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pModule: *mut VkCuModuleNVX) -> VkResult;

	pub fn vkCreateCuFunctionNVX(device: VkDevice, pCreateInfo: *const VkCuFunctionCreateInfoNVX, pAllocator: *const VkAllocationCallbacks, pFunction: *mut VkCuFunctionNVX) -> VkResult;

	pub fn vkDestroyCuModuleNVX(device: VkDevice, module: VkCuModuleNVX, pAllocator: *const VkAllocationCallbacks);

	pub fn vkDestroyCuFunctionNVX(device: VkDevice, function: VkCuFunctionNVX, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCmdCuLaunchKernelNVX(commandBuffer: VkCommandBuffer, pLaunchInfo: *const VkCuLaunchInfoNVX);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewHandleInfoNVX {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub imageView: VkImageView,
	pub descriptorType: VkDescriptorType,
	pub sampler: VkSampler,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewAddressPropertiesNVX {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceAddress: VkDeviceAddress,
	pub size: VkDeviceSize,
}

pub type PFN_vkGetImageViewHandleNVX = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> u32;

pub type PFN_vkGetImageViewAddressNVX = unsafe extern "C" fn(device: VkDevice, imageView: VkImageView, pProperties: *mut VkImageViewAddressPropertiesNVX) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetImageViewHandleNVX(device: VkDevice, pInfo: *const VkImageViewHandleInfoNVX) -> u32;

	pub fn vkGetImageViewAddressNVX(device: VkDevice, imageView: VkImageView, pProperties: *mut VkImageViewAddressPropertiesNVX) -> VkResult;
}

pub type PFN_vkCmdDrawIndirectCountAMD =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

pub type PFN_vkCmdDrawIndexedIndirectCountAMD =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdDrawIndirectCountAMD(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

	pub fn vkCmdDrawIndexedIndirectCountAMD(
		commandBuffer: VkCommandBuffer,
		buffer: VkBuffer,
		offset: VkDeviceSize,
		countBuffer: VkBuffer,
		countBufferOffset: VkDeviceSize,
		maxDrawCount: u32,
		stride: u32,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub supportsTextureGatherLODBiasAMD: VkBool32,
}

pub type VkShaderInfoTypeAMD = u32;
pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: VkShaderInfoTypeAMD = 0;
pub const VK_SHADER_INFO_TYPE_BINARY_AMD: VkShaderInfoTypeAMD = 1;
pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: VkShaderInfoTypeAMD = 2;
pub const VK_SHADER_INFO_TYPE_MAX_ENUM_AMD: VkShaderInfoTypeAMD = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShaderResourceUsageAMD {
	pub numUsedVgprs: u32,
	pub numUsedSgprs: u32,
	pub ldsSizePerLocalWorkGroup: u32,
	pub ldsUsageSizeInBytes: usize,
	pub scratchMemUsageInBytes: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShaderStatisticsInfoAMD {
	pub shaderStageMask: VkShaderStageFlags,
	pub resourceUsage: VkShaderResourceUsageAMD,
	pub numPhysicalVgprs: u32,
	pub numPhysicalSgprs: u32,
	pub numAvailableVgprs: u32,
	pub numAvailableSgprs: u32,
	pub computeWorkGroupSize: [u32; 3],
}

pub type PFN_vkGetShaderInfoAMD =
	unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, shaderStage: VkShaderStageFlagBits, infoType: VkShaderInfoTypeAMD, pInfoSize: *mut usize, pInfo: *mut ()) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetShaderInfoAMD(device: VkDevice, pipeline: VkPipeline, shaderStage: VkShaderStageFlagBits, infoType: VkShaderInfoTypeAMD, pInfoSize: *mut usize, pInfo: *mut ()) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub cornerSampledImage: VkBool32,
}

pub type VkExternalMemoryHandleTypeFlagsNV = VkFlags;
pub type VkExternalMemoryHandleTypeFlagBitsNV = u32;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = 1;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = 2;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = 4;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagBitsNV = 8;
pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_FLAG_BITS_MAX_ENUM_NV: VkExternalMemoryHandleTypeFlagBitsNV = 2147483647;

pub type VkExternalMemoryFeatureFlagsNV = VkFlags;
pub type VkExternalMemoryFeatureFlagBitsNV = u32;
pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = 1;
pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = 2;
pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagBitsNV = 4;
pub const VK_EXTERNAL_MEMORY_FEATURE_FLAG_BITS_MAX_ENUM_NV: VkExternalMemoryFeatureFlagBitsNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalImageFormatPropertiesNV {
	pub imageFormatProperties: VkImageFormatProperties,
	pub externalMemoryFeatures: VkExternalMemoryFeatureFlagsNV,
	pub exportFromImportedHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
	pub compatibleHandleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

pub type PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "C" fn(
	physicalDevice: VkPhysicalDevice,
	format: VkFormat,
	type_: VkImageType,
	tiling: VkImageTiling,
	usage: VkImageUsageFlags,
	flags: VkImageCreateFlags,
	externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
	pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceExternalImageFormatPropertiesNV(
		physicalDevice: VkPhysicalDevice,
		format: VkFormat,
		type_: VkImageType,
		tiling: VkImageTiling,
		usage: VkImageUsageFlags,
		flags: VkImageCreateFlags,
		externalHandleType: VkExternalMemoryHandleTypeFlagsNV,
		pExternalImageFormatProperties: *mut VkExternalImageFormatPropertiesNV,
	) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExternalMemoryImageCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkExportMemoryAllocateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleTypes: VkExternalMemoryHandleTypeFlagsNV,
}

pub type VkValidationCheckEXT = u32;
pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = 0;
pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = 1;
pub const VK_VALIDATION_CHECK_MAX_ENUM_EXT: VkValidationCheckEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkValidationFlagsEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub disabledValidationCheckCount: u32,
	pub pDisabledValidationChecks: *const VkValidationCheckEXT,
}

pub type VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT = VkPhysicalDeviceTextureCompressionASTCHDRFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewASTCDecodeModeEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub decodeMode: VkFormat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub decodeModeSharedExponent: VkBool32,
}

pub type VkConditionalRenderingFlagsEXT = VkFlags;
pub type VkConditionalRenderingFlagBitsEXT = u32;
pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT: VkConditionalRenderingFlagBitsEXT = 1;
pub const VK_CONDITIONAL_RENDERING_FLAG_BITS_MAX_ENUM_EXT: VkConditionalRenderingFlagBitsEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkConditionalRenderingBeginInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub flags: VkConditionalRenderingFlagsEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub conditionalRendering: VkBool32,
	pub inheritedConditionalRendering: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub conditionalRenderingEnable: VkBool32,
}

pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT);

pub type PFN_vkCmdEndConditionalRenderingEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdBeginConditionalRenderingEXT(commandBuffer: VkCommandBuffer, pConditionalRenderingBegin: *const VkConditionalRenderingBeginInfoEXT);

	pub fn vkCmdEndConditionalRenderingEXT(commandBuffer: VkCommandBuffer);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkViewportWScalingNV {
	pub xcoeff: f32,
	pub ycoeff: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub viewportWScalingEnable: VkBool32,
	pub viewportCount: u32,
	pub pViewportWScalings: *const VkViewportWScalingNV,
}

pub type PFN_vkCmdSetViewportWScalingNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetViewportWScalingNV(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewportWScalings: *const VkViewportWScalingNV);
}

pub type PFN_vkReleaseDisplayEXT = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkReleaseDisplayEXT(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;
}

pub type VkSurfaceCounterFlagsEXT = VkFlags;
pub type VkSurfaceCounterFlagBitsEXT = u32;
pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagBitsEXT = 1;
pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagBitsEXT = 1;
pub const VK_SURFACE_COUNTER_FLAG_BITS_MAX_ENUM_EXT: VkSurfaceCounterFlagBitsEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSurfaceCapabilities2EXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minImageCount: u32,
	pub maxImageCount: u32,
	pub currentExtent: VkExtent2D,
	pub minImageExtent: VkExtent2D,
	pub maxImageExtent: VkExtent2D,
	pub maxImageArrayLayers: u32,
	pub supportedTransforms: VkSurfaceTransformFlagsKHR,
	pub currentTransform: VkSurfaceTransformFlagBitsKHR,
	pub supportedCompositeAlpha: VkCompositeAlphaFlagsKHR,
	pub supportedUsageFlags: VkImageUsageFlags,
	pub supportedSurfaceCounters: VkSurfaceCounterFlagsEXT,
}

pub type PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceSurfaceCapabilities2EXT(physicalDevice: VkPhysicalDevice, surface: VkSurfaceKHR, pSurfaceCapabilities: *mut VkSurfaceCapabilities2EXT) -> VkResult;
}

pub type VkDisplayPowerStateEXT = u32;
pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT = 0;
pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT = 1;
pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT = 2;
pub const VK_DISPLAY_POWER_STATE_MAX_ENUM_EXT: VkDisplayPowerStateEXT = 2147483647;

pub type VkDeviceEventTypeEXT = u32;
pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT = 0;
pub const VK_DEVICE_EVENT_TYPE_MAX_ENUM_EXT: VkDeviceEventTypeEXT = 2147483647;

pub type VkDisplayEventTypeEXT = u32;
pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT = 0;
pub const VK_DISPLAY_EVENT_TYPE_MAX_ENUM_EXT: VkDisplayEventTypeEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayPowerInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub powerState: VkDisplayPowerStateEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceEventInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceEvent: VkDeviceEventTypeEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayEventInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub displayEvent: VkDisplayEventTypeEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSwapchainCounterCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub surfaceCounters: VkSurfaceCounterFlagsEXT,
}

pub type PFN_vkDisplayPowerControlEXT = unsafe extern "C" fn(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;

pub type PFN_vkRegisterDeviceEventEXT =
	unsafe extern "C" fn(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

pub type PFN_vkRegisterDisplayEventEXT =
	unsafe extern "C" fn(device: VkDevice, display: VkDisplayKHR, pDisplayEventInfo: *const VkDisplayEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkDisplayPowerControlEXT(device: VkDevice, display: VkDisplayKHR, pDisplayPowerInfo: *const VkDisplayPowerInfoEXT) -> VkResult;

	pub fn vkRegisterDeviceEventEXT(device: VkDevice, pDeviceEventInfo: *const VkDeviceEventInfoEXT, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;

	pub fn vkRegisterDisplayEventEXT(
		device: VkDevice,
		display: VkDisplayKHR,
		pDisplayEventInfo: *const VkDisplayEventInfoEXT,
		pAllocator: *const VkAllocationCallbacks,
		pFence: *mut VkFence,
	) -> VkResult;

	pub fn vkGetSwapchainCounterEXT(device: VkDevice, swapchain: VkSwapchainKHR, counter: VkSurfaceCounterFlagBitsEXT, pCounterValue: *mut u64) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRefreshCycleDurationGOOGLE {
	pub refreshDuration: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPastPresentationTimingGOOGLE {
	pub presentID: u32,
	pub desiredPresentTime: u64,
	pub actualPresentTime: u64,
	pub earliestPresentTime: u64,
	pub presentMargin: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentTimeGOOGLE {
	pub presentID: u32,
	pub desiredPresentTime: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPresentTimesInfoGOOGLE {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub swapchainCount: u32,
	pub pTimes: *const VkPresentTimeGOOGLE,
}

pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;

pub type PFN_vkGetPastPresentationTimingGOOGLE =
	unsafe extern "C" fn(device: VkDevice, swapchain: VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetRefreshCycleDurationGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pDisplayTimingProperties: *mut VkRefreshCycleDurationGOOGLE) -> VkResult;

	pub fn vkGetPastPresentationTimingGOOGLE(device: VkDevice, swapchain: VkSwapchainKHR, pPresentationTimingCount: *mut u32, pPresentationTimings: *mut VkPastPresentationTimingGOOGLE) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub perViewPositionAllComponents: VkBool32,
}

pub type VkViewportCoordinateSwizzleNV = u32;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: VkViewportCoordinateSwizzleNV = 0;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: VkViewportCoordinateSwizzleNV = 1;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: VkViewportCoordinateSwizzleNV = 2;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: VkViewportCoordinateSwizzleNV = 3;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: VkViewportCoordinateSwizzleNV = 4;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: VkViewportCoordinateSwizzleNV = 5;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: VkViewportCoordinateSwizzleNV = 6;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: VkViewportCoordinateSwizzleNV = 7;
pub const VK_VIEWPORT_COORDINATE_SWIZZLE_MAX_ENUM_NV: VkViewportCoordinateSwizzleNV = 2147483647;

pub type VkPipelineViewportSwizzleStateCreateFlagsNV = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkViewportSwizzleNV {
	pub x: VkViewportCoordinateSwizzleNV,
	pub y: VkViewportCoordinateSwizzleNV,
	pub z: VkViewportCoordinateSwizzleNV,
	pub w: VkViewportCoordinateSwizzleNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineViewportSwizzleStateCreateFlagsNV,
	pub viewportCount: u32,
	pub pViewportSwizzles: *const VkViewportSwizzleNV,
}

pub type VkDiscardRectangleModeEXT = u32;
pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT = 0;
pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT = 1;
pub const VK_DISCARD_RECTANGLE_MODE_MAX_ENUM_EXT: VkDiscardRectangleModeEXT = 2147483647;

pub type VkPipelineDiscardRectangleStateCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxDiscardRectangles: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineDiscardRectangleStateCreateFlagsEXT,
	pub discardRectangleMode: VkDiscardRectangleModeEXT,
	pub discardRectangleCount: u32,
	pub pDiscardRectangles: *const VkRect2D,
}

pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetDiscardRectangleEXT(commandBuffer: VkCommandBuffer, firstDiscardRectangle: u32, discardRectangleCount: u32, pDiscardRectangles: *const VkRect2D);
}

pub type VkConservativeRasterizationModeEXT = u32;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: VkConservativeRasterizationModeEXT = 0;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 1;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 2;
pub const VK_CONSERVATIVE_RASTERIZATION_MODE_MAX_ENUM_EXT: VkConservativeRasterizationModeEXT = 2147483647;

pub type VkPipelineRasterizationConservativeStateCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub primitiveOverestimationSize: f32,
	pub maxExtraPrimitiveOverestimationSize: f32,
	pub extraPrimitiveOverestimationSizeGranularity: f32,
	pub primitiveUnderestimation: VkBool32,
	pub conservativePointAndLineRasterization: VkBool32,
	pub degenerateTrianglesRasterized: VkBool32,
	pub degenerateLinesRasterized: VkBool32,
	pub fullyCoveredFragmentShaderInputVariable: VkBool32,
	pub conservativeRasterizationPostDepthCoverage: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineRasterizationConservativeStateCreateFlagsEXT,
	pub conservativeRasterizationMode: VkConservativeRasterizationModeEXT,
	pub extraPrimitiveOverestimationSize: f32,
}

pub type VkPipelineRasterizationDepthClipStateCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub depthClipEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
	pub depthClipEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkXYColorEXT {
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkHdrMetadataEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub displayPrimaryRed: VkXYColorEXT,
	pub displayPrimaryGreen: VkXYColorEXT,
	pub displayPrimaryBlue: VkXYColorEXT,
	pub whitePoint: VkXYColorEXT,
	pub maxLuminance: f32,
	pub minLuminance: f32,
	pub maxContentLightLevel: f32,
	pub maxFrameAverageLightLevel: f32,
}

pub type PFN_vkSetHdrMetadataEXT = unsafe extern "C" fn(device: VkDevice, swapchainCount: u32, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkSetHdrMetadataEXT(device: VkDevice, swapchainCount: u32, pSwapchains: *const VkSwapchainKHR, pMetadata: *const VkHdrMetadataEXT);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessengerEXT_T {
	_unused: [u8; 0],
}

pub type VkDebugUtilsMessengerEXT = *mut VkDebugUtilsMessengerEXT_T;

pub type VkDebugUtilsMessengerCallbackDataFlagsEXT = VkFlags;

pub type VkDebugUtilsMessageSeverityFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageSeverityFlagBitsEXT = u32;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 1;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 16;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 256;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 4096;
pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_FLAG_BITS_MAX_ENUM_EXT: VkDebugUtilsMessageSeverityFlagBitsEXT = 2147483647;

pub type VkDebugUtilsMessageTypeFlagsEXT = VkFlags;
pub type VkDebugUtilsMessageTypeFlagBitsEXT = u32;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 1;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 2;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 4;
pub const VK_DEBUG_UTILS_MESSAGE_TYPE_FLAG_BITS_MAX_ENUM_EXT: VkDebugUtilsMessageTypeFlagBitsEXT = 2147483647;

pub type VkDebugUtilsMessengerCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugUtilsLabelEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pLabelName: *const i8,
	pub color: [f32; 4],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugUtilsObjectNameInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub objectType: VkObjectType,
	pub objectHandle: u64,
	pub pObjectName: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDebugUtilsMessengerCallbackDataFlagsEXT,
	pub pMessageIdName: *const i8,
	pub messageIdNumber: i32,
	pub pMessage: *const i8,
	pub queueLabelCount: u32,
	pub pQueueLabels: *const VkDebugUtilsLabelEXT,
	pub cmdBufLabelCount: u32,
	pub pCmdBufLabels: *const VkDebugUtilsLabelEXT,
	pub objectCount: u32,
	pub pObjects: *const VkDebugUtilsObjectNameInfoEXT,
}

pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "C" fn(
	messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
	pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
	pUserData: *mut (),
) -> VkBool32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDebugUtilsMessengerCreateFlagsEXT,
	pub messageSeverity: VkDebugUtilsMessageSeverityFlagsEXT,
	pub messageType: VkDebugUtilsMessageTypeFlagsEXT,
	pub pfnUserCallback: PFN_vkDebugUtilsMessengerCallbackEXT,
	pub pUserData: *mut (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDebugUtilsObjectTagInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub objectType: VkObjectType,
	pub objectHandle: u64,
	pub tagName: u64,
	pub tagSize: usize,
	pub pTag: *const (),
}

pub type PFN_vkSetDebugUtilsObjectNameEXT = unsafe extern "C" fn(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult;

pub type PFN_vkSetDebugUtilsObjectTagEXT = unsafe extern "C" fn(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;

pub type PFN_vkQueueBeginDebugUtilsLabelEXT = unsafe extern "C" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

pub type PFN_vkQueueEndDebugUtilsLabelEXT = unsafe extern "C" fn(queue: VkQueue);

pub type PFN_vkQueueInsertDebugUtilsLabelEXT = unsafe extern "C" fn(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

pub type PFN_vkCmdBeginDebugUtilsLabelEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

pub type PFN_vkCmdEndDebugUtilsLabelEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

pub type PFN_vkCmdInsertDebugUtilsLabelEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

pub type PFN_vkCreateDebugUtilsMessengerEXT =
	unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pMessenger: *mut VkDebugUtilsMessengerEXT) -> VkResult;

pub type PFN_vkDestroyDebugUtilsMessengerEXT = unsafe extern "C" fn(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkSubmitDebugUtilsMessageEXT = unsafe extern "C" fn(
	instance: VkInstance,
	messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
	messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
	pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkSetDebugUtilsObjectNameEXT(device: VkDevice, pNameInfo: *const VkDebugUtilsObjectNameInfoEXT) -> VkResult;

	pub fn vkSetDebugUtilsObjectTagEXT(device: VkDevice, pTagInfo: *const VkDebugUtilsObjectTagInfoEXT) -> VkResult;

	pub fn vkQueueBeginDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

	pub fn vkQueueEndDebugUtilsLabelEXT(queue: VkQueue);

	pub fn vkQueueInsertDebugUtilsLabelEXT(queue: VkQueue, pLabelInfo: *const VkDebugUtilsLabelEXT);

	pub fn vkCmdBeginDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

	pub fn vkCmdEndDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer);

	pub fn vkCmdInsertDebugUtilsLabelEXT(commandBuffer: VkCommandBuffer, pLabelInfo: *const VkDebugUtilsLabelEXT);

	pub fn vkCreateDebugUtilsMessengerEXT(
		instance: VkInstance,
		pCreateInfo: *const VkDebugUtilsMessengerCreateInfoEXT,
		pAllocator: *const VkAllocationCallbacks,
		pMessenger: *mut VkDebugUtilsMessengerEXT,
	) -> VkResult;

	pub fn vkDestroyDebugUtilsMessengerEXT(instance: VkInstance, messenger: VkDebugUtilsMessengerEXT, pAllocator: *const VkAllocationCallbacks);

	pub fn vkSubmitDebugUtilsMessageEXT(
		instance: VkInstance,
		messageSeverity: VkDebugUtilsMessageSeverityFlagBitsEXT,
		messageTypes: VkDebugUtilsMessageTypeFlagsEXT,
		pCallbackData: *const VkDebugUtilsMessengerCallbackDataEXT,
	);
}

pub use VkSamplerReductionMode as VkSamplerReductionModeEXT;

pub type VkSamplerReductionModeCreateInfoEXT = VkSamplerReductionModeCreateInfo;
pub type VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT = VkPhysicalDeviceSamplerFilterMinmaxProperties;
pub type VkPhysicalDeviceInlineUniformBlockFeaturesEXT = VkPhysicalDeviceInlineUniformBlockFeatures;
pub type VkPhysicalDeviceInlineUniformBlockPropertiesEXT = VkPhysicalDeviceInlineUniformBlockProperties;
pub type VkWriteDescriptorSetInlineUniformBlockEXT = VkWriteDescriptorSetInlineUniformBlock;
pub type VkDescriptorPoolInlineUniformBlockCreateInfoEXT = VkDescriptorPoolInlineUniformBlockCreateInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSampleLocationEXT {
	pub x: f32,
	pub y: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSampleLocationsInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub sampleLocationsPerPixel: VkSampleCountFlagBits,
	pub sampleLocationGridSize: VkExtent2D,
	pub sampleLocationsCount: u32,
	pub pSampleLocations: *const VkSampleLocationEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAttachmentSampleLocationsEXT {
	pub attachmentIndex: u32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassSampleLocationsEXT {
	pub subpassIndex: u32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub attachmentInitialSampleLocationsCount: u32,
	pub pAttachmentInitialSampleLocations: *const VkAttachmentSampleLocationsEXT,
	pub postSubpassSampleLocationsCount: u32,
	pub pPostSubpassSampleLocations: *const VkSubpassSampleLocationsEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub sampleLocationsEnable: VkBool32,
	pub sampleLocationsInfo: VkSampleLocationsInfoEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub sampleLocationSampleCounts: VkSampleCountFlags,
	pub maxSampleLocationGridSize: VkExtent2D,
	pub sampleLocationCoordinateRange: [f32; 2],
	pub sampleLocationSubPixelBits: u32,
	pub variableSampleLocations: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMultisamplePropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxSampleLocationGridSize: VkExtent2D,
}

pub type PFN_vkCmdSetSampleLocationsEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT);

pub type PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, samples: VkSampleCountFlagBits, pMultisampleProperties: *mut VkMultisamplePropertiesEXT);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetSampleLocationsEXT(commandBuffer: VkCommandBuffer, pSampleLocationsInfo: *const VkSampleLocationsInfoEXT);

	pub fn vkGetPhysicalDeviceMultisamplePropertiesEXT(physicalDevice: VkPhysicalDevice, samples: VkSampleCountFlagBits, pMultisampleProperties: *mut VkMultisamplePropertiesEXT);
}

pub type VkBlendOverlapEXT = u32;
pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = 0;
pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = 1;
pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = 2;
pub const VK_BLEND_OVERLAP_MAX_ENUM_EXT: VkBlendOverlapEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub advancedBlendCoherentOperations: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub advancedBlendMaxColorAttachments: u32,
	pub advancedBlendIndependentBlend: VkBool32,
	pub advancedBlendNonPremultipliedSrcColor: VkBool32,
	pub advancedBlendNonPremultipliedDstColor: VkBool32,
	pub advancedBlendCorrelatedOverlap: VkBool32,
	pub advancedBlendAllOperations: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub srcPremultiplied: VkBool32,
	pub dstPremultiplied: VkBool32,
	pub blendOverlap: VkBlendOverlapEXT,
}

pub type VkPipelineCoverageToColorStateCreateFlagsNV = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCoverageToColorStateCreateFlagsNV,
	pub coverageToColorEnable: VkBool32,
	pub coverageToColorLocation: u32,
}

pub type VkCoverageModulationModeNV = u32;
pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: VkCoverageModulationModeNV = 0;
pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: VkCoverageModulationModeNV = 1;
pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: VkCoverageModulationModeNV = 2;
pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: VkCoverageModulationModeNV = 3;
pub const VK_COVERAGE_MODULATION_MODE_MAX_ENUM_NV: VkCoverageModulationModeNV = 2147483647;

pub type VkPipelineCoverageModulationStateCreateFlagsNV = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCoverageModulationStateCreateFlagsNV,
	pub coverageModulationMode: VkCoverageModulationModeNV,
	pub coverageModulationTableEnable: VkBool32,
	pub coverageModulationTableCount: u32,
	pub pCoverageModulationTable: *const f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderSMCount: u32,
	pub shaderWarpsPerSM: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderSMBuiltins: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrmFormatModifierPropertiesEXT {
	pub drmFormatModifier: u64,
	pub drmFormatModifierPlaneCount: u32,
	pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrmFormatModifierPropertiesListEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierPropertiesEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub drmFormatModifier: u64,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifiers: *const u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub drmFormatModifier: u64,
	pub drmFormatModifierPlaneCount: u32,
	pub pPlaneLayouts: *const VkSubresourceLayout,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub drmFormatModifier: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrmFormatModifierProperties2EXT {
	pub drmFormatModifier: u64,
	pub drmFormatModifierPlaneCount: u32,
	pub drmFormatModifierTilingFeatures: VkFormatFeatureFlags2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub drmFormatModifierCount: u32,
	pub pDrmFormatModifierProperties: *mut VkDrmFormatModifierProperties2EXT,
}

pub type PFN_vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "C" fn(device: VkDevice, image: VkImage, pProperties: *mut VkImageDrmFormatModifierPropertiesEXT) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetImageDrmFormatModifierPropertiesEXT(device: VkDevice, image: VkImage, pProperties: *mut VkImageDrmFormatModifierPropertiesEXT) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkValidationCacheEXT_T {
	_unused: [u8; 0],
}

pub type VkValidationCacheEXT = *mut VkValidationCacheEXT_T;

pub type VkValidationCacheHeaderVersionEXT = u32;
pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;
pub const VK_VALIDATION_CACHE_HEADER_VERSION_MAX_ENUM_EXT: VkValidationCacheHeaderVersionEXT = 2147483647;

pub type VkValidationCacheCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkValidationCacheCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkValidationCacheCreateFlagsEXT,
	pub initialDataSize: usize,
	pub pInitialData: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub validationCache: VkValidationCacheEXT,
}

pub type PFN_vkCreateValidationCacheEXT =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkValidationCacheCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pValidationCache: *mut VkValidationCacheEXT) -> VkResult;

pub type PFN_vkDestroyValidationCacheEXT = unsafe extern "C" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkMergeValidationCachesEXT = unsafe extern "C" fn(device: VkDevice, dstCache: VkValidationCacheEXT, srcCacheCount: u32, pSrcCaches: *const VkValidationCacheEXT) -> VkResult;

pub type PFN_vkGetValidationCacheDataEXT = unsafe extern "C" fn(device: VkDevice, validationCache: VkValidationCacheEXT, pDataSize: *mut usize, pData: *mut ()) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateValidationCacheEXT(
		device: VkDevice,
		pCreateInfo: *const VkValidationCacheCreateInfoEXT,
		pAllocator: *const VkAllocationCallbacks,
		pValidationCache: *mut VkValidationCacheEXT,
	) -> VkResult;

	pub fn vkDestroyValidationCacheEXT(device: VkDevice, validationCache: VkValidationCacheEXT, pAllocator: *const VkAllocationCallbacks);

	pub fn vkMergeValidationCachesEXT(device: VkDevice, dstCache: VkValidationCacheEXT, srcCacheCount: u32, pSrcCaches: *const VkValidationCacheEXT) -> VkResult;

	pub fn vkGetValidationCacheDataEXT(device: VkDevice, validationCache: VkValidationCacheEXT, pDataSize: *mut usize, pData: *mut ()) -> VkResult;
}

pub use VkDescriptorBindingFlagBits as VkDescriptorBindingFlagBitsEXT;

pub type VkDescriptorBindingFlagsEXT = VkDescriptorBindingFlags;
pub type VkDescriptorSetLayoutBindingFlagsCreateInfoEXT = VkDescriptorSetLayoutBindingFlagsCreateInfo;
pub type VkPhysicalDeviceDescriptorIndexingFeaturesEXT = VkPhysicalDeviceDescriptorIndexingFeatures;
pub type VkPhysicalDeviceDescriptorIndexingPropertiesEXT = VkPhysicalDeviceDescriptorIndexingProperties;
pub type VkDescriptorSetVariableDescriptorCountAllocateInfoEXT = VkDescriptorSetVariableDescriptorCountAllocateInfo;
pub type VkDescriptorSetVariableDescriptorCountLayoutSupportEXT = VkDescriptorSetVariableDescriptorCountLayoutSupport;

pub type VkShadingRatePaletteEntryNV = u32;
pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: VkShadingRatePaletteEntryNV = 0;
pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 1;
pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 2;
pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 3;
pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 4;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 5;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: VkShadingRatePaletteEntryNV = 6;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 7;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 8;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 9;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: VkShadingRatePaletteEntryNV = 10;
pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: VkShadingRatePaletteEntryNV = 11;
pub const VK_SHADING_RATE_PALETTE_ENTRY_MAX_ENUM_NV: VkShadingRatePaletteEntryNV = 2147483647;

pub type VkCoarseSampleOrderTypeNV = u32;
pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: VkCoarseSampleOrderTypeNV = 0;
pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: VkCoarseSampleOrderTypeNV = 1;
pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: VkCoarseSampleOrderTypeNV = 2;
pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: VkCoarseSampleOrderTypeNV = 3;
pub const VK_COARSE_SAMPLE_ORDER_TYPE_MAX_ENUM_NV: VkCoarseSampleOrderTypeNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkShadingRatePaletteNV {
	pub shadingRatePaletteEntryCount: u32,
	pub pShadingRatePaletteEntries: *const VkShadingRatePaletteEntryNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub shadingRateImageEnable: VkBool32,
	pub viewportCount: u32,
	pub pShadingRatePalettes: *const VkShadingRatePaletteNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shadingRateImage: VkBool32,
	pub shadingRateCoarseSampleOrder: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shadingRateTexelSize: VkExtent2D,
	pub shadingRatePaletteSize: u32,
	pub shadingRateMaxCoarseSamples: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCoarseSampleLocationNV {
	pub pixelX: u32,
	pub pixelY: u32,
	pub sample: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCoarseSampleOrderCustomNV {
	pub shadingRate: VkShadingRatePaletteEntryNV,
	pub sampleCount: u32,
	pub sampleLocationCount: u32,
	pub pSampleLocations: *const VkCoarseSampleLocationNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub sampleOrderType: VkCoarseSampleOrderTypeNV,
	pub customSampleOrderCount: u32,
	pub pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
}

pub type PFN_vkCmdBindShadingRateImageNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, imageView: VkImageView, imageLayout: VkImageLayout);

pub type PFN_vkCmdSetViewportShadingRatePaletteNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pShadingRatePalettes: *const VkShadingRatePaletteNV);

pub type PFN_vkCmdSetCoarseSampleOrderNV =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, sampleOrderType: VkCoarseSampleOrderTypeNV, customSampleOrderCount: u32, pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdBindShadingRateImageNV(commandBuffer: VkCommandBuffer, imageView: VkImageView, imageLayout: VkImageLayout);

	pub fn vkCmdSetViewportShadingRatePaletteNV(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pShadingRatePalettes: *const VkShadingRatePaletteNV);

	pub fn vkCmdSetCoarseSampleOrderNV(
		commandBuffer: VkCommandBuffer,
		sampleOrderType: VkCoarseSampleOrderTypeNV,
		customSampleOrderCount: u32,
		pCustomSampleOrders: *const VkCoarseSampleOrderCustomNV,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureNV_T {
	_unused: [u8; 0],
}

pub type VkAccelerationStructureNV = *mut VkAccelerationStructureNV_T;

pub type VkRayTracingShaderGroupTypeKHR = u32;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: VkRayTracingShaderGroupTypeKHR = 0;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: VkRayTracingShaderGroupTypeKHR = 1;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: VkRayTracingShaderGroupTypeKHR = 2;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_NV: VkRayTracingShaderGroupTypeKHR = 0;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_NV: VkRayTracingShaderGroupTypeKHR = 1;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_NV: VkRayTracingShaderGroupTypeKHR = 2;
pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_MAX_ENUM_KHR: VkRayTracingShaderGroupTypeKHR = 2147483647;

pub use VkRayTracingShaderGroupTypeKHR as VkRayTracingShaderGroupTypeNV;

pub type VkGeometryTypeKHR = u32;
pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR: VkGeometryTypeKHR = 0;
pub const VK_GEOMETRY_TYPE_AABBS_KHR: VkGeometryTypeKHR = 1;
pub const VK_GEOMETRY_TYPE_INSTANCES_KHR: VkGeometryTypeKHR = 2;
pub const VK_GEOMETRY_TYPE_TRIANGLES_NV: VkGeometryTypeKHR = 0;
pub const VK_GEOMETRY_TYPE_AABBS_NV: VkGeometryTypeKHR = 1;
pub const VK_GEOMETRY_TYPE_MAX_ENUM_KHR: VkGeometryTypeKHR = 2147483647;

pub use VkGeometryTypeKHR as VkGeometryTypeNV;

pub type VkAccelerationStructureTypeKHR = u32;
pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: VkAccelerationStructureTypeKHR = 0;
pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: VkAccelerationStructureTypeKHR = 1;
pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: VkAccelerationStructureTypeKHR = 2;
pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV: VkAccelerationStructureTypeKHR = 0;
pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV: VkAccelerationStructureTypeKHR = 1;
pub const VK_ACCELERATION_STRUCTURE_TYPE_MAX_ENUM_KHR: VkAccelerationStructureTypeKHR = 2147483647;

pub use VkAccelerationStructureTypeKHR as VkAccelerationStructureTypeNV;

pub type VkCopyAccelerationStructureModeKHR = u32;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: VkCopyAccelerationStructureModeKHR = 0;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: VkCopyAccelerationStructureModeKHR = 1;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR: VkCopyAccelerationStructureModeKHR = 2;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR: VkCopyAccelerationStructureModeKHR = 3;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV: VkCopyAccelerationStructureModeKHR = 0;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV: VkCopyAccelerationStructureModeKHR = 1;
pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_MAX_ENUM_KHR: VkCopyAccelerationStructureModeKHR = 2147483647;

pub use VkCopyAccelerationStructureModeKHR as VkCopyAccelerationStructureModeNV;

pub type VkAccelerationStructureMemoryRequirementsTypeNV = u32;
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 0;
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 1;
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 2;
pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_MAX_ENUM_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 2147483647;

pub type VkGeometryFlagsKHR = VkFlags;
pub type VkGeometryFlagBitsKHR = VkFlags;
pub const VK_GEOMETRY_OPAQUE_BIT_KHR: VkGeometryFlagBitsKHR = 1;
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: VkGeometryFlagBitsKHR = 2;
pub const VK_GEOMETRY_OPAQUE_BIT_NV: VkGeometryFlagBitsKHR = 1;
pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV: VkGeometryFlagBitsKHR = 2;
pub const VK_GEOMETRY_FLAG_BITS_MAX_ENUM_KHR: VkGeometryFlagBitsKHR = 2147483647;

pub type VkGeometryFlagsNV = VkGeometryFlagsKHR;
pub use VkGeometryFlagBitsKHR as VkGeometryFlagBitsNV;

pub type VkGeometryInstanceFlagsKHR = VkFlags;
pub type VkGeometryInstanceFlagBitsKHR = VkFlags;
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = 1;
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR: VkGeometryInstanceFlagBitsKHR = 2;
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = 4;
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = 8;
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: VkGeometryInstanceFlagBitsKHR = 2;
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV: VkGeometryInstanceFlagBitsKHR = 1;
pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV: VkGeometryInstanceFlagBitsKHR = 2;
pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR = 4;
pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV: VkGeometryInstanceFlagBitsKHR = 8;
pub const VK_GEOMETRY_INSTANCE_FLAG_BITS_MAX_ENUM_KHR: VkGeometryInstanceFlagBitsKHR = 2147483647;

pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR;
pub use VkGeometryInstanceFlagBitsKHR as VkGeometryInstanceFlagBitsNV;

pub type VkBuildAccelerationStructureFlagsKHR = VkFlags;
pub type VkBuildAccelerationStructureFlagBitsKHR = VkFlags;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = 1;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = 2;
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = 4;
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = 8;
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR: VkBuildAccelerationStructureFlagBitsKHR = 16;
pub const VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 32;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 1;
pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 2;
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 4;
pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 8;
pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV: VkBuildAccelerationStructureFlagBitsKHR = 16;
pub const VK_BUILD_ACCELERATION_STRUCTURE_FLAG_BITS_MAX_ENUM_KHR: VkBuildAccelerationStructureFlagBitsKHR = 2147483647;

pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR;
pub use VkBuildAccelerationStructureFlagBitsKHR as VkBuildAccelerationStructureFlagBitsNV;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkRayTracingShaderGroupTypeKHR,
	pub generalShader: u32,
	pub closestHitShader: u32,
	pub anyHitShader: u32,
	pub intersectionShader: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRayTracingPipelineCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub groupCount: u32,
	pub pGroups: *const VkRayTracingShaderGroupCreateInfoNV,
	pub maxRecursionDepth: u32,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeometryTrianglesNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub vertexData: VkBuffer,
	pub vertexOffset: VkDeviceSize,
	pub vertexCount: u32,
	pub vertexStride: VkDeviceSize,
	pub vertexFormat: VkFormat,
	pub indexData: VkBuffer,
	pub indexOffset: VkDeviceSize,
	pub indexCount: u32,
	pub indexType: VkIndexType,
	pub transformData: VkBuffer,
	pub transformOffset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeometryAABBNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub aabbData: VkBuffer,
	pub numAABBs: u32,
	pub stride: u32,
	pub offset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeometryDataNV {
	pub triangles: VkGeometryTrianglesNV,
	pub aabbs: VkGeometryAABBNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeometryNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub geometryType: VkGeometryTypeKHR,
	pub geometry: VkGeometryDataNV,
	pub flags: VkGeometryFlagsKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkAccelerationStructureTypeNV,
	pub flags: VkBuildAccelerationStructureFlagsNV,
	pub instanceCount: u32,
	pub geometryCount: u32,
	pub pGeometries: *const VkGeometryNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub compactedSize: VkDeviceSize,
	pub info: VkAccelerationStructureInfoNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub accelerationStructure: VkAccelerationStructureNV,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub deviceIndexCount: u32,
	pub pDeviceIndices: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub accelerationStructureCount: u32,
	pub pAccelerationStructures: *const VkAccelerationStructureNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkAccelerationStructureMemoryRequirementsTypeNV,
	pub accelerationStructure: VkAccelerationStructureNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderGroupHandleSize: u32,
	pub maxRecursionDepth: u32,
	pub maxShaderGroupStride: u32,
	pub shaderGroupBaseAlignment: u32,
	pub maxGeometryCount: u64,
	pub maxInstanceCount: u64,
	pub maxTriangleCount: u64,
	pub maxDescriptorSetAccelerationStructures: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkTransformMatrixKHR {
	pub matrix: [[f32; 4]; 3],
}

pub type VkTransformMatrixNV = VkTransformMatrixKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAabbPositionsKHR {
	pub minX: f32,
	pub minY: f32,
	pub minZ: f32,
	pub maxX: f32,
	pub maxY: f32,
	pub maxZ: f32,
}

pub type VkAabbPositionsNV = VkAabbPositionsKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureInstanceKHR {
	pub transform: VkTransformMatrixKHR,
	pub instanceCustomIndex: u32,
	pub mask: u32,
	pub instanceShaderBindingTableRecordOffset: u32,
	pub flags: VkGeometryInstanceFlagsKHR,
	pub accelerationStructureReference: u64,
}

pub type VkAccelerationStructureInstanceNV = VkAccelerationStructureInstanceKHR;

pub type PFN_vkCreateAccelerationStructureNV = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pAccelerationStructure: *mut VkAccelerationStructureNV,
) -> VkResult;

pub type PFN_vkDestroyAccelerationStructureNV = unsafe extern "C" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkGetAccelerationStructureMemoryRequirementsNV =
	unsafe extern "C" fn(device: VkDevice, pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV, pMemoryRequirements: *mut VkMemoryRequirements2KHR);

pub type PFN_vkBindAccelerationStructureMemoryNV = unsafe extern "C" fn(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV) -> VkResult;

pub type PFN_vkCmdBuildAccelerationStructureNV = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	pInfo: *const VkAccelerationStructureInfoNV,
	instanceData: VkBuffer,
	instanceOffset: VkDeviceSize,
	update: VkBool32,
	dst: VkAccelerationStructureNV,
	src: VkAccelerationStructureNV,
	scratch: VkBuffer,
	scratchOffset: VkDeviceSize,
);

pub type PFN_vkCmdCopyAccelerationStructureNV =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, dst: VkAccelerationStructureNV, src: VkAccelerationStructureNV, mode: VkCopyAccelerationStructureModeKHR);

pub type PFN_vkCmdTraceRaysNV = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	raygenShaderBindingTableBuffer: VkBuffer,
	raygenShaderBindingOffset: VkDeviceSize,
	missShaderBindingTableBuffer: VkBuffer,
	missShaderBindingOffset: VkDeviceSize,
	missShaderBindingStride: VkDeviceSize,
	hitShaderBindingTableBuffer: VkBuffer,
	hitShaderBindingOffset: VkDeviceSize,
	hitShaderBindingStride: VkDeviceSize,
	callableShaderBindingTableBuffer: VkBuffer,
	callableShaderBindingOffset: VkDeviceSize,
	callableShaderBindingStride: VkDeviceSize,
	width: u32,
	height: u32,
	depth: u32,
);

pub type PFN_vkCreateRayTracingPipelinesNV = unsafe extern "C" fn(
	device: VkDevice,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *mut VkPipeline,
) -> VkResult;

pub type PFN_vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

pub type PFN_vkGetRayTracingShaderGroupHandlesNV = unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

pub type PFN_vkGetAccelerationStructureHandleNV = unsafe extern "C" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, dataSize: usize, pData: *mut ()) -> VkResult;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureNV,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32,
);

pub type PFN_vkCompileDeferredNV = unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, shader: u32) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateAccelerationStructureNV(
		device: VkDevice,
		pCreateInfo: *const VkAccelerationStructureCreateInfoNV,
		pAllocator: *const VkAllocationCallbacks,
		pAccelerationStructure: *mut VkAccelerationStructureNV,
	) -> VkResult;

	pub fn vkDestroyAccelerationStructureNV(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, pAllocator: *const VkAllocationCallbacks);

	pub fn vkGetAccelerationStructureMemoryRequirementsNV(device: VkDevice, pInfo: *const VkAccelerationStructureMemoryRequirementsInfoNV, pMemoryRequirements: *mut VkMemoryRequirements2KHR);

	pub fn vkBindAccelerationStructureMemoryNV(device: VkDevice, bindInfoCount: u32, pBindInfos: *const VkBindAccelerationStructureMemoryInfoNV) -> VkResult;

	pub fn vkCmdBuildAccelerationStructureNV(
		commandBuffer: VkCommandBuffer,
		pInfo: *const VkAccelerationStructureInfoNV,
		instanceData: VkBuffer,
		instanceOffset: VkDeviceSize,
		update: VkBool32,
		dst: VkAccelerationStructureNV,
		src: VkAccelerationStructureNV,
		scratch: VkBuffer,
		scratchOffset: VkDeviceSize,
	);

	pub fn vkCmdCopyAccelerationStructureNV(commandBuffer: VkCommandBuffer, dst: VkAccelerationStructureNV, src: VkAccelerationStructureNV, mode: VkCopyAccelerationStructureModeKHR);

	pub fn vkCmdTraceRaysNV(
		commandBuffer: VkCommandBuffer,
		raygenShaderBindingTableBuffer: VkBuffer,
		raygenShaderBindingOffset: VkDeviceSize,
		missShaderBindingTableBuffer: VkBuffer,
		missShaderBindingOffset: VkDeviceSize,
		missShaderBindingStride: VkDeviceSize,
		hitShaderBindingTableBuffer: VkBuffer,
		hitShaderBindingOffset: VkDeviceSize,
		hitShaderBindingStride: VkDeviceSize,
		callableShaderBindingTableBuffer: VkBuffer,
		callableShaderBindingOffset: VkDeviceSize,
		callableShaderBindingStride: VkDeviceSize,
		width: u32,
		height: u32,
		depth: u32,
	);

	pub fn vkCreateRayTracingPipelinesNV(
		device: VkDevice,
		pipelineCache: VkPipelineCache,
		createInfoCount: u32,
		pCreateInfos: *const VkRayTracingPipelineCreateInfoNV,
		pAllocator: *const VkAllocationCallbacks,
		pPipelines: *mut VkPipeline,
	) -> VkResult;

	pub fn vkGetRayTracingShaderGroupHandlesKHR(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

	pub fn vkGetRayTracingShaderGroupHandlesNV(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

	pub fn vkGetAccelerationStructureHandleNV(device: VkDevice, accelerationStructure: VkAccelerationStructureNV, dataSize: usize, pData: *mut ()) -> VkResult;

	pub fn vkCmdWriteAccelerationStructuresPropertiesNV(
		commandBuffer: VkCommandBuffer,
		accelerationStructureCount: u32,
		pAccelerationStructures: *const VkAccelerationStructureNV,
		queryType: VkQueryType,
		queryPool: VkQueryPool,
		firstQuery: u32,
	);

	pub fn vkCompileDeferredNV(device: VkDevice, pipeline: VkPipeline, shader: u32) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub representativeFragmentTest: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub representativeFragmentTestEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub imageViewType: VkImageViewType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub filterCubic: VkBool32,
	pub filterCubicMinmax: VkBool32,
}

pub use VkQueueGlobalPriorityKHR as VkQueueGlobalPriorityEXT;

pub type VkDeviceQueueGlobalPriorityCreateInfoEXT = VkDeviceQueueGlobalPriorityCreateInfoKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImportMemoryHostPointerInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
	pub pHostPointer: *mut (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryHostPointerPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryTypeBits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minImportedHostPointerAlignment: VkDeviceSize,
}

pub type PFN_vkGetMemoryHostPointerPropertiesEXT =
	unsafe extern "C" fn(device: VkDevice, handleType: VkExternalMemoryHandleTypeFlagBits, pHostPointer: *const (), pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetMemoryHostPointerPropertiesEXT(
		device: VkDevice,
		handleType: VkExternalMemoryHandleTypeFlagBits,
		pHostPointer: *const (),
		pMemoryHostPointerProperties: *mut VkMemoryHostPointerPropertiesEXT,
	) -> VkResult;
}

pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdWriteBufferMarkerAMD(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, marker: u32);
}

pub const VkPipelineCompilerControlFlagBitsAMD_VK_PIPELINE_COMPILER_CONTROL_FLAG_BITS_MAX_ENUM_AMD: VkPipelineCompilerControlFlagBitsAMD = 2147483647;
pub type VkPipelineCompilerControlFlagBitsAMD = u32;
pub type VkPipelineCompilerControlFlagsAMD = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub compilerControlFlags: VkPipelineCompilerControlFlagsAMD,
}

pub type VkTimeDomainEXT = u32;
pub const VK_TIME_DOMAIN_DEVICE_EXT: VkTimeDomainEXT = 0;
pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT: VkTimeDomainEXT = 1;
pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT: VkTimeDomainEXT = 2;
pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT: VkTimeDomainEXT = 3;
pub const VK_TIME_DOMAIN_MAX_ENUM_EXT: VkTimeDomainEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCalibratedTimestampInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub timeDomain: VkTimeDomainEXT,
}

pub type PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pTimeDomainCount: *mut u32, pTimeDomains: *mut VkTimeDomainEXT) -> VkResult;

pub type PFN_vkGetCalibratedTimestampsEXT =
	unsafe extern "C" fn(device: VkDevice, timestampCount: u32, pTimestampInfos: *const VkCalibratedTimestampInfoEXT, pTimestamps: *mut u64, pMaxDeviation: *mut u64) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(physicalDevice: VkPhysicalDevice, pTimeDomainCount: *mut u32, pTimeDomains: *mut VkTimeDomainEXT) -> VkResult;

	pub fn vkGetCalibratedTimestampsEXT(device: VkDevice, timestampCount: u32, pTimestampInfos: *const VkCalibratedTimestampInfoEXT, pTimestamps: *mut u64, pMaxDeviation: *mut u64) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderEngineCount: u32,
	pub shaderArraysPerEngineCount: u32,
	pub computeUnitsPerShaderArray: u32,
	pub simdPerComputeUnit: u32,
	pub wavefrontsPerSimd: u32,
	pub wavefrontSize: u32,
	pub sgprsPerSimd: u32,
	pub minSgprAllocation: u32,
	pub maxSgprAllocation: u32,
	pub sgprAllocationGranularity: u32,
	pub vgprsPerSimd: u32,
	pub minVgprAllocation: u32,
	pub maxVgprAllocation: u32,
	pub vgprAllocationGranularity: u32,
}

pub type VkMemoryOverallocationBehaviorAMD = u32;
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: VkMemoryOverallocationBehaviorAMD = 0;
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: VkMemoryOverallocationBehaviorAMD = 1;
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: VkMemoryOverallocationBehaviorAMD = 2;
pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_MAX_ENUM_AMD: VkMemoryOverallocationBehaviorAMD = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub overallocationBehavior: VkMemoryOverallocationBehaviorAMD,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxVertexAttribDivisor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
	pub binding: u32,
	pub divisor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub vertexBindingDivisorCount: u32,
	pub pVertexBindingDivisors: *const VkVertexInputBindingDivisorDescriptionEXT,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub vertexAttributeInstanceRateDivisor: VkBool32,
	pub vertexAttributeInstanceRateZeroDivisor: VkBool32,
}

pub use VkPipelineCreationFeedbackFlagBits as VkPipelineCreationFeedbackFlagBitsEXT;

pub type VkPipelineCreationFeedbackFlagsEXT = VkPipelineCreationFeedbackFlags;
pub type VkPipelineCreationFeedbackCreateInfoEXT = VkPipelineCreationFeedbackCreateInfo;
pub type VkPipelineCreationFeedbackEXT = VkPipelineCreationFeedback;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub computeDerivativeGroupQuads: VkBool32,
	pub computeDerivativeGroupLinear: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub taskShader: VkBool32,
	pub meshShader: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxDrawMeshTasksCount: u32,
	pub maxTaskWorkGroupInvocations: u32,
	pub maxTaskWorkGroupSize: [u32; 3],
	pub maxTaskTotalMemorySize: u32,
	pub maxTaskOutputCount: u32,
	pub maxMeshWorkGroupInvocations: u32,
	pub maxMeshWorkGroupSize: [u32; 3],
	pub maxMeshTotalMemorySize: u32,
	pub maxMeshOutputVertices: u32,
	pub maxMeshOutputPrimitives: u32,
	pub maxMeshMultiviewViewCount: u32,
	pub meshOutputPerVertexGranularity: u32,
	pub meshOutputPerPrimitiveGranularity: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDrawMeshTasksIndirectCommandNV {
	pub taskCount: u32,
	pub firstTask: u32,
}

pub type PFN_vkCmdDrawMeshTasksNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, taskCount: u32, firstTask: u32);

pub type PFN_vkCmdDrawMeshTasksIndirectNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

pub type PFN_vkCmdDrawMeshTasksIndirectCountNV =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, countBuffer: VkBuffer, countBufferOffset: VkDeviceSize, maxDrawCount: u32, stride: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdDrawMeshTasksNV(commandBuffer: VkCommandBuffer, taskCount: u32, firstTask: u32);

	pub fn vkCmdDrawMeshTasksIndirectNV(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);

	pub fn vkCmdDrawMeshTasksIndirectCountNV(
		commandBuffer: VkCommandBuffer,
		buffer: VkBuffer,
		offset: VkDeviceSize,
		countBuffer: VkBuffer,
		countBufferOffset: VkDeviceSize,
		maxDrawCount: u32,
		stride: u32,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentShaderBarycentric: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub imageFootprint: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub exclusiveScissorCount: u32,
	pub pExclusiveScissors: *const VkRect2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub exclusiveScissor: VkBool32,
}

pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, firstExclusiveScissor: u32, exclusiveScissorCount: u32, pExclusiveScissors: *const VkRect2D);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetExclusiveScissorNV(commandBuffer: VkCommandBuffer, firstExclusiveScissor: u32, exclusiveScissorCount: u32, pExclusiveScissors: *const VkRect2D);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub checkpointExecutionStageMask: VkPipelineStageFlags,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCheckpointDataNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub stage: VkPipelineStageFlagBits,
	pub pCheckpointMarker: *mut (),
}

pub type PFN_vkCmdSetCheckpointNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pCheckpointMarker: *const ());

pub type PFN_vkGetQueueCheckpointDataNV = unsafe extern "C" fn(queue: VkQueue, pCheckpointDataCount: *mut u32, pCheckpointData: *mut VkCheckpointDataNV);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetCheckpointNV(commandBuffer: VkCommandBuffer, pCheckpointMarker: *const ());

	pub fn vkGetQueueCheckpointDataNV(queue: VkQueue, pCheckpointDataCount: *mut u32, pCheckpointData: *mut VkCheckpointDataNV);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderIntegerFunctions2: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceConfigurationINTEL_T {
	_unused: [u8; 0],
}

pub type VkPerformanceConfigurationINTEL = *mut VkPerformanceConfigurationINTEL_T;

pub type VkPerformanceConfigurationTypeINTEL = u32;
pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: VkPerformanceConfigurationTypeINTEL = 0;
pub const VK_PERFORMANCE_CONFIGURATION_TYPE_MAX_ENUM_INTEL: VkPerformanceConfigurationTypeINTEL = 2147483647;

pub type VkQueryPoolSamplingModeINTEL = u32;
pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: VkQueryPoolSamplingModeINTEL = 0;
pub const VK_QUERY_POOL_SAMPLING_MODE_MAX_ENUM_INTEL: VkQueryPoolSamplingModeINTEL = 2147483647;

pub type VkPerformanceOverrideTypeINTEL = u32;
pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: VkPerformanceOverrideTypeINTEL = 0;
pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: VkPerformanceOverrideTypeINTEL = 1;
pub const VK_PERFORMANCE_OVERRIDE_TYPE_MAX_ENUM_INTEL: VkPerformanceOverrideTypeINTEL = 2147483647;

pub type VkPerformanceParameterTypeINTEL = u32;
pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: VkPerformanceParameterTypeINTEL = 0;
pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: VkPerformanceParameterTypeINTEL = 1;
pub const VK_PERFORMANCE_PARAMETER_TYPE_MAX_ENUM_INTEL: VkPerformanceParameterTypeINTEL = 2147483647;

pub type VkPerformanceValueTypeINTEL = u32;
pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: VkPerformanceValueTypeINTEL = 0;
pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: VkPerformanceValueTypeINTEL = 1;
pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: VkPerformanceValueTypeINTEL = 2;
pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: VkPerformanceValueTypeINTEL = 3;
pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: VkPerformanceValueTypeINTEL = 4;
pub const VK_PERFORMANCE_VALUE_TYPE_MAX_ENUM_INTEL: VkPerformanceValueTypeINTEL = 2147483647;

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkPerformanceValueDataINTEL {
	pub value32: u32,
	pub value64: u64,
	pub valueFloat: f32,
	pub valueBool: VkBool32,
	pub valueString: *const i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkPerformanceValueINTEL {
	pub type_: VkPerformanceValueTypeINTEL,
	pub data: VkPerformanceValueDataINTEL,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkInitializePerformanceApiInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pUserData: *mut (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub performanceCountersSampling: VkQueryPoolSamplingModeINTEL,
}

pub type VkQueryPoolCreateInfoINTEL = VkQueryPoolPerformanceQueryCreateInfoINTEL;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceMarkerInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub marker: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub marker: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceOverrideInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkPerformanceOverrideTypeINTEL,
	pub enable: VkBool32,
	pub parameter: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkPerformanceConfigurationTypeINTEL,
}

pub type PFN_vkInitializePerformanceApiINTEL = unsafe extern "C" fn(device: VkDevice, pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL) -> VkResult;

pub type PFN_vkUninitializePerformanceApiINTEL = unsafe extern "C" fn(device: VkDevice);

pub type PFN_vkCmdSetPerformanceMarkerINTEL = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceMarkerInfoINTEL) -> VkResult;

pub type PFN_vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL) -> VkResult;

pub type PFN_vkCmdSetPerformanceOverrideINTEL = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pOverrideInfo: *const VkPerformanceOverrideInfoINTEL) -> VkResult;

pub type PFN_vkAcquirePerformanceConfigurationINTEL =
	unsafe extern "C" fn(device: VkDevice, pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL, pConfiguration: *mut VkPerformanceConfigurationINTEL) -> VkResult;

pub type PFN_vkReleasePerformanceConfigurationINTEL = unsafe extern "C" fn(device: VkDevice, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

pub type PFN_vkQueueSetPerformanceConfigurationINTEL = unsafe extern "C" fn(queue: VkQueue, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

pub type PFN_vkGetPerformanceParameterINTEL = unsafe extern "C" fn(device: VkDevice, parameter: VkPerformanceParameterTypeINTEL, pValue: *mut VkPerformanceValueINTEL) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkInitializePerformanceApiINTEL(device: VkDevice, pInitializeInfo: *const VkInitializePerformanceApiInfoINTEL) -> VkResult;

	pub fn vkUninitializePerformanceApiINTEL(device: VkDevice);

	pub fn vkCmdSetPerformanceMarkerINTEL(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceMarkerInfoINTEL) -> VkResult;

	pub fn vkCmdSetPerformanceStreamMarkerINTEL(commandBuffer: VkCommandBuffer, pMarkerInfo: *const VkPerformanceStreamMarkerInfoINTEL) -> VkResult;

	pub fn vkCmdSetPerformanceOverrideINTEL(commandBuffer: VkCommandBuffer, pOverrideInfo: *const VkPerformanceOverrideInfoINTEL) -> VkResult;

	pub fn vkAcquirePerformanceConfigurationINTEL(device: VkDevice, pAcquireInfo: *const VkPerformanceConfigurationAcquireInfoINTEL, pConfiguration: *mut VkPerformanceConfigurationINTEL) -> VkResult;

	pub fn vkReleasePerformanceConfigurationINTEL(device: VkDevice, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

	pub fn vkQueueSetPerformanceConfigurationINTEL(queue: VkQueue, configuration: VkPerformanceConfigurationINTEL) -> VkResult;

	pub fn vkGetPerformanceParameterINTEL(device: VkDevice, parameter: VkPerformanceParameterTypeINTEL, pValue: *mut VkPerformanceValueINTEL) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pciDomain: u32,
	pub pciBus: u32,
	pub pciDevice: u32,
	pub pciFunction: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub localDimmingSupport: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub localDimmingEnable: VkBool32,
}

pub type PFN_vkSetLocalDimmingAMD = unsafe extern "C" fn(device: VkDevice, swapChain: VkSwapchainKHR, localDimmingEnable: VkBool32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkSetLocalDimmingAMD(device: VkDevice, swapChain: VkSwapchainKHR, localDimmingEnable: VkBool32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentDensityMap: VkBool32,
	pub fragmentDensityMapDynamic: VkBool32,
	pub fragmentDensityMapNonSubsampledImages: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minFragmentDensityTexelSize: VkExtent2D,
	pub maxFragmentDensityTexelSize: VkExtent2D,
	pub fragmentDensityInvocations: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub fragmentDensityMapAttachment: VkAttachmentReference,
}

pub type VkPhysicalDeviceScalarBlockLayoutFeaturesEXT = VkPhysicalDeviceScalarBlockLayoutFeatures;
pub type VkPhysicalDeviceSubgroupSizeControlFeaturesEXT = VkPhysicalDeviceSubgroupSizeControlFeatures;
pub type VkPhysicalDeviceSubgroupSizeControlPropertiesEXT = VkPhysicalDeviceSubgroupSizeControlProperties;
pub type VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT = VkPipelineShaderStageRequiredSubgroupSizeCreateInfo;

pub type VkShaderCorePropertiesFlagsAMD = VkFlags;
pub type VkShaderCorePropertiesFlagBitsAMD = VkFlags;
pub const VK_SHADER_CORE_PROPERTIES_FLAG_BITS_MAX_ENUM_AMD: VkShaderCorePropertiesFlagBitsAMD = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderCoreFeatures: VkShaderCorePropertiesFlagsAMD,
	pub activeComputeUnitCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceCoherentMemory: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderImageInt64Atomics: VkBool32,
	pub sparseImageInt64Atomics: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub heapBudget: [VkDeviceSize; 16],
	pub heapUsage: [VkDeviceSize; 16],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub memoryPriority: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryPriorityAllocateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub priority: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub dedicatedAllocationImageAliasing: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub bufferDeviceAddress: VkBool32,
	pub bufferDeviceAddressCaptureReplay: VkBool32,
	pub bufferDeviceAddressMultiDevice: VkBool32,
}

pub type VkPhysicalDeviceBufferAddressFeaturesEXT = VkPhysicalDeviceBufferDeviceAddressFeaturesEXT;
pub type VkBufferDeviceAddressInfoEXT = VkBufferDeviceAddressInfo;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub deviceAddress: VkDeviceAddress,
}

pub type PFN_vkGetBufferDeviceAddressEXT = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetBufferDeviceAddressEXT(device: VkDevice, pInfo: *const VkBufferDeviceAddressInfo) -> VkDeviceAddress;
}

pub use VkToolPurposeFlagBits as VkToolPurposeFlagBitsEXT;

pub type VkToolPurposeFlagsEXT = VkToolPurposeFlags;
pub type VkPhysicalDeviceToolPropertiesEXT = VkPhysicalDeviceToolProperties;

pub type PFN_vkGetPhysicalDeviceToolPropertiesEXT = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceToolPropertiesEXT(physicalDevice: VkPhysicalDevice, pToolCount: *mut u32, pToolProperties: *mut VkPhysicalDeviceToolProperties) -> VkResult;
}

pub type VkImageStencilUsageCreateInfoEXT = VkImageStencilUsageCreateInfo;

pub type VkValidationFeatureEnableEXT = u32;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: VkValidationFeatureEnableEXT = 0;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: VkValidationFeatureEnableEXT = 1;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: VkValidationFeatureEnableEXT = 2;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: VkValidationFeatureEnableEXT = 3;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: VkValidationFeatureEnableEXT = 4;
pub const VkValidationFeatureEnableEXT_VK_VALIDATION_FEATURE_ENABLE_MAX_ENUM_EXT: VkValidationFeatureEnableEXT = 2147483647;

pub type VkValidationFeatureDisableEXT = u32;
pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: VkValidationFeatureDisableEXT = 0;
pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: VkValidationFeatureDisableEXT = 1;
pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: VkValidationFeatureDisableEXT = 2;
pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: VkValidationFeatureDisableEXT = 3;
pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: VkValidationFeatureDisableEXT = 4;
pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: VkValidationFeatureDisableEXT = 5;
pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: VkValidationFeatureDisableEXT = 6;
pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: VkValidationFeatureDisableEXT = 7;
pub const VK_VALIDATION_FEATURE_DISABLE_MAX_ENUM_EXT: VkValidationFeatureDisableEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkValidationFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub enabledValidationFeatureCount: u32,
	pub pEnabledValidationFeatures: *const VkValidationFeatureEnableEXT,
	pub disabledValidationFeatureCount: u32,
	pub pDisabledValidationFeatures: *const VkValidationFeatureDisableEXT,
}

pub type VkComponentTypeNV = u32;
pub const VK_COMPONENT_TYPE_FLOAT16_NV: VkComponentTypeNV = 0;
pub const VK_COMPONENT_TYPE_FLOAT32_NV: VkComponentTypeNV = 1;
pub const VK_COMPONENT_TYPE_FLOAT64_NV: VkComponentTypeNV = 2;
pub const VK_COMPONENT_TYPE_SINT8_NV: VkComponentTypeNV = 3;
pub const VK_COMPONENT_TYPE_SINT16_NV: VkComponentTypeNV = 4;
pub const VK_COMPONENT_TYPE_SINT32_NV: VkComponentTypeNV = 5;
pub const VK_COMPONENT_TYPE_SINT64_NV: VkComponentTypeNV = 6;
pub const VK_COMPONENT_TYPE_UINT8_NV: VkComponentTypeNV = 7;
pub const VK_COMPONENT_TYPE_UINT16_NV: VkComponentTypeNV = 8;
pub const VK_COMPONENT_TYPE_UINT32_NV: VkComponentTypeNV = 9;
pub const VK_COMPONENT_TYPE_UINT64_NV: VkComponentTypeNV = 10;
pub const VK_COMPONENT_TYPE_MAX_ENUM_NV: VkComponentTypeNV = 2147483647;

pub type VkScopeNV = u32;
pub const VK_SCOPE_DEVICE_NV: VkScopeNV = 1;
pub const VK_SCOPE_WORKGROUP_NV: VkScopeNV = 2;
pub const VK_SCOPE_SUBGROUP_NV: VkScopeNV = 3;
pub const VK_SCOPE_QUEUE_FAMILY_NV: VkScopeNV = 5;
pub const VK_SCOPE_MAX_ENUM_NV: VkScopeNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCooperativeMatrixPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub MSize: u32,
	pub NSize: u32,
	pub KSize: u32,
	pub AType: VkComponentTypeNV,
	pub BType: VkComponentTypeNV,
	pub CType: VkComponentTypeNV,
	pub DType: VkComponentTypeNV,
	pub scope: VkScopeNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub cooperativeMatrix: VkBool32,
	pub cooperativeMatrixRobustBufferAccess: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub cooperativeMatrixSupportedStages: VkShaderStageFlags,
}

pub type PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkCooperativeMatrixPropertiesNV) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceCooperativeMatrixPropertiesNV(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkCooperativeMatrixPropertiesNV) -> VkResult;
}

pub type VkCoverageReductionModeNV = u32;
pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV: VkCoverageReductionModeNV = 0;
pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: VkCoverageReductionModeNV = 1;
pub const VK_COVERAGE_REDUCTION_MODE_MAX_ENUM_NV: VkCoverageReductionModeNV = 2147483647;

pub type VkPipelineCoverageReductionStateCreateFlagsNV = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub coverageReductionMode: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCoverageReductionStateCreateFlagsNV,
	pub coverageReductionMode: VkCoverageReductionModeNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkFramebufferMixedSamplesCombinationNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub coverageReductionMode: VkCoverageReductionModeNV,
	pub rasterizationSamples: VkSampleCountFlagBits,
	pub depthStencilSamples: VkSampleCountFlags,
	pub colorSamples: VkSampleCountFlags,
}

pub type PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
	unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, pCombinationCount: *mut u32, pCombinations: *mut VkFramebufferMixedSamplesCombinationNV) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV(
		physicalDevice: VkPhysicalDevice,
		pCombinationCount: *mut u32,
		pCombinations: *mut VkFramebufferMixedSamplesCombinationNV,
	) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentShaderSampleInterlock: VkBool32,
	pub fragmentShaderPixelInterlock: VkBool32,
	pub fragmentShaderShadingRateInterlock: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub ycbcrImageArrays: VkBool32,
}

pub type VkProvokingVertexModeEXT = u32;
pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: VkProvokingVertexModeEXT = 0;
pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: VkProvokingVertexModeEXT = 1;
pub const VK_PROVOKING_VERTEX_MODE_MAX_ENUM_EXT: VkProvokingVertexModeEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub provokingVertexLast: VkBool32,
	pub transformFeedbackPreservesProvokingVertex: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub provokingVertexModePerPipeline: VkBool32,
	pub transformFeedbackPreservesTriangleFanProvokingVertex: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub provokingVertexMode: VkProvokingVertexModeEXT,
}

pub type VkHeadlessSurfaceCreateFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkHeadlessSurfaceCreateFlagsEXT,
}

pub type PFN_vkCreateHeadlessSurfaceEXT =
	unsafe extern "C" fn(instance: VkInstance, pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateHeadlessSurfaceEXT(instance: VkInstance, pCreateInfo: *const VkHeadlessSurfaceCreateInfoEXT, pAllocator: *const VkAllocationCallbacks, pSurface: *mut VkSurfaceKHR) -> VkResult;
}

pub type VkLineRasterizationModeEXT = u32;
pub const VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT: VkLineRasterizationModeEXT = 0;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT: VkLineRasterizationModeEXT = 1;
pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT: VkLineRasterizationModeEXT = 2;
pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT: VkLineRasterizationModeEXT = 3;
pub const VK_LINE_RASTERIZATION_MODE_MAX_ENUM_EXT: VkLineRasterizationModeEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub rectangularLines: VkBool32,
	pub bresenhamLines: VkBool32,
	pub smoothLines: VkBool32,
	pub stippledRectangularLines: VkBool32,
	pub stippledBresenhamLines: VkBool32,
	pub stippledSmoothLines: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub lineSubPixelPrecisionBits: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub lineRasterizationMode: VkLineRasterizationModeEXT,
	pub stippledLineEnable: VkBool32,
	pub lineStippleFactor: u32,
	pub lineStipplePattern: u16,
}

pub type PFN_vkCmdSetLineStippleEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetLineStippleEXT(commandBuffer: VkCommandBuffer, lineStippleFactor: u32, lineStipplePattern: u16);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderBufferFloat32Atomics: VkBool32,
	pub shaderBufferFloat32AtomicAdd: VkBool32,
	pub shaderBufferFloat64Atomics: VkBool32,
	pub shaderBufferFloat64AtomicAdd: VkBool32,
	pub shaderSharedFloat32Atomics: VkBool32,
	pub shaderSharedFloat32AtomicAdd: VkBool32,
	pub shaderSharedFloat64Atomics: VkBool32,
	pub shaderSharedFloat64AtomicAdd: VkBool32,
	pub shaderImageFloat32Atomics: VkBool32,
	pub shaderImageFloat32AtomicAdd: VkBool32,
	pub sparseImageFloat32Atomics: VkBool32,
	pub sparseImageFloat32AtomicAdd: VkBool32,
}

pub type VkPhysicalDeviceHostQueryResetFeaturesEXT = VkPhysicalDeviceHostQueryResetFeatures;

pub type PFN_vkResetQueryPoolEXT = unsafe extern "C" fn(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkResetQueryPoolEXT(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub indexTypeUint8: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub extendedDynamicState: VkBool32,
}

pub type PFN_vkCmdSetCullModeEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);

pub type PFN_vkCmdSetFrontFaceEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);

pub type PFN_vkCmdSetPrimitiveTopologyEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);

pub type PFN_vkCmdSetViewportWithCountEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);

pub type PFN_vkCmdSetScissorWithCountEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);

pub type PFN_vkCmdBindVertexBuffers2EXT = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	firstBinding: u32,
	bindingCount: u32,
	pBuffers: *const VkBuffer,
	pOffsets: *const VkDeviceSize,
	pSizes: *const VkDeviceSize,
	pStrides: *const VkDeviceSize,
);

pub type PFN_vkCmdSetDepthTestEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);

pub type PFN_vkCmdSetDepthWriteEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);

pub type PFN_vkCmdSetDepthCompareOpEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);

pub type PFN_vkCmdSetDepthBoundsTestEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);

pub type PFN_vkCmdSetStencilTestEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);

pub type PFN_vkCmdSetStencilOpEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetCullModeEXT(commandBuffer: VkCommandBuffer, cullMode: VkCullModeFlags);

	pub fn vkCmdSetFrontFaceEXT(commandBuffer: VkCommandBuffer, frontFace: VkFrontFace);

	pub fn vkCmdSetPrimitiveTopologyEXT(commandBuffer: VkCommandBuffer, primitiveTopology: VkPrimitiveTopology);

	pub fn vkCmdSetViewportWithCountEXT(commandBuffer: VkCommandBuffer, viewportCount: u32, pViewports: *const VkViewport);

	pub fn vkCmdSetScissorWithCountEXT(commandBuffer: VkCommandBuffer, scissorCount: u32, pScissors: *const VkRect2D);

	pub fn vkCmdBindVertexBuffers2EXT(
		commandBuffer: VkCommandBuffer,
		firstBinding: u32,
		bindingCount: u32,
		pBuffers: *const VkBuffer,
		pOffsets: *const VkDeviceSize,
		pSizes: *const VkDeviceSize,
		pStrides: *const VkDeviceSize,
	);

	pub fn vkCmdSetDepthTestEnableEXT(commandBuffer: VkCommandBuffer, depthTestEnable: VkBool32);

	pub fn vkCmdSetDepthWriteEnableEXT(commandBuffer: VkCommandBuffer, depthWriteEnable: VkBool32);

	pub fn vkCmdSetDepthCompareOpEXT(commandBuffer: VkCommandBuffer, depthCompareOp: VkCompareOp);

	pub fn vkCmdSetDepthBoundsTestEnableEXT(commandBuffer: VkCommandBuffer, depthBoundsTestEnable: VkBool32);

	pub fn vkCmdSetStencilTestEnableEXT(commandBuffer: VkCommandBuffer, stencilTestEnable: VkBool32);

	pub fn vkCmdSetStencilOpEXT(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, failOp: VkStencilOp, passOp: VkStencilOp, depthFailOp: VkStencilOp, compareOp: VkCompareOp);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderBufferFloat16Atomics: VkBool32,
	pub shaderBufferFloat16AtomicAdd: VkBool32,
	pub shaderBufferFloat16AtomicMinMax: VkBool32,
	pub shaderBufferFloat32AtomicMinMax: VkBool32,
	pub shaderBufferFloat64AtomicMinMax: VkBool32,
	pub shaderSharedFloat16Atomics: VkBool32,
	pub shaderSharedFloat16AtomicAdd: VkBool32,
	pub shaderSharedFloat16AtomicMinMax: VkBool32,
	pub shaderSharedFloat32AtomicMinMax: VkBool32,
	pub shaderSharedFloat64AtomicMinMax: VkBool32,
	pub shaderImageFloat32AtomicMinMax: VkBool32,
	pub sparseImageFloat32AtomicMinMax: VkBool32,
}

pub type VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT = VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsLayoutNV_T {
	_unused: [u8; 0],
}

pub type VkIndirectCommandsLayoutNV = *mut VkIndirectCommandsLayoutNV_T;

pub type VkIndirectCommandsTokenTypeNV = u32;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: VkIndirectCommandsTokenTypeNV = 0;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: VkIndirectCommandsTokenTypeNV = 1;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: VkIndirectCommandsTokenTypeNV = 2;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: VkIndirectCommandsTokenTypeNV = 3;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: VkIndirectCommandsTokenTypeNV = 4;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: VkIndirectCommandsTokenTypeNV = 5;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: VkIndirectCommandsTokenTypeNV = 6;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: VkIndirectCommandsTokenTypeNV = 7;
pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_MAX_ENUM_NV: VkIndirectCommandsTokenTypeNV = 2147483647;

pub type VkIndirectStateFlagsNV = VkFlags;
pub type VkIndirectStateFlagBitsNV = VkFlags;
pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: VkIndirectStateFlagBitsNV = 1;
pub const VK_INDIRECT_STATE_FLAG_BITS_MAX_ENUM_NV: VkIndirectStateFlagBitsNV = 2147483647;

pub type VkIndirectCommandsLayoutUsageFlagsNV = VkFlags;
pub type VkIndirectCommandsLayoutUsageFlagBitsNV = VkFlags;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = 1;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = 2;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = 4;
pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_FLAG_BITS_MAX_ENUM_NV: VkIndirectCommandsLayoutUsageFlagBitsNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxGraphicsShaderGroupCount: u32,
	pub maxIndirectSequenceCount: u32,
	pub maxIndirectCommandsTokenCount: u32,
	pub maxIndirectCommandsStreamCount: u32,
	pub maxIndirectCommandsTokenOffset: u32,
	pub maxIndirectCommandsStreamStride: u32,
	pub minSequencesCountBufferOffsetAlignment: u32,
	pub minSequencesIndexBufferOffsetAlignment: u32,
	pub minIndirectCommandsBufferOffsetAlignment: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceGeneratedCommands: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub groupCount: u32,
	pub pGroups: *const VkGraphicsShaderGroupCreateInfoNV,
	pub pipelineCount: u32,
	pub pPipelines: *const VkPipeline,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindShaderGroupIndirectCommandNV {
	pub groupIndex: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindIndexBufferIndirectCommandNV {
	pub bufferAddress: VkDeviceAddress,
	pub size: u32,
	pub indexType: VkIndexType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkBindVertexBufferIndirectCommandNV {
	pub bufferAddress: VkDeviceAddress,
	pub size: u32,
	pub stride: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSetStateFlagsIndirectCommandNV {
	pub data: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsStreamNV {
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsLayoutTokenNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub tokenType: VkIndirectCommandsTokenTypeNV,
	pub stream: u32,
	pub offset: u32,
	pub vertexBindingUnit: u32,
	pub vertexDynamicStride: VkBool32,
	pub pushconstantPipelineLayout: VkPipelineLayout,
	pub pushconstantShaderStageFlags: VkShaderStageFlags,
	pub pushconstantOffset: u32,
	pub pushconstantSize: u32,
	pub indirectStateFlags: VkIndirectStateFlagsNV,
	pub indexTypeCount: u32,
	pub pIndexTypes: *const VkIndexType,
	pub pIndexTypeValues: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkIndirectCommandsLayoutUsageFlagsNV,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub tokenCount: u32,
	pub pTokens: *const VkIndirectCommandsLayoutTokenNV,
	pub streamCount: u32,
	pub pStreamStrides: *const u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeneratedCommandsInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipeline: VkPipeline,
	pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pub streamCount: u32,
	pub pStreams: *const VkIndirectCommandsStreamNV,
	pub sequencesCount: u32,
	pub preprocessBuffer: VkBuffer,
	pub preprocessOffset: VkDeviceSize,
	pub preprocessSize: VkDeviceSize,
	pub sequencesCountBuffer: VkBuffer,
	pub sequencesCountOffset: VkDeviceSize,
	pub sequencesIndexBuffer: VkBuffer,
	pub sequencesIndexOffset: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub pipeline: VkPipeline,
	pub indirectCommandsLayout: VkIndirectCommandsLayoutNV,
	pub maxSequencesCount: u32,
}

pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV =
	unsafe extern "C" fn(device: VkDevice, pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV, pMemoryRequirements: *mut VkMemoryRequirements2);

pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, isPreprocessed: VkBool32, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline, groupIndex: u32);

pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
	pAllocator: *const VkAllocationCallbacks,
	pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
) -> VkResult;

pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "C" fn(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNV, pAllocator: *const VkAllocationCallbacks);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetGeneratedCommandsMemoryRequirementsNV(device: VkDevice, pInfo: *const VkGeneratedCommandsMemoryRequirementsInfoNV, pMemoryRequirements: *mut VkMemoryRequirements2);

	pub fn vkCmdPreprocessGeneratedCommandsNV(commandBuffer: VkCommandBuffer, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

	pub fn vkCmdExecuteGeneratedCommandsNV(commandBuffer: VkCommandBuffer, isPreprocessed: VkBool32, pGeneratedCommandsInfo: *const VkGeneratedCommandsInfoNV);

	pub fn vkCmdBindPipelineShaderGroupNV(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline, groupIndex: u32);

	pub fn vkCreateIndirectCommandsLayoutNV(
		device: VkDevice,
		pCreateInfo: *const VkIndirectCommandsLayoutCreateInfoNV,
		pAllocator: *const VkAllocationCallbacks,
		pIndirectCommandsLayout: *mut VkIndirectCommandsLayoutNV,
	) -> VkResult;

	pub fn vkDestroyIndirectCommandsLayoutNV(device: VkDevice, indirectCommandsLayout: VkIndirectCommandsLayoutNV, pAllocator: *const VkAllocationCallbacks);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub inheritedViewportScissor2D: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub viewportScissor2D: VkBool32,
	pub viewportDepthCount: u32,
	pub pViewportDepths: *const VkViewport,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub texelBufferAlignment: VkBool32,
}

pub type VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT = VkPhysicalDeviceTexelBufferAlignmentProperties;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRenderPassTransformBeginInfoQCOM {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub transform: VkSurfaceTransformFlagBitsKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub transform: VkSurfaceTransformFlagBitsKHR,
	pub renderArea: VkRect2D,
}

pub type VkDeviceMemoryReportEventTypeEXT = u32;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: VkDeviceMemoryReportEventTypeEXT = 0;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: VkDeviceMemoryReportEventTypeEXT = 1;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = 2;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = 3;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: VkDeviceMemoryReportEventTypeEXT = 4;
pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_MAX_ENUM_EXT: VkDeviceMemoryReportEventTypeEXT = 2147483647;

pub type VkDeviceMemoryReportFlagsEXT = VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub deviceMemoryReport: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub flags: VkDeviceMemoryReportFlagsEXT,
	pub type_: VkDeviceMemoryReportEventTypeEXT,
	pub memoryObjectId: u64,
	pub size: VkDeviceSize,
	pub objectType: VkObjectType,
	pub objectHandle: u64,
	pub heapIndex: u32,
}

pub type PFN_vkDeviceMemoryReportCallbackEXT = unsafe extern "C" fn(pCallbackData: *const VkDeviceMemoryReportCallbackDataEXT, pUserData: *mut ());

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDeviceMemoryReportFlagsEXT,
	pub pfnUserCallback: PFN_vkDeviceMemoryReportCallbackEXT,
	pub pUserData: *mut (),
}

pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, drmFd: i32, display: VkDisplayKHR) -> VkResult;

pub type PFN_vkGetDrmDisplayEXT = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, drmFd: i32, connectorId: u32, display: *mut VkDisplayKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkAcquireDrmDisplayEXT(physicalDevice: VkPhysicalDevice, drmFd: i32, display: VkDisplayKHR) -> VkResult;

	pub fn vkGetDrmDisplayEXT(physicalDevice: VkPhysicalDevice, drmFd: i32, connectorId: u32, display: *mut VkDisplayKHR) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub robustBufferAccess2: VkBool32,
	pub robustImageAccess2: VkBool32,
	pub nullDescriptor: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub robustStorageBufferAccessSizeAlignment: VkDeviceSize,
	pub robustUniformBufferAccessSizeAlignment: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub customBorderColor: VkClearColorValue,
	pub format: VkFormat,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxCustomBorderColorSamplers: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub customBorderColors: VkBool32,
	pub customBorderColorWithoutFormat: VkBool32,
}

pub type VkPrivateDataSlotEXT = VkPrivateDataSlot;
pub type VkPrivateDataSlotCreateFlagsEXT = VkPrivateDataSlotCreateFlags;
pub type VkPhysicalDevicePrivateDataFeaturesEXT = VkPhysicalDevicePrivateDataFeatures;
pub type VkDevicePrivateDataCreateInfoEXT = VkDevicePrivateDataCreateInfo;
pub type VkPrivateDataSlotCreateInfoEXT = VkPrivateDataSlotCreateInfo;

pub type PFN_vkCreatePrivateDataSlotEXT =
	unsafe extern "C" fn(device: VkDevice, pCreateInfo: *const VkPrivateDataSlotCreateInfo, pAllocator: *const VkAllocationCallbacks, pPrivateDataSlot: *mut VkPrivateDataSlot) -> VkResult;

pub type PFN_vkDestroyPrivateDataSlotEXT = unsafe extern "C" fn(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkSetPrivateDataEXT = unsafe extern "C" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;

pub type PFN_vkGetPrivateDataEXT = unsafe extern "C" fn(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreatePrivateDataSlotEXT(
		device: VkDevice,
		pCreateInfo: *const VkPrivateDataSlotCreateInfo,
		pAllocator: *const VkAllocationCallbacks,
		pPrivateDataSlot: *mut VkPrivateDataSlot,
	) -> VkResult;

	pub fn vkDestroyPrivateDataSlotEXT(device: VkDevice, privateDataSlot: VkPrivateDataSlot, pAllocator: *const VkAllocationCallbacks);

	pub fn vkSetPrivateDataEXT(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, data: u64) -> VkResult;

	pub fn vkGetPrivateDataEXT(device: VkDevice, objectType: VkObjectType, objectHandle: u64, privateDataSlot: VkPrivateDataSlot, pData: *mut u64);
}

pub type VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT = VkPhysicalDevicePipelineCreationCacheControlFeatures;

pub type VkDeviceDiagnosticsConfigFlagsNV = VkFlags;
pub type VkDeviceDiagnosticsConfigFlagBitsNV = VkFlags;
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = 1;
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = 2;
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: VkDeviceDiagnosticsConfigFlagBitsNV = 4;
pub const VK_DEVICE_DIAGNOSTICS_CONFIG_FLAG_BITS_MAX_ENUM_NV: VkDeviceDiagnosticsConfigFlagBitsNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub diagnosticsConfig: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkDeviceDiagnosticsConfigFlagsNV,
}

pub type VkGraphicsPipelineLibraryFlagsEXT = VkFlags;
pub type VkGraphicsPipelineLibraryFlagBitsEXT = VkFlags;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = 1;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = 2;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = 4;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = 8;
pub const VK_GRAPHICS_PIPELINE_LIBRARY_FLAG_BITS_MAX_ENUM_EXT: VkGraphicsPipelineLibraryFlagBitsEXT = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub graphicsPipelineLibrary: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub graphicsPipelineLibraryFastLinking: VkBool32,
	pub graphicsPipelineLibraryIndependentInterpolationDecoration: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub flags: VkGraphicsPipelineLibraryFlagsEXT,
}

pub type VkFragmentShadingRateTypeNV = u32;
pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: VkFragmentShadingRateTypeNV = 0;
pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: VkFragmentShadingRateTypeNV = 1;
pub const VK_FRAGMENT_SHADING_RATE_TYPE_MAX_ENUM_NV: VkFragmentShadingRateTypeNV = 2147483647;

pub type VkFragmentShadingRateNV = u32;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: VkFragmentShadingRateNV = 0;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: VkFragmentShadingRateNV = 1;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: VkFragmentShadingRateNV = 4;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: VkFragmentShadingRateNV = 5;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: VkFragmentShadingRateNV = 6;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: VkFragmentShadingRateNV = 9;
pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: VkFragmentShadingRateNV = 10;
pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 11;
pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 12;
pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 13;
pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 14;
pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: VkFragmentShadingRateNV = 15;
pub const VK_FRAGMENT_SHADING_RATE_MAX_ENUM_NV: VkFragmentShadingRateNV = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentShadingRateEnums: VkBool32,
	pub supersampleFragmentShadingRates: VkBool32,
	pub noInvocationFragmentShadingRates: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxFragmentShadingRateInvocationCount: VkSampleCountFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub shadingRateType: VkFragmentShadingRateTypeNV,
	pub shadingRate: VkFragmentShadingRateNV,
	pub combinerOps: [VkFragmentShadingRateCombinerOpKHR; 2],
}

pub type PFN_vkCmdSetFragmentShadingRateEnumNV = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, shadingRate: VkFragmentShadingRateNV, combinerOps: *const VkFragmentShadingRateCombinerOpKHR);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetFragmentShadingRateEnumNV(commandBuffer: VkCommandBuffer, shadingRate: VkFragmentShadingRateNV, combinerOps: *const VkFragmentShadingRateCombinerOpKHR);
}

pub type VkAccelerationStructureMotionInstanceTypeNV = u32;
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: VkAccelerationStructureMotionInstanceTypeNV = 0;
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: VkAccelerationStructureMotionInstanceTypeNV = 1;
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: VkAccelerationStructureMotionInstanceTypeNV = 2;
pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MAX_ENUM_NV: VkAccelerationStructureMotionInstanceTypeNV = 2147483647;

pub type VkAccelerationStructureMotionInfoFlagsNV = VkFlags;
pub type VkAccelerationStructureMotionInstanceFlagsNV = VkFlags;

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkDeviceOrHostAddressConstKHR {
	pub deviceAddress: VkDeviceAddress,
	pub hostAddress: *const (),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub vertexData: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureMotionInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub maxInstances: u32,
	pub flags: VkAccelerationStructureMotionInfoFlagsNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
	pub transformT0: VkTransformMatrixKHR,
	pub transformT1: VkTransformMatrixKHR,
	pub instanceCustomIndex: u32,
	pub mask: u32,
	pub instanceShaderBindingTableRecordOffset: u32,
	pub flags: VkGeometryInstanceFlagsKHR,
	pub accelerationStructureReference: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSRTDataNV {
	pub sx: f32,
	pub a: f32,
	pub b: f32,
	pub pvx: f32,
	pub sy: f32,
	pub c: f32,
	pub pvy: f32,
	pub sz: f32,
	pub pvz: f32,
	pub qx: f32,
	pub qy: f32,
	pub qz: f32,
	pub qw: f32,
	pub tx: f32,
	pub ty: f32,
	pub tz: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureSRTMotionInstanceNV {
	pub transformT0: VkSRTDataNV,
	pub transformT1: VkSRTDataNV,
	pub instanceCustomIndex: u32,
	pub mask: u32,
	pub instanceShaderBindingTableRecordOffset: u32,
	pub flags: VkGeometryInstanceFlagsKHR,
	pub accelerationStructureReference: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkAccelerationStructureMotionInstanceDataNV {
	pub staticInstance: VkAccelerationStructureInstanceKHR,
	pub matrixMotionInstance: VkAccelerationStructureMatrixMotionInstanceNV,
	pub srtMotionInstance: VkAccelerationStructureSRTMotionInstanceNV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureMotionInstanceNV {
	pub type_: VkAccelerationStructureMotionInstanceTypeNV,
	pub flags: VkAccelerationStructureMotionInstanceFlagsNV,
	pub data: VkAccelerationStructureMotionInstanceDataNV,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub rayTracingMotionBlur: VkBool32,
	pub rayTracingMotionBlurPipelineTraceRaysIndirect: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub ycbcr2plane444Formats: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentDensityMapDeferred: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub subsampledLoads: VkBool32,
	pub subsampledCoarseReconstructionEarlyAccess: VkBool32,
	pub maxSubsampledArrayLayers: u32,
	pub maxDescriptorSetSubsampledSamplers: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyCommandTransformInfoQCOM {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub transform: VkSurfaceTransformFlagBitsKHR,
}

pub type VkPhysicalDeviceImageRobustnessFeaturesEXT = VkPhysicalDeviceImageRobustnessFeatures;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub formatA4R4G4B4: VkBool32,
	pub formatA4B4G4R4: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub rasterizationOrderColorAttachmentAccess: VkBool32,
	pub rasterizationOrderDepthAttachmentAccess: VkBool32,
	pub rasterizationOrderStencilAttachmentAccess: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub formatRgba10x6WithoutYCbCrSampler: VkBool32,
}

pub type PFN_vkAcquireWinrtDisplayNV = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

pub type PFN_vkGetWinrtDisplayNV = unsafe extern "C" fn(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkAcquireWinrtDisplayNV(physicalDevice: VkPhysicalDevice, display: VkDisplayKHR) -> VkResult;

	pub fn vkGetWinrtDisplayNV(physicalDevice: VkPhysicalDevice, deviceRelativeId: u32, pDisplay: *mut VkDisplayKHR) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub mutableDescriptorType: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMutableDescriptorTypeListVALVE {
	pub descriptorTypeCount: u32,
	pub pDescriptorTypes: *const VkDescriptorType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMutableDescriptorTypeCreateInfoVALVE {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub mutableDescriptorTypeListCount: u32,
	pub pMutableDescriptorTypeLists: *const VkMutableDescriptorTypeListVALVE,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub vertexInputDynamicState: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkVertexInputBindingDescription2EXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate,
	pub divisor: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkVertexInputAttributeDescription2EXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32,
}

pub type PFN_vkCmdSetVertexInputEXT = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	vertexBindingDescriptionCount: u32,
	pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
	vertexAttributeDescriptionCount: u32,
	pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetVertexInputEXT(
		commandBuffer: VkCommandBuffer,
		vertexBindingDescriptionCount: u32,
		pVertexBindingDescriptions: *const VkVertexInputBindingDescription2EXT,
		vertexAttributeDescriptionCount: u32,
		pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription2EXT,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub hasPrimary: VkBool32,
	pub hasRender: VkBool32,
	pub primaryMajor: i64,
	pub primaryMinor: i64,
	pub renderMajor: i64,
	pub renderMinor: i64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub depthClipControl: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub negativeOneToOne: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub primitiveTopologyListRestart: VkBool32,
	pub primitiveTopologyPatchListRestart: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub renderPass: VkRenderPass,
	pub subpass: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub subpassShading: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxSubpassShadingWorkgroupSizeAspectRatio: u32,
}

pub type PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "C" fn(device: VkDevice, renderpass: VkRenderPass, pMaxWorkgroupSize: *mut VkExtent2D) -> VkResult;

pub type PFN_vkCmdSubpassShadingHUAWEI = unsafe extern "C" fn(commandBuffer: VkCommandBuffer);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI(device: VkDevice, renderpass: VkRenderPass, pMaxWorkgroupSize: *mut VkExtent2D) -> VkResult;

	pub fn vkCmdSubpassShadingHUAWEI(commandBuffer: VkCommandBuffer);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub invocationMask: VkBool32,
}

pub type PFN_vkCmdBindInvocationMaskHUAWEI = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, imageView: VkImageView, imageLayout: VkImageLayout);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdBindInvocationMaskHUAWEI(commandBuffer: VkCommandBuffer, imageView: VkImageView, imageLayout: VkImageLayout);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRemoteAddressNV(*mut ());

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMemoryGetRemoteAddressInfoNV {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub memory: VkDeviceMemory,
	pub handleType: VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub externalMemoryRDMA: VkBool32,
}

pub type PFN_vkGetMemoryRemoteAddressNV = unsafe extern "C" fn(device: VkDevice, pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV, pAddress: *mut VkRemoteAddressNV) -> VkResult;

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetMemoryRemoteAddressNV(device: VkDevice, pMemoryGetRemoteAddressInfo: *const VkMemoryGetRemoteAddressInfoNV, pAddress: *mut VkRemoteAddressNV) -> VkResult;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub extendedDynamicState2: VkBool32,
	pub extendedDynamicState2LogicOp: VkBool32,
	pub extendedDynamicState2PatchControlPoints: VkBool32,
}

pub type PFN_vkCmdSetPatchControlPointsEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, patchControlPoints: u32);

pub type PFN_vkCmdSetRasterizerDiscardEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);

pub type PFN_vkCmdSetDepthBiasEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);

pub type PFN_vkCmdSetLogicOpEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp);

pub type PFN_vkCmdSetPrimitiveRestartEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetPatchControlPointsEXT(commandBuffer: VkCommandBuffer, patchControlPoints: u32);

	pub fn vkCmdSetRasterizerDiscardEnableEXT(commandBuffer: VkCommandBuffer, rasterizerDiscardEnable: VkBool32);

	pub fn vkCmdSetDepthBiasEnableEXT(commandBuffer: VkCommandBuffer, depthBiasEnable: VkBool32);

	pub fn vkCmdSetLogicOpEXT(commandBuffer: VkCommandBuffer, logicOp: VkLogicOp);

	pub fn vkCmdSetPrimitiveRestartEnableEXT(commandBuffer: VkCommandBuffer, primitiveRestartEnable: VkBool32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub colorWriteEnable: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPipelineColorWriteCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub attachmentCount: u32,
	pub pColorWriteEnables: *const VkBool32,
}

pub type PFN_vkCmdSetColorWriteEnableEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, attachmentCount: u32, pColorWriteEnables: *const VkBool32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdSetColorWriteEnableEXT(commandBuffer: VkCommandBuffer, attachmentCount: u32, pColorWriteEnables: *const VkBool32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub primitivesGeneratedQuery: VkBool32,
	pub primitivesGeneratedQueryWithRasterizerDiscard: VkBool32,
	pub primitivesGeneratedQueryWithNonZeroStreams: VkBool32,
}

pub type VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT = VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR;
pub type VkQueueFamilyGlobalPriorityPropertiesEXT = VkQueueFamilyGlobalPriorityPropertiesKHR;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub minLod: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkImageViewMinLodCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub minLod: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub multiDraw: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxMultiDrawCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMultiDrawInfoEXT {
	pub firstVertex: u32,
	pub vertexCount: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkMultiDrawIndexedInfoEXT {
	pub firstIndex: u32,
	pub indexCount: u32,
	pub vertexOffset: i32,
}

pub type PFN_vkCmdDrawMultiEXT = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, drawCount: u32, pVertexInfo: *const VkMultiDrawInfoEXT, instanceCount: u32, firstInstance: u32, stride: u32);

pub type PFN_vkCmdDrawMultiIndexedEXT =
	unsafe extern "C" fn(commandBuffer: VkCommandBuffer, drawCount: u32, pIndexInfo: *const VkMultiDrawIndexedInfoEXT, instanceCount: u32, firstInstance: u32, stride: u32, pVertexOffset: *const i32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdDrawMultiEXT(commandBuffer: VkCommandBuffer, drawCount: u32, pVertexInfo: *const VkMultiDrawInfoEXT, instanceCount: u32, firstInstance: u32, stride: u32);

	pub fn vkCmdDrawMultiIndexedEXT(
		commandBuffer: VkCommandBuffer,
		drawCount: u32,
		pIndexInfo: *const VkMultiDrawIndexedInfoEXT,
		instanceCount: u32,
		firstInstance: u32,
		stride: u32,
		pVertexOffset: *const i32,
	);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub image2DViewOf3D: VkBool32,
	pub sampler2DViewOf3D: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub borderColorSwizzle: VkBool32,
	pub borderColorSwizzleFromImage: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub components: VkComponentMapping,
	pub srgb: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub pageableDeviceLocalMemory: VkBool32,
}

pub type PFN_vkSetDeviceMemoryPriorityEXT = unsafe extern "C" fn(device: VkDevice, memory: VkDeviceMemory, priority: f32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkSetDeviceMemoryPriorityEXT(device: VkDevice, memory: VkDeviceMemory, priority: f32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub descriptorSetHostMapping: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetBindingReferenceVALVE {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub descriptorSetLayout: VkDescriptorSetLayout,
	pub binding: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub descriptorOffset: usize,
	pub descriptorSize: u32,
}

pub type PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE =
	unsafe extern "C" fn(device: VkDevice, pBindingReference: *const VkDescriptorSetBindingReferenceVALVE, pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE);

pub type PFN_vkGetDescriptorSetHostMappingVALVE = unsafe extern "C" fn(device: VkDevice, descriptorSet: VkDescriptorSet, ppData: *mut *mut ());

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkGetDescriptorSetLayoutHostMappingInfoVALVE(device: VkDevice, pBindingReference: *const VkDescriptorSetBindingReferenceVALVE, pHostMapping: *mut VkDescriptorSetLayoutHostMappingInfoVALVE);

	pub fn vkGetDescriptorSetHostMappingVALVE(device: VkDevice, descriptorSet: VkDescriptorSet, ppData: *mut *mut ());
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentDensityMapOffset: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub fragmentDensityOffsetGranularity: VkExtent2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub fragmentDensityOffsetCount: u32,
	pub pFragmentDensityOffsets: *const VkOffset2D,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub linearColorAttachment: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureKHR_T {
	_unused: [u8; 0],
}

pub type VkAccelerationStructureKHR = *mut VkAccelerationStructureKHR_T;

pub type VkBuildAccelerationStructureModeKHR = u32;
pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: VkBuildAccelerationStructureModeKHR = 0;
pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: VkBuildAccelerationStructureModeKHR = 1;
pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_MAX_ENUM_KHR: VkBuildAccelerationStructureModeKHR = 2147483647;

pub type VkAccelerationStructureBuildTypeKHR = u32;
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: VkAccelerationStructureBuildTypeKHR = 0;
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: VkAccelerationStructureBuildTypeKHR = 1;
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: VkAccelerationStructureBuildTypeKHR = 2;
pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_MAX_ENUM_KHR: VkAccelerationStructureBuildTypeKHR = 2147483647;

pub type VkAccelerationStructureCompatibilityKHR = u32;
pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: VkAccelerationStructureCompatibilityKHR = 0;
pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: VkAccelerationStructureCompatibilityKHR = 1;
pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_MAX_ENUM_KHR: VkAccelerationStructureCompatibilityKHR = 2147483647;

pub type VkAccelerationStructureCreateFlagsKHR = VkFlags;
pub type VkAccelerationStructureCreateFlagBitsKHR = VkFlags;
pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkAccelerationStructureCreateFlagBitsKHR = 1;
pub const VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV: VkAccelerationStructureCreateFlagBitsKHR = 4;
pub const VK_ACCELERATION_STRUCTURE_CREATE_FLAG_BITS_MAX_ENUM_KHR: VkAccelerationStructureCreateFlagBitsKHR = 2147483647;

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkDeviceOrHostAddressKHR {
	pub deviceAddress: VkDeviceAddress,
	pub hostAddress: *mut (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
	pub primitiveCount: u32,
	pub primitiveOffset: u32,
	pub firstVertex: u32,
	pub transformOffset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub vertexFormat: VkFormat,
	pub vertexData: VkDeviceOrHostAddressConstKHR,
	pub vertexStride: VkDeviceSize,
	pub maxVertex: u32,
	pub indexType: VkIndexType,
	pub indexData: VkDeviceOrHostAddressConstKHR,
	pub transformData: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub data: VkDeviceOrHostAddressConstKHR,
	pub stride: VkDeviceSize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub arrayOfPointers: VkBool32,
	pub data: VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VkAccelerationStructureGeometryDataKHR {
	pub triangles: VkAccelerationStructureGeometryTrianglesDataKHR,
	pub aabbs: VkAccelerationStructureGeometryAabbsDataKHR,
	pub instances: VkAccelerationStructureGeometryInstancesDataKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureGeometryKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub geometryType: VkGeometryTypeKHR,
	pub geometry: VkAccelerationStructureGeometryDataKHR,
	pub flags: VkGeometryFlagsKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkAccelerationStructureTypeKHR,
	pub flags: VkBuildAccelerationStructureFlagsKHR,
	pub mode: VkBuildAccelerationStructureModeKHR,
	pub srcAccelerationStructure: VkAccelerationStructureKHR,
	pub dstAccelerationStructure: VkAccelerationStructureKHR,
	pub geometryCount: u32,
	pub pGeometries: *const VkAccelerationStructureGeometryKHR,
	pub ppGeometries: *const *const VkAccelerationStructureGeometryKHR,
	pub scratchData: VkDeviceOrHostAddressKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub createFlags: VkAccelerationStructureCreateFlagsKHR,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub type_: VkAccelerationStructureTypeKHR,
	pub deviceAddress: VkDeviceAddress,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub accelerationStructureCount: u32,
	pub pAccelerationStructures: *const VkAccelerationStructureKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub accelerationStructure: VkBool32,
	pub accelerationStructureCaptureReplay: VkBool32,
	pub accelerationStructureIndirectBuild: VkBool32,
	pub accelerationStructureHostCommands: VkBool32,
	pub descriptorBindingAccelerationStructureUpdateAfterBind: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub maxGeometryCount: u64,
	pub maxInstanceCount: u64,
	pub maxPrimitiveCount: u64,
	pub maxPerStageDescriptorAccelerationStructures: u32,
	pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures: u32,
	pub maxDescriptorSetAccelerationStructures: u32,
	pub maxDescriptorSetUpdateAfterBindAccelerationStructures: u32,
	pub minAccelerationStructureScratchOffsetAlignment: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub accelerationStructure: VkAccelerationStructureKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureVersionInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub pVersionData: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub src: VkAccelerationStructureKHR,
	pub dst: VkDeviceOrHostAddressKHR,
	pub mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub src: VkDeviceOrHostAddressConstKHR,
	pub dst: VkAccelerationStructureKHR,
	pub mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkCopyAccelerationStructureInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub src: VkAccelerationStructureKHR,
	pub dst: VkAccelerationStructureKHR,
	pub mode: VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub accelerationStructureSize: VkDeviceSize,
	pub updateScratchSize: VkDeviceSize,
	pub buildScratchSize: VkDeviceSize,
}

pub type PFN_vkCreateAccelerationStructureKHR = unsafe extern "C" fn(
	device: VkDevice,
	pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pAccelerationStructure: *mut VkAccelerationStructureKHR,
) -> VkResult;

pub type PFN_vkDestroyAccelerationStructureKHR = unsafe extern "C" fn(device: VkDevice, accelerationStructure: VkAccelerationStructureKHR, pAllocator: *const VkAllocationCallbacks);

pub type PFN_vkCmdBuildAccelerationStructuresKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
);

pub type PFN_vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	pIndirectDeviceAddresses: *const VkDeviceAddress,
	pIndirectStrides: *const u32,
	ppMaxPrimitiveCounts: *const *const u32,
);

pub type PFN_vkBuildAccelerationStructuresKHR = unsafe extern "C" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	infoCount: u32,
	pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
	ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
) -> VkResult;

pub type PFN_vkCopyAccelerationStructureKHR = unsafe extern "C" fn(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyAccelerationStructureInfoKHR) -> VkResult;

pub type PFN_vkCopyAccelerationStructureToMemoryKHR =
	unsafe extern "C" fn(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR) -> VkResult;

pub type PFN_vkCopyMemoryToAccelerationStructureKHR =
	unsafe extern "C" fn(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR) -> VkResult;

pub type PFN_vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
	device: VkDevice,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureKHR,
	queryType: VkQueryType,
	dataSize: usize,
	pData: *mut (),
	stride: usize,
) -> VkResult;

pub type PFN_vkCmdCopyAccelerationStructureKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureInfoKHR);

pub type PFN_vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR);

pub type PFN_vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR);

pub type PFN_vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "C" fn(device: VkDevice, pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR) -> VkDeviceAddress;

pub type PFN_vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	accelerationStructureCount: u32,
	pAccelerationStructures: *const VkAccelerationStructureKHR,
	queryType: VkQueryType,
	queryPool: VkQueryPool,
	firstQuery: u32,
);

pub type PFN_vkGetDeviceAccelerationStructureCompatibilityKHR =
	unsafe extern "C" fn(device: VkDevice, pVersionInfo: *const VkAccelerationStructureVersionInfoKHR, pCompatibility: *mut VkAccelerationStructureCompatibilityKHR);

pub type PFN_vkGetAccelerationStructureBuildSizesKHR = unsafe extern "C" fn(
	device: VkDevice,
	buildType: VkAccelerationStructureBuildTypeKHR,
	pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
	pMaxPrimitiveCounts: *const u32,
	pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCreateAccelerationStructureKHR(
		device: VkDevice,
		pCreateInfo: *const VkAccelerationStructureCreateInfoKHR,
		pAllocator: *const VkAllocationCallbacks,
		pAccelerationStructure: *mut VkAccelerationStructureKHR,
	) -> VkResult;

	pub fn vkDestroyAccelerationStructureKHR(device: VkDevice, accelerationStructure: VkAccelerationStructureKHR, pAllocator: *const VkAllocationCallbacks);

	pub fn vkCmdBuildAccelerationStructuresKHR(
		commandBuffer: VkCommandBuffer,
		infoCount: u32,
		pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
		ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
	);

	pub fn vkCmdBuildAccelerationStructuresIndirectKHR(
		commandBuffer: VkCommandBuffer,
		infoCount: u32,
		pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
		pIndirectDeviceAddresses: *const VkDeviceAddress,
		pIndirectStrides: *const u32,
		ppMaxPrimitiveCounts: *const *const u32,
	);

	pub fn vkBuildAccelerationStructuresKHR(
		device: VkDevice,
		deferredOperation: VkDeferredOperationKHR,
		infoCount: u32,
		pInfos: *const VkAccelerationStructureBuildGeometryInfoKHR,
		ppBuildRangeInfos: *const *const VkAccelerationStructureBuildRangeInfoKHR,
	) -> VkResult;

	pub fn vkCopyAccelerationStructureKHR(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyAccelerationStructureInfoKHR) -> VkResult;

	pub fn vkCopyAccelerationStructureToMemoryKHR(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR) -> VkResult;

	pub fn vkCopyMemoryToAccelerationStructureKHR(device: VkDevice, deferredOperation: VkDeferredOperationKHR, pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR) -> VkResult;

	pub fn vkWriteAccelerationStructuresPropertiesKHR(
		device: VkDevice,
		accelerationStructureCount: u32,
		pAccelerationStructures: *const VkAccelerationStructureKHR,
		queryType: VkQueryType,
		dataSize: usize,
		pData: *mut (),
		stride: usize,
	) -> VkResult;

	pub fn vkCmdCopyAccelerationStructureKHR(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureInfoKHR);

	pub fn vkCmdCopyAccelerationStructureToMemoryKHR(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyAccelerationStructureToMemoryInfoKHR);

	pub fn vkCmdCopyMemoryToAccelerationStructureKHR(commandBuffer: VkCommandBuffer, pInfo: *const VkCopyMemoryToAccelerationStructureInfoKHR);

	pub fn vkGetAccelerationStructureDeviceAddressKHR(device: VkDevice, pInfo: *const VkAccelerationStructureDeviceAddressInfoKHR) -> VkDeviceAddress;

	pub fn vkCmdWriteAccelerationStructuresPropertiesKHR(
		commandBuffer: VkCommandBuffer,
		accelerationStructureCount: u32,
		pAccelerationStructures: *const VkAccelerationStructureKHR,
		queryType: VkQueryType,
		queryPool: VkQueryPool,
		firstQuery: u32,
	);

	pub fn vkGetDeviceAccelerationStructureCompatibilityKHR(device: VkDevice, pVersionInfo: *const VkAccelerationStructureVersionInfoKHR, pCompatibility: *mut VkAccelerationStructureCompatibilityKHR);

	pub fn vkGetAccelerationStructureBuildSizesKHR(
		device: VkDevice,
		buildType: VkAccelerationStructureBuildTypeKHR,
		pBuildInfo: *const VkAccelerationStructureBuildGeometryInfoKHR,
		pMaxPrimitiveCounts: *const u32,
		pSizeInfo: *mut VkAccelerationStructureBuildSizesInfoKHR,
	);
}

pub type VkShaderGroupShaderKHR = u32;
pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR: VkShaderGroupShaderKHR = 0;
pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: VkShaderGroupShaderKHR = 1;
pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: VkShaderGroupShaderKHR = 2;
pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: VkShaderGroupShaderKHR = 3;
pub const VK_SHADER_GROUP_SHADER_MAX_ENUM_KHR: VkShaderGroupShaderKHR = 2147483647;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub type_: VkRayTracingShaderGroupTypeKHR,
	pub generalShader: u32,
	pub closestHitShader: u32,
	pub anyHitShader: u32,
	pub intersectionShader: u32,
	pub pShaderGroupCaptureReplayHandle: *const (),
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub maxPipelineRayPayloadSize: u32,
	pub maxPipelineRayHitAttributeSize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkRayTracingPipelineCreateInfoKHR {
	pub sType: VkStructureType,
	pub pNext: *const (),
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo,
	pub groupCount: u32,
	pub pGroups: *const VkRayTracingShaderGroupCreateInfoKHR,
	pub maxPipelineRayRecursionDepth: u32,
	pub pLibraryInfo: *const VkPipelineLibraryCreateInfoKHR,
	pub pLibraryInterface: *const VkRayTracingPipelineInterfaceCreateInfoKHR,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub rayTracingPipeline: VkBool32,
	pub rayTracingPipelineShaderGroupHandleCaptureReplay: VkBool32,
	pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed: VkBool32,
	pub rayTracingPipelineTraceRaysIndirect: VkBool32,
	pub rayTraversalPrimitiveCulling: VkBool32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub shaderGroupHandleSize: u32,
	pub maxRayRecursionDepth: u32,
	pub maxShaderGroupStride: u32,
	pub shaderGroupBaseAlignment: u32,
	pub shaderGroupHandleCaptureReplaySize: u32,
	pub maxRayDispatchInvocationCount: u32,
	pub shaderGroupHandleAlignment: u32,
	pub maxRayHitAttributeSize: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkStridedDeviceAddressRegionKHR {
	pub deviceAddress: VkDeviceAddress,
	pub stride: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkTraceRaysIndirectCommandKHR {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

pub type PFN_vkCmdTraceRaysKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	width: u32,
	height: u32,
	depth: u32,
);

pub type PFN_vkCreateRayTracingPipelinesKHR = unsafe extern "C" fn(
	device: VkDevice,
	deferredOperation: VkDeferredOperationKHR,
	pipelineCache: VkPipelineCache,
	createInfoCount: u32,
	pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
	pAllocator: *const VkAllocationCallbacks,
	pPipelines: *mut VkPipeline,
) -> VkResult;

pub type PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR =
	unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

pub type PFN_vkCmdTraceRaysIndirectKHR = unsafe extern "C" fn(
	commandBuffer: VkCommandBuffer,
	pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
	indirectDeviceAddress: VkDeviceAddress,
);

pub type PFN_vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "C" fn(device: VkDevice, pipeline: VkPipeline, group: u32, groupShader: VkShaderGroupShaderKHR) -> VkDeviceSize;

pub type PFN_vkCmdSetRayTracingPipelineStackSizeKHR = unsafe extern "C" fn(commandBuffer: VkCommandBuffer, pipelineStackSize: u32);

#[link(name = "vulkan")]
unsafe extern "C" {
	pub fn vkCmdTraceRaysKHR(
		commandBuffer: VkCommandBuffer,
		pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		width: u32,
		height: u32,
		depth: u32,
	);

	pub fn vkCreateRayTracingPipelinesKHR(
		device: VkDevice,
		deferredOperation: VkDeferredOperationKHR,
		pipelineCache: VkPipelineCache,
		createInfoCount: u32,
		pCreateInfos: *const VkRayTracingPipelineCreateInfoKHR,
		pAllocator: *const VkAllocationCallbacks,
		pPipelines: *mut VkPipeline,
	) -> VkResult;

	pub fn vkGetRayTracingCaptureReplayShaderGroupHandlesKHR(device: VkDevice, pipeline: VkPipeline, firstGroup: u32, groupCount: u32, dataSize: usize, pData: *mut ()) -> VkResult;

	pub fn vkCmdTraceRaysIndirectKHR(
		commandBuffer: VkCommandBuffer,
		pRaygenShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pMissShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pHitShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		pCallableShaderBindingTable: *const VkStridedDeviceAddressRegionKHR,
		indirectDeviceAddress: VkDeviceAddress,
	);

	pub fn vkGetRayTracingShaderGroupStackSizeKHR(device: VkDevice, pipeline: VkPipeline, group: u32, groupShader: VkShaderGroupShaderKHR) -> VkDeviceSize;

	pub fn vkCmdSetRayTracingPipelineStackSizeKHR(commandBuffer: VkCommandBuffer, pipelineStackSize: u32);
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
	pub sType: VkStructureType,
	pub pNext: *mut (),
	pub rayQuery: VkBool32,
}
