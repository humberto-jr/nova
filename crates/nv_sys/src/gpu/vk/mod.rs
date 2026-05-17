use ::core::{
	any, //
	iter::Iterator,
	slice,
};

use crate::spec;

pub use crate::ffi::khronos::vulkan::{
	core, //
	wayland,
	xcb,
};

mod dispatch;
mod pod;
mod shader;

//
// API dynamic loading:
//

pub use dispatch::VULKAN_LIBRARY_FILENAME;

pub use dispatch::MOLTENVK_LIBRARY_FILENAME;

pub use dispatch::InstanceExtension;

pub use dispatch::DeviceExtension;

pub use dispatch::AllocationCallbacks;

pub use dispatch::Loader;

pub use dispatch::StaticLoader;

pub use dispatch::InstanceFnTable;

pub use dispatch::SurfaceFnTable;

pub use dispatch::XcbSurfaceFnTable;

pub use dispatch::WaylandSurfaceFnTable;

pub use dispatch::HeadlessSurfaceFnTable;

pub use dispatch::DebugUtilsFnTable;

pub use dispatch::DisplayFnTable;

pub use dispatch::DeviceFnTable;

pub use dispatch::SwapchainFnTable;

pub use dispatch::PushDescriptorFnTable;

pub use dispatch::RayTracingPipelineFnTable;

pub use dispatch::AccelerationStructureFnTable;

#[inline]
pub fn loader() -> spec::Result<Loader> {
	Loader::new(VULKAN_LIBRARY_FILENAME)
}

//
// Base types:
//

pub type Result = core::VkResult;

pub type Bool32 = core::VkBool32;

pub type Flags = core::VkFlags;

pub type Flags64 = core::VkFlags64;

pub type DeviceAddress = core::VkDeviceAddress;

pub type DeviceSize = core::VkDeviceSize;

//
// POD wrappers and handles:
//

pub use pod::*;

pub type Buffer = core::VkBuffer;

pub type Image = core::VkImage;

pub type Instance = core::VkInstance;

pub type PhysicalDevice = core::VkPhysicalDevice;

pub type Device = core::VkDevice;

pub type Queue = core::VkQueue;

pub type Semaphore = core::VkSemaphore;

pub type CommandBuffer = core::VkCommandBuffer;

pub type Fence = core::VkFence;

pub type DeviceMemory = core::VkDeviceMemory;

pub type Event = core::VkEvent;

pub type QueryPool = core::VkQueryPool;

pub type BufferView = core::VkBufferView;

pub type ImageView = core::VkImageView;

pub type ShaderModule = core::VkShaderModule;

pub type PipelineCache = core::VkPipelineCache;

pub type PipelineLayout = core::VkPipelineLayout;

pub type Pipeline = core::VkPipeline;

pub type RenderPass = core::VkRenderPass;

pub type DescriptorSetLayout = core::VkDescriptorSetLayout;

pub type Sampler = core::VkSampler;

pub type DescriptorSet = core::VkDescriptorSet;

pub type DescriptorPool = core::VkDescriptorPool;

pub type Framebuffer = core::VkFramebuffer;

pub type CommandPool = core::VkCommandPool;

pub type SamplerYcbcrConversion = core::VkSamplerYcbcrConversion;

pub type DescriptorUpdateTemplate = core::VkDescriptorUpdateTemplate;

pub type PrivateDataSlot = core::VkPrivateDataSlot;

pub type SurfaceKHR = core::VkSurfaceKHR;

pub type SwapchainKHR = core::VkSwapchainKHR;

pub type DisplayKHR = core::VkDisplayKHR;

pub type DisplayModeKHR = core::VkDisplayModeKHR;

pub type DeferredOperationKHR = core::VkDeferredOperationKHR;

pub type DebugReportCallbackEXT = core::VkDebugReportCallbackEXT;

pub type CuModuleNVX = core::VkCuModuleNVX;

pub type CuFunctionNVX = core::VkCuFunctionNVX;

pub type DebugUtilsMessengerEXT = core::VkDebugUtilsMessengerEXT;

pub type ValidationCacheEXT = core::VkValidationCacheEXT;

pub type AccelerationStructureNV = core::VkAccelerationStructureNV;

pub type PerformanceConfigurationINTEL = core::VkPerformanceConfigurationINTEL;

pub type IndirectCommandsLayoutNV = core::VkIndirectCommandsLayoutNV;

pub type AccelerationStructureKHR = core::VkAccelerationStructureKHR;

//
// Enums, masks and bitflags:
//

pub type SampleMask = core::VkSampleMask;

pub type StructureType = core::VkStructureType;

pub type ImageLayout = core::VkImageLayout;

pub type ObjectType = core::VkObjectType;

pub type PipelineCacheHeaderVersion = core::VkPipelineCacheHeaderVersion;

pub type VendorId = core::VkVendorId;

pub type SystemAllocationScope = core::VkSystemAllocationScope;

pub type InternalAllocationType = core::VkInternalAllocationType;

pub type Format = core::VkFormat;

pub type ImageTiling = core::VkImageTiling;

pub type ImageType = core::VkImageType;

pub type PhysicalDeviceType = core::VkPhysicalDeviceType;

pub type QueryType = core::VkQueryType;

pub type SharingMode = core::VkSharingMode;

pub type ComponentSwizzle = core::VkComponentSwizzle;

pub type ImageViewType = core::VkImageViewType;

pub type BlendFactor = core::VkBlendFactor;

pub type BlendOp = core::VkBlendOp;

pub type CompareOp = core::VkCompareOp;

pub type DynamicState = core::VkDynamicState;

pub type FrontFace = core::VkFrontFace;

pub type VertexInputRate = core::VkVertexInputRate;

pub type PrimitiveTopology = core::VkPrimitiveTopology;

pub type PolygonMode = core::VkPolygonMode;

pub type StencilOp = core::VkStencilOp;

pub type LogicOp = core::VkLogicOp;

pub type BorderColor = core::VkBorderColor;

pub type Filter = core::VkFilter;

pub type SamplerAddressMode = core::VkSamplerAddressMode;

pub type SamplerMipmapMode = core::VkSamplerMipmapMode;

pub type DescriptorType = core::VkDescriptorType;

pub type AttachmentLoadOp = core::VkAttachmentLoadOp;

pub type AttachmentStoreOp = core::VkAttachmentStoreOp;

pub type PipelineBindPoint = core::VkPipelineBindPoint;

pub type CommandBufferLevel = core::VkCommandBufferLevel;

pub type IndexType = core::VkIndexType;

pub type SubpassContents = core::VkSubpassContents;

pub type PointClippingBehavior = core::VkPointClippingBehavior;

pub type TessellationDomainOrigin = core::VkTessellationDomainOrigin;

pub type SamplerYcbcrModelConversion = core::VkSamplerYcbcrModelConversion;

pub type SamplerYcbcrRange = core::VkSamplerYcbcrRange;

pub type ChromaLocation = core::VkChromaLocation;

pub type DescriptorUpdateTemplateType = core::VkDescriptorUpdateTemplateType;

pub type DriverId = core::VkDriverId;

pub type ShaderFloatControlsIndependence = core::VkShaderFloatControlsIndependence;

pub type SamplerReductionMode = core::VkSamplerReductionMode;

pub type SemaphoreType = core::VkSemaphoreType;

pub type PresentModeKHR = core::VkPresentModeKHR;

pub type ColorSpaceKHR = core::VkColorSpaceKHR;

pub type DisplayPlaneAlphaFlagBitsKHR = core::VkDisplayPlaneAlphaFlagBitsKHR;

pub type PerformanceCounterUnitKHR = core::VkPerformanceCounterUnitKHR;

pub type PerformanceCounterScopeKHR = core::VkPerformanceCounterScopeKHR;

pub type PerformanceCounterStorageKHR = core::VkPerformanceCounterStorageKHR;

pub type PerformanceCounterDescriptionFlagBitsKHR = core::VkPerformanceCounterDescriptionFlagBitsKHR;

pub type AcquireProfilingLockFlagBitsKHR = core::VkAcquireProfilingLockFlagBitsKHR;

pub type QueueGlobalPriorityKHR = core::VkQueueGlobalPriorityKHR;

pub type FragmentShadingRateCombinerOpKHR = core::VkFragmentShadingRateCombinerOpKHR;

pub type PipelineExecutableStatisticFormatKHR = core::VkPipelineExecutableStatisticFormatKHR;

pub type DebugReportObjectTypeEXT = core::VkDebugReportObjectTypeEXT;

pub type DebugReportFlagBitsEXT = core::VkDebugReportFlagBitsEXT;

pub type RasterizationOrderAMD = core::VkRasterizationOrderAMD;

pub type ShaderInfoTypeAMD = core::VkShaderInfoTypeAMD;

pub type ExternalMemoryHandleTypeFlagBitsNV = core::VkExternalMemoryHandleTypeFlagBitsNV;

pub type ExternalMemoryFeatureFlagBitsNV = core::VkExternalMemoryFeatureFlagBitsNV;

pub type ValidationCheckEXT = core::VkValidationCheckEXT;

pub type ConditionalRenderingFlagBitsEXT = core::VkConditionalRenderingFlagBitsEXT;

pub type SurfaceCounterFlagBitsEXT = core::VkSurfaceCounterFlagBitsEXT;

pub type DisplayPowerStateEXT = core::VkDisplayPowerStateEXT;

pub type DeviceEventTypeEXT = core::VkDeviceEventTypeEXT;

pub type DisplayEventTypeEXT = core::VkDisplayEventTypeEXT;

pub type ViewportCoordinateSwizzleNV = core::VkViewportCoordinateSwizzleNV;

pub type DiscardRectangleModeEXT = core::VkDiscardRectangleModeEXT;

pub type ConservativeRasterizationModeEXT = core::VkConservativeRasterizationModeEXT;

pub type DebugUtilsMessageSeverityFlagBitsEXT = core::VkDebugUtilsMessageSeverityFlagBitsEXT;

pub type DebugUtilsMessageTypeFlagBitsEXT = core::VkDebugUtilsMessageTypeFlagBitsEXT;

pub type BlendOverlapEXT = core::VkBlendOverlapEXT;

pub type CoverageModulationModeNV = core::VkCoverageModulationModeNV;

pub type ValidationCacheHeaderVersionEXT = core::VkValidationCacheHeaderVersionEXT;

pub type ShadingRatePaletteEntryNV = core::VkShadingRatePaletteEntryNV;

pub type CoarseSampleOrderTypeNV = core::VkCoarseSampleOrderTypeNV;

pub type RayTracingShaderGroupTypeKHR = core::VkRayTracingShaderGroupTypeKHR;

pub type GeometryTypeKHR = core::VkGeometryTypeKHR;

pub type AccelerationStructureTypeKHR = core::VkAccelerationStructureTypeKHR;

pub type CopyAccelerationStructureModeKHR = core::VkCopyAccelerationStructureModeKHR;

pub type AccelerationStructureMemoryRequirementsTypeNV = core::VkAccelerationStructureMemoryRequirementsTypeNV;

pub type PipelineCompilerControlFlagBitsAMD = core::VkPipelineCompilerControlFlagBitsAMD;

pub type TimeDomainEXT = core::VkTimeDomainEXT;

pub type MemoryOverallocationBehaviorAMD = core::VkMemoryOverallocationBehaviorAMD;

pub type PerformanceConfigurationTypeINTEL = core::VkPerformanceConfigurationTypeINTEL;

pub type QueryPoolSamplingModeINTEL = core::VkQueryPoolSamplingModeINTEL;

pub type PerformanceOverrideTypeINTEL = core::VkPerformanceOverrideTypeINTEL;

pub type PerformanceParameterTypeINTEL = core::VkPerformanceParameterTypeINTEL;

pub type PerformanceValueTypeINTEL = core::VkPerformanceValueTypeINTEL;

pub type ValidationFeatureEnableEXT = core::VkValidationFeatureEnableEXT;

pub type ValidationFeatureDisableEXT = core::VkValidationFeatureDisableEXT;

pub type ComponentTypeNV = core::VkComponentTypeNV;

pub type ScopeNV = core::VkScopeNV;

pub type CoverageReductionModeNV = core::VkCoverageReductionModeNV;

pub type ProvokingVertexModeEXT = core::VkProvokingVertexModeEXT;

pub type LineRasterizationModeEXT = core::VkLineRasterizationModeEXT;

pub type IndirectCommandsTokenTypeNV = core::VkIndirectCommandsTokenTypeNV;

pub type DeviceMemoryReportEventTypeEXT = core::VkDeviceMemoryReportEventTypeEXT;

pub type FragmentShadingRateTypeNV = core::VkFragmentShadingRateTypeNV;

pub type FragmentShadingRateNV = core::VkFragmentShadingRateNV;

pub type AccelerationStructureMotionInstanceTypeNV = core::VkAccelerationStructureMotionInstanceTypeNV;

pub type BuildAccelerationStructureModeKHR = core::VkBuildAccelerationStructureModeKHR;

pub type AccelerationStructureBuildTypeKHR = core::VkAccelerationStructureBuildTypeKHR;

pub type AccelerationStructureCompatibilityKHR = core::VkAccelerationStructureCompatibilityKHR;

pub type ShaderGroupShaderKHR = core::VkShaderGroupShaderKHR;

pub type AccessFlags = core::VkAccessFlags;

pub type ImageAspectFlags = core::VkImageAspectFlags;

pub type FormatFeatureFlags = core::VkFormatFeatureFlags;

pub type ImageCreateFlags = core::VkImageCreateFlags;

pub type SampleCountFlags = core::VkSampleCountFlags;

pub type ImageUsageFlags = core::VkImageUsageFlags;

pub type InstanceCreateFlags = core::VkInstanceCreateFlags;

pub type MemoryHeapFlags = core::VkMemoryHeapFlags;

pub type MemoryPropertyFlags = core::VkMemoryPropertyFlags;

pub type QueueFlags = core::VkQueueFlags;

pub type DeviceCreateFlags = core::VkDeviceCreateFlags;

pub type DeviceQueueCreateFlags = core::VkDeviceQueueCreateFlags;

pub type PipelineStageFlags = core::VkPipelineStageFlags;

pub type MemoryMapFlags = core::VkMemoryMapFlags;

pub type SparseMemoryBindFlags = core::VkSparseMemoryBindFlags;

pub type SparseImageFormatFlags = core::VkSparseImageFormatFlags;

pub type FenceCreateFlags = core::VkFenceCreateFlags;

pub type SemaphoreCreateFlags = core::VkSemaphoreCreateFlags;

pub type EventCreateFlags = core::VkEventCreateFlags;

pub type QueryPipelineStatisticFlags = core::VkQueryPipelineStatisticFlags;

pub type QueryPoolCreateFlags = core::VkQueryPoolCreateFlags;

pub type QueryResultFlags = core::VkQueryResultFlags;

pub type BufferCreateFlags = core::VkBufferCreateFlags;

pub type BufferUsageFlags = core::VkBufferUsageFlags;

pub type BufferViewCreateFlags = core::VkBufferViewCreateFlags;

pub type ImageViewCreateFlags = core::VkImageViewCreateFlags;

pub type ShaderModuleCreateFlags = core::VkShaderModuleCreateFlags;

pub type PipelineCacheCreateFlags = core::VkPipelineCacheCreateFlags;

pub type ColorComponentFlags = core::VkColorComponentFlags;

pub type PipelineCreateFlags = core::VkPipelineCreateFlags;

pub type PipelineShaderStageCreateFlags = core::VkPipelineShaderStageCreateFlags;

pub type ShaderStageFlags = core::VkShaderStageFlags;

pub type CullModeFlags = core::VkCullModeFlags;

pub type PipelineVertexInputStateCreateFlags = core::VkPipelineVertexInputStateCreateFlags;

pub type PipelineInputAssemblyStateCreateFlags = core::VkPipelineInputAssemblyStateCreateFlags;

pub type PipelineTessellationStateCreateFlags = core::VkPipelineTessellationStateCreateFlags;

pub type PipelineViewportStateCreateFlags = core::VkPipelineViewportStateCreateFlags;

pub type PipelineRasterizationStateCreateFlags = core::VkPipelineRasterizationStateCreateFlags;

pub type PipelineMultisampleStateCreateFlags = core::VkPipelineMultisampleStateCreateFlags;

pub type PipelineDepthStencilStateCreateFlags = core::VkPipelineDepthStencilStateCreateFlags;

pub type PipelineColorBlendStateCreateFlags = core::VkPipelineColorBlendStateCreateFlags;

pub type PipelineDynamicStateCreateFlags = core::VkPipelineDynamicStateCreateFlags;

pub type PipelineLayoutCreateFlags = core::VkPipelineLayoutCreateFlags;

pub type SamplerCreateFlags = core::VkSamplerCreateFlags;

pub type DescriptorPoolCreateFlags = core::VkDescriptorPoolCreateFlags;

pub type DescriptorPoolResetFlags = core::VkDescriptorPoolResetFlags;

pub type DescriptorSetLayoutCreateFlags = core::VkDescriptorSetLayoutCreateFlags;

pub type AttachmentDescriptionFlags = core::VkAttachmentDescriptionFlags;

pub type DependencyFlags = core::VkDependencyFlags;

pub type FramebufferCreateFlags = core::VkFramebufferCreateFlags;

pub type RenderPassCreateFlags = core::VkRenderPassCreateFlags;

pub type SubpassDescriptionFlags = core::VkSubpassDescriptionFlags;

pub type CommandPoolCreateFlags = core::VkCommandPoolCreateFlags;

pub type CommandPoolResetFlags = core::VkCommandPoolResetFlags;

pub type CommandBufferUsageFlags = core::VkCommandBufferUsageFlags;

pub type QueryControlFlags = core::VkQueryControlFlags;

pub type CommandBufferResetFlags = core::VkCommandBufferResetFlags;

pub type StencilFaceFlags = core::VkStencilFaceFlags;

pub type SubgroupFeatureFlags = core::VkSubgroupFeatureFlags;

pub type PeerMemoryFeatureFlags = core::VkPeerMemoryFeatureFlags;

pub type MemoryAllocateFlags = core::VkMemoryAllocateFlags;

pub type CommandPoolTrimFlags = core::VkCommandPoolTrimFlags;

pub type DescriptorUpdateTemplateCreateFlags = core::VkDescriptorUpdateTemplateCreateFlags;

pub type ExternalMemoryHandleTypeFlags = core::VkExternalMemoryHandleTypeFlags;

pub type ExternalMemoryFeatureFlags = core::VkExternalMemoryFeatureFlags;

pub type ExternalFenceHandleTypeFlags = core::VkExternalFenceHandleTypeFlags;

pub type ExternalFenceFeatureFlags = core::VkExternalFenceFeatureFlags;

pub type FenceImportFlags = core::VkFenceImportFlags;

pub type SemaphoreImportFlags = core::VkSemaphoreImportFlags;

pub type ExternalSemaphoreHandleTypeFlags = core::VkExternalSemaphoreHandleTypeFlags;

pub type ExternalSemaphoreFeatureFlags = core::VkExternalSemaphoreFeatureFlags;

pub type ResolveModeFlags = core::VkResolveModeFlags;

pub type DescriptorBindingFlags = core::VkDescriptorBindingFlags;

pub type SemaphoreWaitFlags = core::VkSemaphoreWaitFlags;

pub type PipelineCreationFeedbackFlags = core::VkPipelineCreationFeedbackFlags;

pub type ToolPurposeFlags = core::VkToolPurposeFlags;

pub type PrivateDataSlotCreateFlags = core::VkPrivateDataSlotCreateFlags;

pub type SubmitFlags = core::VkSubmitFlags;

pub type RenderingFlags = core::VkRenderingFlags;

pub type AccessFlagBits = core::VkAccessFlagBits;

pub type ImageAspectFlagBits = core::VkImageAspectFlagBits;

pub type FormatFeatureFlagBits = core::VkFormatFeatureFlagBits;

pub type ImageCreateFlagBits = core::VkImageCreateFlagBits;

pub type SampleCountFlagBits = core::VkSampleCountFlagBits;

pub type ImageUsageFlagBits = core::VkImageUsageFlagBits;

pub type InstanceCreateFlagBits = core::VkInstanceCreateFlagBits;

pub type MemoryHeapFlagBits = core::VkMemoryHeapFlagBits;

pub type MemoryPropertyFlagBits = core::VkMemoryPropertyFlagBits;

pub type QueueFlagBits = core::VkQueueFlagBits;

pub type DeviceQueueCreateFlagBits = core::VkDeviceQueueCreateFlagBits;

pub type PipelineStageFlagBits = core::VkPipelineStageFlagBits;

pub type SparseMemoryBindFlagBits = core::VkSparseMemoryBindFlagBits;

pub type SparseImageFormatFlagBits = core::VkSparseImageFormatFlagBits;

pub type FenceCreateFlagBits = core::VkFenceCreateFlagBits;

pub type EventCreateFlagBits = core::VkEventCreateFlagBits;

pub type QueryPipelineStatisticFlagBits = core::VkQueryPipelineStatisticFlagBits;

pub type QueryResultFlagBits = core::VkQueryResultFlagBits;

pub type BufferCreateFlagBits = core::VkBufferCreateFlagBits;

pub type BufferUsageFlagBits = core::VkBufferUsageFlagBits;

pub type ImageViewCreateFlagBits = core::VkImageViewCreateFlagBits;

pub type PipelineCacheCreateFlagBits = core::VkPipelineCacheCreateFlagBits;

pub type ColorComponentFlagBits = core::VkColorComponentFlagBits;

pub type PipelineCreateFlagBits = core::VkPipelineCreateFlagBits;

pub type PipelineShaderStageCreateFlagBits = core::VkPipelineShaderStageCreateFlagBits;

pub type ShaderStageFlagBits = core::VkShaderStageFlagBits;

pub type CullModeFlagBits = core::VkCullModeFlagBits;

pub type PipelineDepthStencilStateCreateFlagBits = core::VkPipelineDepthStencilStateCreateFlagBits;

pub type PipelineColorBlendStateCreateFlagBits = core::VkPipelineColorBlendStateCreateFlagBits;

pub type PipelineLayoutCreateFlagBits = core::VkPipelineLayoutCreateFlagBits;

pub type SamplerCreateFlagBits = core::VkSamplerCreateFlagBits;

pub type DescriptorPoolCreateFlagBits = core::VkDescriptorPoolCreateFlagBits;

pub type DescriptorSetLayoutCreateFlagBits = core::VkDescriptorSetLayoutCreateFlagBits;

pub type AttachmentDescriptionFlagBits = core::VkAttachmentDescriptionFlagBits;

pub type DependencyFlagBits = core::VkDependencyFlagBits;

pub type FramebufferCreateFlagBits = core::VkFramebufferCreateFlagBits;

pub type RenderPassCreateFlagBits = core::VkRenderPassCreateFlagBits;

pub type SubpassDescriptionFlagBits = core::VkSubpassDescriptionFlagBits;

pub type CommandPoolCreateFlagBits = core::VkCommandPoolCreateFlagBits;

pub type CommandPoolResetFlagBits = core::VkCommandPoolResetFlagBits;

pub type CommandBufferUsageFlagBits = core::VkCommandBufferUsageFlagBits;

pub type QueryControlFlagBits = core::VkQueryControlFlagBits;

pub type CommandBufferResetFlagBits = core::VkCommandBufferResetFlagBits;

pub type StencilFaceFlagBits = core::VkStencilFaceFlagBits;

pub type SubgroupFeatureFlagBits = core::VkSubgroupFeatureFlagBits;

pub type PeerMemoryFeatureFlagBits = core::VkPeerMemoryFeatureFlagBits;

pub type MemoryAllocateFlagBits = core::VkMemoryAllocateFlagBits;

pub type ExternalMemoryHandleTypeFlagBits = core::VkExternalMemoryHandleTypeFlagBits;

pub type ExternalMemoryFeatureFlagBits = core::VkExternalMemoryFeatureFlagBits;

pub type ExternalFenceHandleTypeFlagBits = core::VkExternalFenceHandleTypeFlagBits;

pub type ExternalFenceFeatureFlagBits = core::VkExternalFenceFeatureFlagBits;

pub type FenceImportFlagBits = core::VkFenceImportFlagBits;

pub type SemaphoreImportFlagBits = core::VkSemaphoreImportFlagBits;

pub type ExternalSemaphoreHandleTypeFlagBits = core::VkExternalSemaphoreHandleTypeFlagBits;

pub type ExternalSemaphoreFeatureFlagBits = core::VkExternalSemaphoreFeatureFlagBits;

pub type ResolveModeFlagBits = core::VkResolveModeFlagBits;

pub type DescriptorBindingFlagBits = core::VkDescriptorBindingFlagBits;

pub type SemaphoreWaitFlagBits = core::VkSemaphoreWaitFlagBits;

pub type PipelineCreationFeedbackFlagBits = core::VkPipelineCreationFeedbackFlagBits;

pub type ToolPurposeFlagBits = core::VkToolPurposeFlagBits;

pub type SubmitFlagBits = core::VkSubmitFlagBits;

pub type RenderingFlagBits = core::VkRenderingFlagBits;

//
// Utils:
//

pub type Inner<T> = <T as pod::Wrapper>::Inner;

#[inline]
pub const fn cast_pod_slice<T: pod::Wrapper>(outer: &[T]) -> &[Inner<T>] {
	unsafe { slice::from_raw_parts(outer.as_ptr() as *const Inner<T>, outer.len()) }
}

#[inline]
pub const fn cast_pod_slice_mut<T: pod::Wrapper>(outer: &mut [T]) -> &mut [Inner<T>] {
	unsafe { slice::from_raw_parts_mut(outer.as_mut_ptr() as *mut Inner<T>, outer.len()) }
}

pub fn select_dedicated_queue_family_indices(family_list: &[pod::QueueFamilyProperties], exclude_mask: u32) -> (u32, u32, u32) {
	let mut family_index = [usize::MAX; 3];

	let not_excluded = |n: usize| -> bool { ((exclude_mask as usize) & (1 << n)) == 0 };

	//
	// 1st pass to find a dedicated queue family for graphics:
	//

	for (n, family) in family_list.iter().enumerate() {
		if not_excluded(n) && family.has_dedicated_graphics_queues() {
			family_index[0] = n;
			break;
		}
	}

	//
	// 2nd pass to find a general queue family for graphics as fallback:
	//

	if family_index[0] == usize::MAX {
		for (n, family) in family_list.iter().enumerate() {
			if not_excluded(n) && family.has_graphics_queues() {
				family_index[0] = n;
				break;
			}
		}
	}

	if (exclude_mask == 0) && (family_index[0] == usize::MAX) {
		crate::panic!("Unable to find a queue family for graphics in the device");
	}

	//
	// 3rd pass to find a dedicated queue family for compute:
	//

	for (n, family) in family_list.iter().enumerate() {
		if not_excluded(n) && family.has_dedicated_compute_queues() {
			family_index[1] = n;
			break;
		}
	}

	//
	// 4th pass to find a general and unused queue family for compute as fallback:
	//

	if family_index[1] == usize::MAX {
		for (n, family) in family_list.iter().enumerate() {
			if not_excluded(n) && family.has_compute_queues() && (n != family_index[0]) {
				family_index[1] = n;
				break;
			}
		}
	}

	if family_index[1] == usize::MAX {
		// NOTE: The last compute family fallback is the graphics one.
		family_index[1] = family_index[0];
	}

	//
	// 5th pass to find a dedicated queue family for transfer (DMA-like):
	//

	for (n, family) in family_list.iter().enumerate() {
		if not_excluded(n) && family.has_dedicated_transfer_queues() {
			family_index[2] = n;
			break;
		}
	}

	//
	// 6th pass to find a general and unused queue family for transfer as fallback:
	//

	if family_index[2] == usize::MAX {
		for (n, family) in family_list.iter().enumerate() {
			if not_excluded(n) && family.has_transfer_queues() && (n != family_index[0]) && (n != family_index[1]) {
				family_index[2] = n;
				break;
			}
		}
	}

	if family_index[2] == usize::MAX {
		// NOTE: The last transfer family fallback is the compute one.
		family_index[2] = family_index[1];
	}

	(family_index[0] as _, family_index[1] as _, family_index[2] as _)
}

pub fn sort_dedicated_queue_family_indices(family_list: &[pod::QueueFamilyProperties], graphics_list: &mut [u32], compute_list: &mut [u32], transfer_list: &mut [u32]) -> (u32, u32, u32) {
	let mut graphics_count = 0;

	let mut compute_count = 0;

	let mut transfer_count = 0;

	let mut exclude_mask = 0;

	loop {
		let (graphics_index, compute_index, transfer_index) = select_dedicated_queue_family_indices(family_list, exclude_mask);

		if (graphics_index == u32::MAX) && (compute_index == u32::MAX) && (transfer_index == u32::MAX) {
			break;
		}

		if (graphics_index != u32::MAX) && (graphics_count < graphics_list.len()) {
			graphics_list[graphics_count] = graphics_index;
			exclude_mask |= 1 << graphics_index;
			graphics_count += 1;
		}

		if (compute_index != u32::MAX) && (compute_index != graphics_index) && (compute_count < compute_list.len()) {
			compute_list[compute_count] = compute_index;
			exclude_mask |= 1 << compute_index;
			compute_count += 1;
		}

		if (transfer_index != u32::MAX) && (transfer_index != compute_index) && (transfer_index != graphics_index) && (transfer_count < transfer_list.len()) {
			transfer_list[transfer_count] = transfer_index;
			exclude_mask |= 1 << transfer_index;
			transfer_count += 1;
		}

		if (graphics_count >= graphics_list.len()) && (compute_count >= compute_list.len()) && (transfer_count >= transfer_list.len()) {
			break;
		}
	}

	(graphics_count as _, compute_count as _, transfer_count as _)
}

#[inline]
pub fn fn_cstr_typename<Fn: 'static>() -> *const i8 {
	fn_typename::<Fn>().as_bytes().as_ptr() as *const i8
}

pub fn fn_typename<Fn: 'static>() -> &'static str {
	let type_id = any::TypeId::of::<Fn>();

	if type_id == any::TypeId::of::<core::PFN_vkAllocationFunction>() {
		"vkAllocationFunction\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeFunction>() {
		"vkFreeFunction\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInternalAllocationNotification>() {
		"vkInternalAllocationNotification\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInternalFreeNotification>() {
		"vkInternalFreeNotification\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReallocationFunction>() {
		"vkReallocationFunction\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkVoidFunction>() {
		"vkVoidFunction\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateInstance>() {
		"vkCreateInstance\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyInstance>() {
		"vkDestroyInstance\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDevices>() {
		"vkEnumeratePhysicalDevices\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures>() {
		"vkGetPhysicalDeviceFeatures\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties>() {
		"vkGetPhysicalDeviceFormatProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties>() {
		"vkGetPhysicalDeviceImageFormatProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties>() {
		"vkGetPhysicalDeviceProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties>() {
		"vkGetPhysicalDeviceQueueFamilyProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties>() {
		"vkGetPhysicalDeviceMemoryProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetInstanceProcAddr>() {
		"vkGetInstanceProcAddr\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceProcAddr>() {
		"vkGetDeviceProcAddr\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDevice>() {
		"vkCreateDevice\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDevice>() {
		"vkDestroyDevice\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceExtensionProperties>() {
		"vkEnumerateInstanceExtensionProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateDeviceExtensionProperties>() {
		"vkEnumerateDeviceExtensionProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceLayerProperties>() {
		"vkEnumerateInstanceLayerProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateDeviceLayerProperties>() {
		"vkEnumerateDeviceLayerProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceQueue>() {
		"vkGetDeviceQueue\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit>() {
		"vkQueueSubmit\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueWaitIdle>() {
		"vkQueueWaitIdle\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeviceWaitIdle>() {
		"vkDeviceWaitIdle\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateMemory>() {
		"vkAllocateMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeMemory>() {
		"vkFreeMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMapMemory>() {
		"vkMapMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUnmapMemory>() {
		"vkUnmapMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFlushMappedMemoryRanges>() {
		"vkFlushMappedMemoryRanges\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInvalidateMappedMemoryRanges>() {
		"vkInvalidateMappedMemoryRanges\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryCommitment>() {
		"vkGetDeviceMemoryCommitment\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory>() {
		"vkBindBufferMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory>() {
		"vkBindImageMemory\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements>() {
		"vkGetBufferMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements>() {
		"vkGetImageMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements>() {
		"vkGetImageSparseMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties>() {
		"vkGetPhysicalDeviceSparseImageFormatProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueBindSparse>() {
		"vkQueueBindSparse\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateFence>() {
		"vkCreateFence\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyFence>() {
		"vkDestroyFence\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetFences>() {
		"vkResetFences\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetFenceStatus>() {
		"vkGetFenceStatus\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitForFences>() {
		"vkWaitForFences\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSemaphore>() {
		"vkCreateSemaphore\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySemaphore>() {
		"vkDestroySemaphore\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateEvent>() {
		"vkCreateEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyEvent>() {
		"vkDestroyEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetEventStatus>() {
		"vkGetEventStatus\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetEvent>() {
		"vkSetEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetEvent>() {
		"vkResetEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateQueryPool>() {
		"vkCreateQueryPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyQueryPool>() {
		"vkDestroyQueryPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueryPoolResults>() {
		"vkGetQueryPoolResults\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateBuffer>() {
		"vkCreateBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyBuffer>() {
		"vkDestroyBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateBufferView>() {
		"vkCreateBufferView\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyBufferView>() {
		"vkDestroyBufferView\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateImage>() {
		"vkCreateImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyImage>() {
		"vkDestroyImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSubresourceLayout>() {
		"vkGetImageSubresourceLayout\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateImageView>() {
		"vkCreateImageView\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyImageView>() {
		"vkDestroyImageView\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateShaderModule>() {
		"vkCreateShaderModule\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyShaderModule>() {
		"vkDestroyShaderModule\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePipelineCache>() {
		"vkCreatePipelineCache\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipelineCache>() {
		"vkDestroyPipelineCache\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineCacheData>() {
		"vkGetPipelineCacheData\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMergePipelineCaches>() {
		"vkMergePipelineCaches\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateGraphicsPipelines>() {
		"vkCreateGraphicsPipelines\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateComputePipelines>() {
		"vkCreateComputePipelines\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipeline>() {
		"vkDestroyPipeline\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePipelineLayout>() {
		"vkCreatePipelineLayout\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipelineLayout>() {
		"vkDestroyPipelineLayout\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSampler>() {
		"vkCreateSampler\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySampler>() {
		"vkDestroySampler\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorSetLayout>() {
		"vkCreateDescriptorSetLayout\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorSetLayout>() {
		"vkDestroyDescriptorSetLayout\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorPool>() {
		"vkCreateDescriptorPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorPool>() {
		"vkDestroyDescriptorPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetDescriptorPool>() {
		"vkResetDescriptorPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateDescriptorSets>() {
		"vkAllocateDescriptorSets\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeDescriptorSets>() {
		"vkFreeDescriptorSets\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSets>() {
		"vkUpdateDescriptorSets\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateFramebuffer>() {
		"vkCreateFramebuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyFramebuffer>() {
		"vkDestroyFramebuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass>() {
		"vkCreateRenderPass\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyRenderPass>() {
		"vkDestroyRenderPass\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRenderAreaGranularity>() {
		"vkGetRenderAreaGranularity\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCommandPool>() {
		"vkCreateCommandPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCommandPool>() {
		"vkDestroyCommandPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetCommandPool>() {
		"vkResetCommandPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateCommandBuffers>() {
		"vkAllocateCommandBuffers\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeCommandBuffers>() {
		"vkFreeCommandBuffers\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBeginCommandBuffer>() {
		"vkBeginCommandBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEndCommandBuffer>() {
		"vkEndCommandBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetCommandBuffer>() {
		"vkResetCommandBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindPipeline>() {
		"vkCmdBindPipeline\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewport>() {
		"vkCmdSetViewport\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissor>() {
		"vkCmdSetScissor\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLineWidth>() {
		"vkCmdSetLineWidth\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBias>() {
		"vkCmdSetDepthBias\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetBlendConstants>() {
		"vkCmdSetBlendConstants\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBounds>() {
		"vkCmdSetDepthBounds\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilCompareMask>() {
		"vkCmdSetStencilCompareMask\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilWriteMask>() {
		"vkCmdSetStencilWriteMask\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilReference>() {
		"vkCmdSetStencilReference\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindDescriptorSets>() {
		"vkCmdBindDescriptorSets\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindIndexBuffer>() {
		"vkCmdBindIndexBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers>() {
		"vkCmdBindVertexBuffers\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDraw>() {
		"vkCmdDraw\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexed>() {
		"vkCmdDrawIndexed\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirect>() {
		"vkCmdDrawIndirect\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirect>() {
		"vkCmdDrawIndexedIndirect\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatch>() {
		"vkCmdDispatch\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchIndirect>() {
		"vkCmdDispatchIndirect\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer>() {
		"vkCmdCopyBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage>() {
		"vkCmdCopyImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage>() {
		"vkCmdBlitImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage>() {
		"vkCmdCopyBufferToImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer>() {
		"vkCmdCopyImageToBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdUpdateBuffer>() {
		"vkCmdUpdateBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdFillBuffer>() {
		"vkCmdFillBuffer\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearColorImage>() {
		"vkCmdClearColorImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearDepthStencilImage>() {
		"vkCmdClearDepthStencilImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearAttachments>() {
		"vkCmdClearAttachments\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage>() {
		"vkCmdResolveImage\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent>() {
		"vkCmdSetEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent>() {
		"vkCmdResetEvent\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents>() {
		"vkCmdWaitEvents\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier>() {
		"vkCmdPipelineBarrier\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginQuery>() {
		"vkCmdBeginQuery\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndQuery>() {
		"vkCmdEndQuery\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetQueryPool>() {
		"vkCmdResetQueryPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp>() {
		"vkCmdWriteTimestamp\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyQueryPoolResults>() {
		"vkCmdCopyQueryPoolResults\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushConstants>() {
		"vkCmdPushConstants\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass>() {
		"vkCmdBeginRenderPass\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass>() {
		"vkCmdNextSubpass\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass>() {
		"vkCmdEndRenderPass\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdExecuteCommands>() {
		"vkCmdExecuteCommands\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceVersion>() {
		"vkEnumerateInstanceVersion\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory2>() {
		"vkBindBufferMemory2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory2>() {
		"vkBindImageMemory2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPeerMemoryFeatures>() {
		"vkGetDeviceGroupPeerMemoryFeatures\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDeviceMask>() {
		"vkCmdSetDeviceMask\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchBase>() {
		"vkCmdDispatchBase\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceGroups>() {
		"vkEnumeratePhysicalDeviceGroups\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements2>() {
		"vkGetImageMemoryRequirements2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements2>() {
		"vkGetBufferMemoryRequirements2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements2>() {
		"vkGetImageSparseMemoryRequirements2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures2>() {
		"vkGetPhysicalDeviceFeatures2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties2>() {
		"vkGetPhysicalDeviceProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties2>() {
		"vkGetPhysicalDeviceFormatProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties2>() {
		"vkGetPhysicalDeviceImageFormatProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>() {
		"vkGetPhysicalDeviceQueueFamilyProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties2>() {
		"vkGetPhysicalDeviceMemoryProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>() {
		"vkGetPhysicalDeviceSparseImageFormatProperties2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkTrimCommandPool>() {
		"vkTrimCommandPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceQueue2>() {
		"vkGetDeviceQueue2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSamplerYcbcrConversion>() {
		"vkCreateSamplerYcbcrConversion\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySamplerYcbcrConversion>() {
		"vkDestroySamplerYcbcrConversion\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorUpdateTemplate>() {
		"vkCreateDescriptorUpdateTemplate\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorUpdateTemplate>() {
		"vkDestroyDescriptorUpdateTemplate\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSetWithTemplate>() {
		"vkUpdateDescriptorSetWithTemplate\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalBufferProperties>() {
		"vkGetPhysicalDeviceExternalBufferProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalFenceProperties>() {
		"vkGetPhysicalDeviceExternalFenceProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>() {
		"vkGetPhysicalDeviceExternalSemaphoreProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutSupport>() {
		"vkGetDescriptorSetLayoutSupport\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCount>() {
		"vkCmdDrawIndirectCount\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCount>() {
		"vkCmdDrawIndexedIndirectCount\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass2>() {
		"vkCreateRenderPass2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass2>() {
		"vkCmdBeginRenderPass2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass2>() {
		"vkCmdNextSubpass2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass2>() {
		"vkCmdEndRenderPass2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetQueryPool>() {
		"vkResetQueryPool\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreCounterValue>() {
		"vkGetSemaphoreCounterValue\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitSemaphores>() {
		"vkWaitSemaphores\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSignalSemaphore>() {
		"vkSignalSemaphore\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddress>() {
		"vkGetBufferDeviceAddress\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferOpaqueCaptureAddress>() {
		"vkGetBufferOpaqueCaptureAddress\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryOpaqueCaptureAddress>() {
		"vkGetDeviceMemoryOpaqueCaptureAddress\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceToolProperties>() {
		"vkGetPhysicalDeviceToolProperties\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePrivateDataSlot>() {
		"vkCreatePrivateDataSlot\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPrivateDataSlot>() {
		"vkDestroyPrivateDataSlot\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetPrivateData>() {
		"vkSetPrivateData\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPrivateData>() {
		"vkGetPrivateData\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent2>() {
		"vkCmdSetEvent2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent2>() {
		"vkCmdResetEvent2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents2>() {
		"vkCmdWaitEvents2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier2>() {
		"vkCmdPipelineBarrier2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp2>() {
		"vkCmdWriteTimestamp2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit2>() {
		"vkQueueSubmit2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer2>() {
		"vkCmdCopyBuffer2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage2>() {
		"vkCmdCopyImage2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage2>() {
		"vkCmdCopyBufferToImage2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer2>() {
		"vkCmdCopyImageToBuffer2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage2>() {
		"vkCmdBlitImage2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage2>() {
		"vkCmdResolveImage2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRendering>() {
		"vkCmdBeginRendering\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRendering>() {
		"vkCmdEndRendering\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCullMode>() {
		"vkCmdSetCullMode\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFrontFace>() {
		"vkCmdSetFrontFace\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveTopology>() {
		"vkCmdSetPrimitiveTopology\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWithCount>() {
		"vkCmdSetViewportWithCount\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissorWithCount>() {
		"vkCmdSetScissorWithCount\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers2>() {
		"vkCmdBindVertexBuffers2\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthTestEnable>() {
		"vkCmdSetDepthTestEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthWriteEnable>() {
		"vkCmdSetDepthWriteEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthCompareOp>() {
		"vkCmdSetDepthCompareOp\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBoundsTestEnable>() {
		"vkCmdSetDepthBoundsTestEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilTestEnable>() {
		"vkCmdSetStencilTestEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilOp>() {
		"vkCmdSetStencilOp\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRasterizerDiscardEnable>() {
		"vkCmdSetRasterizerDiscardEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBiasEnable>() {
		"vkCmdSetDepthBiasEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveRestartEnable>() {
		"vkCmdSetPrimitiveRestartEnable\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceBufferMemoryRequirements>() {
		"vkGetDeviceBufferMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageMemoryRequirements>() {
		"vkGetDeviceImageMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageSparseMemoryRequirements>() {
		"vkGetDeviceImageSparseMemoryRequirements\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySurfaceKHR>() {
		"vkDestroySurfaceKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceSupportKHR>() {
		"vkGetPhysicalDeviceSurfaceSupportKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>() {
		"vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>() {
		"vkGetPhysicalDeviceSurfaceFormatsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>() {
		"vkGetPhysicalDeviceSurfacePresentModesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSwapchainKHR>() {
		"vkCreateSwapchainKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySwapchainKHR>() {
		"vkDestroySwapchainKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainImagesKHR>() {
		"vkGetSwapchainImagesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireNextImageKHR>() {
		"vkAcquireNextImageKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueuePresentKHR>() {
		"vkQueuePresentKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPresentCapabilitiesKHR>() {
		"vkGetDeviceGroupPresentCapabilitiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupSurfacePresentModesKHR>() {
		"vkGetDeviceGroupSurfacePresentModesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDevicePresentRectanglesKHR>() {
		"vkGetPhysicalDevicePresentRectanglesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireNextImage2KHR>() {
		"vkAcquireNextImage2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>() {
		"vkGetPhysicalDeviceDisplayPropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>() {
		"vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneSupportedDisplaysKHR>() {
		"vkGetDisplayPlaneSupportedDisplaysKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayModePropertiesKHR>() {
		"vkGetDisplayModePropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDisplayModeKHR>() {
		"vkCreateDisplayModeKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneCapabilitiesKHR>() {
		"vkGetDisplayPlaneCapabilitiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDisplayPlaneSurfaceKHR>() {
		"vkCreateDisplayPlaneSurfaceKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSharedSwapchainsKHR>() {
		"vkCreateSharedSwapchainsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderingKHR>() {
		"vkCmdBeginRenderingKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderingKHR>() {
		"vkCmdEndRenderingKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures2KHR>() {
		"vkGetPhysicalDeviceFeatures2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties2KHR>() {
		"vkGetPhysicalDeviceProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties2KHR>() {
		"vkGetPhysicalDeviceFormatProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>() {
		"vkGetPhysicalDeviceImageFormatProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>() {
		"vkGetPhysicalDeviceQueueFamilyProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties2KHR>() {
		"vkGetPhysicalDeviceMemoryProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>() {
		"vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR>() {
		"vkGetDeviceGroupPeerMemoryFeaturesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDeviceMaskKHR>() {
		"vkCmdSetDeviceMaskKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchBaseKHR>() {
		"vkCmdDispatchBaseKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkTrimCommandPoolKHR>() {
		"vkTrimCommandPoolKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceGroupsKHR>() {
		"vkEnumeratePhysicalDeviceGroupsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>() {
		"vkGetPhysicalDeviceExternalBufferPropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryFdKHR>() {
		"vkGetMemoryFdKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryFdPropertiesKHR>() {
		"vkGetMemoryFdPropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>() {
		"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkImportSemaphoreFdKHR>() {
		"vkImportSemaphoreFdKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreFdKHR>() {
		"vkGetSemaphoreFdKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushDescriptorSetKHR>() {
		"vkCmdPushDescriptorSetKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushDescriptorSetWithTemplateKHR>() {
		"vkCmdPushDescriptorSetWithTemplateKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorUpdateTemplateKHR>() {
		"vkCreateDescriptorUpdateTemplateKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorUpdateTemplateKHR>() {
		"vkDestroyDescriptorUpdateTemplateKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSetWithTemplateKHR>() {
		"vkUpdateDescriptorSetWithTemplateKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass2KHR>() {
		"vkCreateRenderPass2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass2KHR>() {
		"vkCmdBeginRenderPass2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass2KHR>() {
		"vkCmdNextSubpass2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass2KHR>() {
		"vkCmdEndRenderPass2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainStatusKHR>() {
		"vkGetSwapchainStatusKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>() {
		"vkGetPhysicalDeviceExternalFencePropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkImportFenceFdKHR>() {
		"vkImportFenceFdKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetFenceFdKHR>() {
		"vkGetFenceFdKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>() {
		"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>() {
		"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireProfilingLockKHR>() {
		"vkAcquireProfilingLockKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleaseProfilingLockKHR>() {
		"vkReleaseProfilingLockKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>() {
		"vkGetPhysicalDeviceSurfaceCapabilities2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>() {
		"vkGetPhysicalDeviceSurfaceFormats2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayProperties2KHR>() {
		"vkGetPhysicalDeviceDisplayProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>() {
		"vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayModeProperties2KHR>() {
		"vkGetDisplayModeProperties2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneCapabilities2KHR>() {
		"vkGetDisplayPlaneCapabilities2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements2KHR>() {
		"vkGetImageMemoryRequirements2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements2KHR>() {
		"vkGetBufferMemoryRequirements2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements2KHR>() {
		"vkGetImageSparseMemoryRequirements2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSamplerYcbcrConversionKHR>() {
		"vkCreateSamplerYcbcrConversionKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySamplerYcbcrConversionKHR>() {
		"vkDestroySamplerYcbcrConversionKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory2KHR>() {
		"vkBindBufferMemory2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory2KHR>() {
		"vkBindImageMemory2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutSupportKHR>() {
		"vkGetDescriptorSetLayoutSupportKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCountKHR>() {
		"vkCmdDrawIndirectCountKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCountKHR>() {
		"vkCmdDrawIndexedIndirectCountKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreCounterValueKHR>() {
		"vkGetSemaphoreCounterValueKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitSemaphoresKHR>() {
		"vkWaitSemaphoresKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSignalSemaphoreKHR>() {
		"vkSignalSemaphoreKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>() {
		"vkGetPhysicalDeviceFragmentShadingRatesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFragmentShadingRateKHR>() {
		"vkCmdSetFragmentShadingRateKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitForPresentKHR>() {
		"vkWaitForPresentKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddressKHR>() {
		"vkGetBufferDeviceAddressKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferOpaqueCaptureAddressKHR>() {
		"vkGetBufferOpaqueCaptureAddressKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>() {
		"vkGetDeviceMemoryOpaqueCaptureAddressKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDeferredOperationKHR>() {
		"vkCreateDeferredOperationKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDeferredOperationKHR>() {
		"vkDestroyDeferredOperationKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeferredOperationMaxConcurrencyKHR>() {
		"vkGetDeferredOperationMaxConcurrencyKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeferredOperationResultKHR>() {
		"vkGetDeferredOperationResultKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeferredOperationJoinKHR>() {
		"vkDeferredOperationJoinKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutablePropertiesKHR>() {
		"vkGetPipelineExecutablePropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutableStatisticsKHR>() {
		"vkGetPipelineExecutableStatisticsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutableInternalRepresentationsKHR>() {
		"vkGetPipelineExecutableInternalRepresentationsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent2KHR>() {
		"vkCmdSetEvent2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent2KHR>() {
		"vkCmdResetEvent2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents2KHR>() {
		"vkCmdWaitEvents2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier2KHR>() {
		"vkCmdPipelineBarrier2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp2KHR>() {
		"vkCmdWriteTimestamp2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit2KHR>() {
		"vkQueueSubmit2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteBufferMarker2AMD>() {
		"vkCmdWriteBufferMarker2AMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueueCheckpointData2NV>() {
		"vkGetQueueCheckpointData2NV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer2KHR>() {
		"vkCmdCopyBuffer2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage2KHR>() {
		"vkCmdCopyImage2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage2KHR>() {
		"vkCmdCopyBufferToImage2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer2KHR>() {
		"vkCmdCopyImageToBuffer2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage2KHR>() {
		"vkCmdBlitImage2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage2KHR>() {
		"vkCmdResolveImage2KHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceBufferMemoryRequirementsKHR>() {
		"vkGetDeviceBufferMemoryRequirementsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageMemoryRequirementsKHR>() {
		"vkGetDeviceImageMemoryRequirementsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageSparseMemoryRequirementsKHR>() {
		"vkGetDeviceImageSparseMemoryRequirementsKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugReportCallbackEXT>() {
		"vkDebugReportCallbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDebugReportCallbackEXT>() {
		"vkCreateDebugReportCallbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDebugReportCallbackEXT>() {
		"vkDestroyDebugReportCallbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugReportMessageEXT>() {
		"vkDebugReportMessageEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugMarkerSetObjectTagEXT>() {
		"vkDebugMarkerSetObjectTagEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugMarkerSetObjectNameEXT>() {
		"vkDebugMarkerSetObjectNameEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerBeginEXT>() {
		"vkCmdDebugMarkerBeginEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerEndEXT>() {
		"vkCmdDebugMarkerEndEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerInsertEXT>() {
		"vkCmdDebugMarkerInsertEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindTransformFeedbackBuffersEXT>() {
		"vkCmdBindTransformFeedbackBuffersEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginTransformFeedbackEXT>() {
		"vkCmdBeginTransformFeedbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndTransformFeedbackEXT>() {
		"vkCmdEndTransformFeedbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginQueryIndexedEXT>() {
		"vkCmdBeginQueryIndexedEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndQueryIndexedEXT>() {
		"vkCmdEndQueryIndexedEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectByteCountEXT>() {
		"vkCmdDrawIndirectByteCountEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCuModuleNVX>() {
		"vkCreateCuModuleNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCuFunctionNVX>() {
		"vkCreateCuFunctionNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCuModuleNVX>() {
		"vkDestroyCuModuleNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCuFunctionNVX>() {
		"vkDestroyCuFunctionNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCuLaunchKernelNVX>() {
		"vkCmdCuLaunchKernelNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageViewHandleNVX>() {
		"vkGetImageViewHandleNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageViewAddressNVX>() {
		"vkGetImageViewAddressNVX\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCountAMD>() {
		"vkCmdDrawIndirectCountAMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCountAMD>() {
		"vkCmdDrawIndexedIndirectCountAMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetShaderInfoAMD>() {
		"vkGetShaderInfoAMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>() {
		"vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginConditionalRenderingEXT>() {
		"vkCmdBeginConditionalRenderingEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndConditionalRenderingEXT>() {
		"vkCmdEndConditionalRenderingEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWScalingNV>() {
		"vkCmdSetViewportWScalingNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleaseDisplayEXT>() {
		"vkReleaseDisplayEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>() {
		"vkGetPhysicalDeviceSurfaceCapabilities2EXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDisplayPowerControlEXT>() {
		"vkDisplayPowerControlEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkRegisterDeviceEventEXT>() {
		"vkRegisterDeviceEventEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkRegisterDisplayEventEXT>() {
		"vkRegisterDisplayEventEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainCounterEXT>() {
		"vkGetSwapchainCounterEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRefreshCycleDurationGOOGLE>() {
		"vkGetRefreshCycleDurationGOOGLE\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPastPresentationTimingGOOGLE>() {
		"vkGetPastPresentationTimingGOOGLE\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDiscardRectangleEXT>() {
		"vkCmdSetDiscardRectangleEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetHdrMetadataEXT>() {
		"vkSetHdrMetadataEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugUtilsMessengerCallbackEXT>() {
		"vkDebugUtilsMessengerCallbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDebugUtilsObjectNameEXT>() {
		"vkSetDebugUtilsObjectNameEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDebugUtilsObjectTagEXT>() {
		"vkSetDebugUtilsObjectTagEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueBeginDebugUtilsLabelEXT>() {
		"vkQueueBeginDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueEndDebugUtilsLabelEXT>() {
		"vkQueueEndDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueInsertDebugUtilsLabelEXT>() {
		"vkQueueInsertDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginDebugUtilsLabelEXT>() {
		"vkCmdBeginDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndDebugUtilsLabelEXT>() {
		"vkCmdEndDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdInsertDebugUtilsLabelEXT>() {
		"vkCmdInsertDebugUtilsLabelEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDebugUtilsMessengerEXT>() {
		"vkCreateDebugUtilsMessengerEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDebugUtilsMessengerEXT>() {
		"vkDestroyDebugUtilsMessengerEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSubmitDebugUtilsMessageEXT>() {
		"vkSubmitDebugUtilsMessageEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetSampleLocationsEXT>() {
		"vkCmdSetSampleLocationsEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>() {
		"vkGetPhysicalDeviceMultisamplePropertiesEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageDrmFormatModifierPropertiesEXT>() {
		"vkGetImageDrmFormatModifierPropertiesEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateValidationCacheEXT>() {
		"vkCreateValidationCacheEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyValidationCacheEXT>() {
		"vkDestroyValidationCacheEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMergeValidationCachesEXT>() {
		"vkMergeValidationCachesEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetValidationCacheDataEXT>() {
		"vkGetValidationCacheDataEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindShadingRateImageNV>() {
		"vkCmdBindShadingRateImageNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportShadingRatePaletteNV>() {
		"vkCmdSetViewportShadingRatePaletteNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCoarseSampleOrderNV>() {
		"vkCmdSetCoarseSampleOrderNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateAccelerationStructureNV>() {
		"vkCreateAccelerationStructureNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyAccelerationStructureNV>() {
		"vkDestroyAccelerationStructureNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureMemoryRequirementsNV>() {
		"vkGetAccelerationStructureMemoryRequirementsNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindAccelerationStructureMemoryNV>() {
		"vkBindAccelerationStructureMemoryNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructureNV>() {
		"vkCmdBuildAccelerationStructureNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureNV>() {
		"vkCmdCopyAccelerationStructureNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysNV>() {
		"vkCmdTraceRaysNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRayTracingPipelinesNV>() {
		"vkCreateRayTracingPipelinesNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupHandlesKHR>() {
		"vkGetRayTracingShaderGroupHandlesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupHandlesNV>() {
		"vkGetRayTracingShaderGroupHandlesNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureHandleNV>() {
		"vkGetAccelerationStructureHandleNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteAccelerationStructuresPropertiesNV>() {
		"vkCmdWriteAccelerationStructuresPropertiesNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCompileDeferredNV>() {
		"vkCompileDeferredNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryHostPointerPropertiesEXT>() {
		"vkGetMemoryHostPointerPropertiesEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteBufferMarkerAMD>() {
		"vkCmdWriteBufferMarkerAMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>() {
		"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetCalibratedTimestampsEXT>() {
		"vkGetCalibratedTimestampsEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksNV>() {
		"vkCmdDrawMeshTasksNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksIndirectNV>() {
		"vkCmdDrawMeshTasksIndirectNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksIndirectCountNV>() {
		"vkCmdDrawMeshTasksIndirectCountNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetExclusiveScissorNV>() {
		"vkCmdSetExclusiveScissorNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCheckpointNV>() {
		"vkCmdSetCheckpointNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueueCheckpointDataNV>() {
		"vkGetQueueCheckpointDataNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInitializePerformanceApiINTEL>() {
		"vkInitializePerformanceApiINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUninitializePerformanceApiINTEL>() {
		"vkUninitializePerformanceApiINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceMarkerINTEL>() {
		"vkCmdSetPerformanceMarkerINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceStreamMarkerINTEL>() {
		"vkCmdSetPerformanceStreamMarkerINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceOverrideINTEL>() {
		"vkCmdSetPerformanceOverrideINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquirePerformanceConfigurationINTEL>() {
		"vkAcquirePerformanceConfigurationINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleasePerformanceConfigurationINTEL>() {
		"vkReleasePerformanceConfigurationINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSetPerformanceConfigurationINTEL>() {
		"vkQueueSetPerformanceConfigurationINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPerformanceParameterINTEL>() {
		"vkGetPerformanceParameterINTEL\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetLocalDimmingAMD>() {
		"vkSetLocalDimmingAMD\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddressEXT>() {
		"vkGetBufferDeviceAddressEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceToolPropertiesEXT>() {
		"vkGetPhysicalDeviceToolPropertiesEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>() {
		"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>() {
		"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateHeadlessSurfaceEXT>() {
		"vkCreateHeadlessSurfaceEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLineStippleEXT>() {
		"vkCmdSetLineStippleEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetQueryPoolEXT>() {
		"vkResetQueryPoolEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCullModeEXT>() {
		"vkCmdSetCullModeEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFrontFaceEXT>() {
		"vkCmdSetFrontFaceEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveTopologyEXT>() {
		"vkCmdSetPrimitiveTopologyEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWithCountEXT>() {
		"vkCmdSetViewportWithCountEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissorWithCountEXT>() {
		"vkCmdSetScissorWithCountEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers2EXT>() {
		"vkCmdBindVertexBuffers2EXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthTestEnableEXT>() {
		"vkCmdSetDepthTestEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthWriteEnableEXT>() {
		"vkCmdSetDepthWriteEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthCompareOpEXT>() {
		"vkCmdSetDepthCompareOpEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBoundsTestEnableEXT>() {
		"vkCmdSetDepthBoundsTestEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilTestEnableEXT>() {
		"vkCmdSetStencilTestEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilOpEXT>() {
		"vkCmdSetStencilOpEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetGeneratedCommandsMemoryRequirementsNV>() {
		"vkGetGeneratedCommandsMemoryRequirementsNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPreprocessGeneratedCommandsNV>() {
		"vkCmdPreprocessGeneratedCommandsNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdExecuteGeneratedCommandsNV>() {
		"vkCmdExecuteGeneratedCommandsNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindPipelineShaderGroupNV>() {
		"vkCmdBindPipelineShaderGroupNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateIndirectCommandsLayoutNV>() {
		"vkCreateIndirectCommandsLayoutNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyIndirectCommandsLayoutNV>() {
		"vkDestroyIndirectCommandsLayoutNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeviceMemoryReportCallbackEXT>() {
		"vkDeviceMemoryReportCallbackEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireDrmDisplayEXT>() {
		"vkAcquireDrmDisplayEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDrmDisplayEXT>() {
		"vkGetDrmDisplayEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePrivateDataSlotEXT>() {
		"vkCreatePrivateDataSlotEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPrivateDataSlotEXT>() {
		"vkDestroyPrivateDataSlotEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetPrivateDataEXT>() {
		"vkSetPrivateDataEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPrivateDataEXT>() {
		"vkGetPrivateDataEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFragmentShadingRateEnumNV>() {
		"vkCmdSetFragmentShadingRateEnumNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireWinrtDisplayNV>() {
		"vkAcquireWinrtDisplayNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetWinrtDisplayNV>() {
		"vkGetWinrtDisplayNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetVertexInputEXT>() {
		"vkCmdSetVertexInputEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI>() {
		"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSubpassShadingHUAWEI>() {
		"vkCmdSubpassShadingHUAWEI\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindInvocationMaskHUAWEI>() {
		"vkCmdBindInvocationMaskHUAWEI\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryRemoteAddressNV>() {
		"vkGetMemoryRemoteAddressNV\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPatchControlPointsEXT>() {
		"vkCmdSetPatchControlPointsEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRasterizerDiscardEnableEXT>() {
		"vkCmdSetRasterizerDiscardEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBiasEnableEXT>() {
		"vkCmdSetDepthBiasEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLogicOpEXT>() {
		"vkCmdSetLogicOpEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveRestartEnableEXT>() {
		"vkCmdSetPrimitiveRestartEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetColorWriteEnableEXT>() {
		"vkCmdSetColorWriteEnableEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMultiEXT>() {
		"vkCmdDrawMultiEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMultiIndexedEXT>() {
		"vkCmdDrawMultiIndexedEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDeviceMemoryPriorityEXT>() {
		"vkSetDeviceMemoryPriorityEXT\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE>() {
		"vkGetDescriptorSetLayoutHostMappingInfoVALVE\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetHostMappingVALVE>() {
		"vkGetDescriptorSetHostMappingVALVE\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateAccelerationStructureKHR>() {
		"vkCreateAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyAccelerationStructureKHR>() {
		"vkDestroyAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructuresKHR>() {
		"vkCmdBuildAccelerationStructuresKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructuresIndirectKHR>() {
		"vkCmdBuildAccelerationStructuresIndirectKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBuildAccelerationStructuresKHR>() {
		"vkBuildAccelerationStructuresKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyAccelerationStructureKHR>() {
		"vkCopyAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyAccelerationStructureToMemoryKHR>() {
		"vkCopyAccelerationStructureToMemoryKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyMemoryToAccelerationStructureKHR>() {
		"vkCopyMemoryToAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWriteAccelerationStructuresPropertiesKHR>() {
		"vkWriteAccelerationStructuresPropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureKHR>() {
		"vkCmdCopyAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureToMemoryKHR>() {
		"vkCmdCopyAccelerationStructureToMemoryKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyMemoryToAccelerationStructureKHR>() {
		"vkCmdCopyMemoryToAccelerationStructureKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureDeviceAddressKHR>() {
		"vkGetAccelerationStructureDeviceAddressKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR>() {
		"vkCmdWriteAccelerationStructuresPropertiesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR>() {
		"vkGetDeviceAccelerationStructureCompatibilityKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureBuildSizesKHR>() {
		"vkGetAccelerationStructureBuildSizesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysKHR>() {
		"vkCmdTraceRaysKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRayTracingPipelinesKHR>() {
		"vkCreateRayTracingPipelinesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>() {
		"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysIndirectKHR>() {
		"vkCmdTraceRaysIndirectKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupStackSizeKHR>() {
		"vkGetRayTracingShaderGroupStackSizeKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRayTracingPipelineStackSizeKHR>() {
		"vkCmdSetRayTracingPipelineStackSizeKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<xcb::PFN_vkCreateXcbSurfaceKHR>() {
		"vkCreateXcbSurfaceKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<xcb::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>() {
		"vkGetPhysicalDeviceXcbPresentationSupportKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<wayland::PFN_vkCreateWaylandSurfaceKHR>() {
		"vkCreateWaylandSurfaceKHR\0"
	}
	//
	else if type_id == any::TypeId::of::<wayland::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>() {
		"vkGetPhysicalDeviceWaylandPresentationSupportKHR\0"
	}
	//
	else {
		crate::panic!("Unable to convert the pointer function {} into a C string", any::type_name::<Fn>());
	}
}
