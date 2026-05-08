mod dispatch;

//
// API dynamic loading:
//

pub use dispatch::VULKAN_LIBRARY_FILENAME;

pub use dispatch::MOLTENVK_LIBRARY_FILENAME;

pub use dispatch::InstanceExtensionName;

pub use dispatch::DeviceExtensionName;

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

pub fn fn_typename<Fn: 'static>() -> *const i8 {
	let type_id = any::TypeId::of::<Fn>();

	if type_id == any::TypeId::of::<core::PFN_vkAllocationFunction>() {
		c_str("vkAllocationFunction\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeFunction>() {
		c_str("vkFreeFunction\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInternalAllocationNotification>() {
		c_str("vkInternalAllocationNotification\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInternalFreeNotification>() {
		c_str("vkInternalFreeNotification\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReallocationFunction>() {
		c_str("vkReallocationFunction\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkVoidFunction>() {
		c_str("vkVoidFunction\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateInstance>() {
		c_str("vkCreateInstance\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyInstance>() {
		c_str("vkDestroyInstance\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDevices>() {
		c_str("vkEnumeratePhysicalDevices\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures>() {
		c_str("vkGetPhysicalDeviceFeatures\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties>() {
		c_str("vkGetPhysicalDeviceFormatProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties>() {
		c_str("vkGetPhysicalDeviceImageFormatProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties>() {
		c_str("vkGetPhysicalDeviceProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties>() {
		c_str("vkGetPhysicalDeviceQueueFamilyProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties>() {
		c_str("vkGetPhysicalDeviceMemoryProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetInstanceProcAddr>() {
		c_str("vkGetInstanceProcAddr\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceProcAddr>() {
		c_str("vkGetDeviceProcAddr\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDevice>() {
		c_str("vkCreateDevice\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDevice>() {
		c_str("vkDestroyDevice\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceExtensionProperties>() {
		c_str("vkEnumerateInstanceExtensionProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateDeviceExtensionProperties>() {
		c_str("vkEnumerateDeviceExtensionProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceLayerProperties>() {
		c_str("vkEnumerateInstanceLayerProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateDeviceLayerProperties>() {
		c_str("vkEnumerateDeviceLayerProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceQueue>() {
		c_str("vkGetDeviceQueue\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit>() {
		c_str("vkQueueSubmit\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueWaitIdle>() {
		c_str("vkQueueWaitIdle\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeviceWaitIdle>() {
		c_str("vkDeviceWaitIdle\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateMemory>() {
		c_str("vkAllocateMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeMemory>() {
		c_str("vkFreeMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMapMemory>() {
		c_str("vkMapMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUnmapMemory>() {
		c_str("vkUnmapMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFlushMappedMemoryRanges>() {
		c_str("vkFlushMappedMemoryRanges\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInvalidateMappedMemoryRanges>() {
		c_str("vkInvalidateMappedMemoryRanges\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryCommitment>() {
		c_str("vkGetDeviceMemoryCommitment\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory>() {
		c_str("vkBindBufferMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory>() {
		c_str("vkBindImageMemory\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements>() {
		c_str("vkGetBufferMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements>() {
		c_str("vkGetImageMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements>() {
		c_str("vkGetImageSparseMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties>() {
		c_str("vkGetPhysicalDeviceSparseImageFormatProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueBindSparse>() {
		c_str("vkQueueBindSparse\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateFence>() {
		c_str("vkCreateFence\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyFence>() {
		c_str("vkDestroyFence\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetFences>() {
		c_str("vkResetFences\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetFenceStatus>() {
		c_str("vkGetFenceStatus\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitForFences>() {
		c_str("vkWaitForFences\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSemaphore>() {
		c_str("vkCreateSemaphore\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySemaphore>() {
		c_str("vkDestroySemaphore\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateEvent>() {
		c_str("vkCreateEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyEvent>() {
		c_str("vkDestroyEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetEventStatus>() {
		c_str("vkGetEventStatus\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetEvent>() {
		c_str("vkSetEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetEvent>() {
		c_str("vkResetEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateQueryPool>() {
		c_str("vkCreateQueryPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyQueryPool>() {
		c_str("vkDestroyQueryPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueryPoolResults>() {
		c_str("vkGetQueryPoolResults\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateBuffer>() {
		c_str("vkCreateBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyBuffer>() {
		c_str("vkDestroyBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateBufferView>() {
		c_str("vkCreateBufferView\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyBufferView>() {
		c_str("vkDestroyBufferView\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateImage>() {
		c_str("vkCreateImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyImage>() {
		c_str("vkDestroyImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSubresourceLayout>() {
		c_str("vkGetImageSubresourceLayout\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateImageView>() {
		c_str("vkCreateImageView\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyImageView>() {
		c_str("vkDestroyImageView\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateShaderModule>() {
		c_str("vkCreateShaderModule\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyShaderModule>() {
		c_str("vkDestroyShaderModule\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePipelineCache>() {
		c_str("vkCreatePipelineCache\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipelineCache>() {
		c_str("vkDestroyPipelineCache\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineCacheData>() {
		c_str("vkGetPipelineCacheData\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMergePipelineCaches>() {
		c_str("vkMergePipelineCaches\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateGraphicsPipelines>() {
		c_str("vkCreateGraphicsPipelines\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateComputePipelines>() {
		c_str("vkCreateComputePipelines\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipeline>() {
		c_str("vkDestroyPipeline\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePipelineLayout>() {
		c_str("vkCreatePipelineLayout\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPipelineLayout>() {
		c_str("vkDestroyPipelineLayout\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSampler>() {
		c_str("vkCreateSampler\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySampler>() {
		c_str("vkDestroySampler\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorSetLayout>() {
		c_str("vkCreateDescriptorSetLayout\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorSetLayout>() {
		c_str("vkDestroyDescriptorSetLayout\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorPool>() {
		c_str("vkCreateDescriptorPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorPool>() {
		c_str("vkDestroyDescriptorPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetDescriptorPool>() {
		c_str("vkResetDescriptorPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateDescriptorSets>() {
		c_str("vkAllocateDescriptorSets\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeDescriptorSets>() {
		c_str("vkFreeDescriptorSets\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSets>() {
		c_str("vkUpdateDescriptorSets\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateFramebuffer>() {
		c_str("vkCreateFramebuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyFramebuffer>() {
		c_str("vkDestroyFramebuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass>() {
		c_str("vkCreateRenderPass\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyRenderPass>() {
		c_str("vkDestroyRenderPass\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRenderAreaGranularity>() {
		c_str("vkGetRenderAreaGranularity\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCommandPool>() {
		c_str("vkCreateCommandPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCommandPool>() {
		c_str("vkDestroyCommandPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetCommandPool>() {
		c_str("vkResetCommandPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAllocateCommandBuffers>() {
		c_str("vkAllocateCommandBuffers\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkFreeCommandBuffers>() {
		c_str("vkFreeCommandBuffers\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBeginCommandBuffer>() {
		c_str("vkBeginCommandBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEndCommandBuffer>() {
		c_str("vkEndCommandBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetCommandBuffer>() {
		c_str("vkResetCommandBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindPipeline>() {
		c_str("vkCmdBindPipeline\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewport>() {
		c_str("vkCmdSetViewport\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissor>() {
		c_str("vkCmdSetScissor\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLineWidth>() {
		c_str("vkCmdSetLineWidth\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBias>() {
		c_str("vkCmdSetDepthBias\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetBlendConstants>() {
		c_str("vkCmdSetBlendConstants\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBounds>() {
		c_str("vkCmdSetDepthBounds\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilCompareMask>() {
		c_str("vkCmdSetStencilCompareMask\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilWriteMask>() {
		c_str("vkCmdSetStencilWriteMask\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilReference>() {
		c_str("vkCmdSetStencilReference\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindDescriptorSets>() {
		c_str("vkCmdBindDescriptorSets\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindIndexBuffer>() {
		c_str("vkCmdBindIndexBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers>() {
		c_str("vkCmdBindVertexBuffers\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDraw>() {
		c_str("vkCmdDraw\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexed>() {
		c_str("vkCmdDrawIndexed\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirect>() {
		c_str("vkCmdDrawIndirect\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirect>() {
		c_str("vkCmdDrawIndexedIndirect\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatch>() {
		c_str("vkCmdDispatch\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchIndirect>() {
		c_str("vkCmdDispatchIndirect\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer>() {
		c_str("vkCmdCopyBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage>() {
		c_str("vkCmdCopyImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage>() {
		c_str("vkCmdBlitImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage>() {
		c_str("vkCmdCopyBufferToImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer>() {
		c_str("vkCmdCopyImageToBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdUpdateBuffer>() {
		c_str("vkCmdUpdateBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdFillBuffer>() {
		c_str("vkCmdFillBuffer\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearColorImage>() {
		c_str("vkCmdClearColorImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearDepthStencilImage>() {
		c_str("vkCmdClearDepthStencilImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdClearAttachments>() {
		c_str("vkCmdClearAttachments\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage>() {
		c_str("vkCmdResolveImage\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent>() {
		c_str("vkCmdSetEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent>() {
		c_str("vkCmdResetEvent\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents>() {
		c_str("vkCmdWaitEvents\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier>() {
		c_str("vkCmdPipelineBarrier\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginQuery>() {
		c_str("vkCmdBeginQuery\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndQuery>() {
		c_str("vkCmdEndQuery\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetQueryPool>() {
		c_str("vkCmdResetQueryPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp>() {
		c_str("vkCmdWriteTimestamp\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyQueryPoolResults>() {
		c_str("vkCmdCopyQueryPoolResults\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushConstants>() {
		c_str("vkCmdPushConstants\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass>() {
		c_str("vkCmdBeginRenderPass\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass>() {
		c_str("vkCmdNextSubpass\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass>() {
		c_str("vkCmdEndRenderPass\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdExecuteCommands>() {
		c_str("vkCmdExecuteCommands\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumerateInstanceVersion>() {
		c_str("vkEnumerateInstanceVersion\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory2>() {
		c_str("vkBindBufferMemory2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory2>() {
		c_str("vkBindImageMemory2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPeerMemoryFeatures>() {
		c_str("vkGetDeviceGroupPeerMemoryFeatures\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDeviceMask>() {
		c_str("vkCmdSetDeviceMask\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchBase>() {
		c_str("vkCmdDispatchBase\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceGroups>() {
		c_str("vkEnumeratePhysicalDeviceGroups\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements2>() {
		c_str("vkGetImageMemoryRequirements2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements2>() {
		c_str("vkGetBufferMemoryRequirements2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements2>() {
		c_str("vkGetImageSparseMemoryRequirements2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures2>() {
		c_str("vkGetPhysicalDeviceFeatures2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties2>() {
		c_str("vkGetPhysicalDeviceProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties2>() {
		c_str("vkGetPhysicalDeviceFormatProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties2>() {
		c_str("vkGetPhysicalDeviceImageFormatProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties2>() {
		c_str("vkGetPhysicalDeviceQueueFamilyProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties2>() {
		c_str("vkGetPhysicalDeviceMemoryProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2>() {
		c_str("vkGetPhysicalDeviceSparseImageFormatProperties2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkTrimCommandPool>() {
		c_str("vkTrimCommandPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceQueue2>() {
		c_str("vkGetDeviceQueue2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSamplerYcbcrConversion>() {
		c_str("vkCreateSamplerYcbcrConversion\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySamplerYcbcrConversion>() {
		c_str("vkDestroySamplerYcbcrConversion\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorUpdateTemplate>() {
		c_str("vkCreateDescriptorUpdateTemplate\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorUpdateTemplate>() {
		c_str("vkDestroyDescriptorUpdateTemplate\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSetWithTemplate>() {
		c_str("vkUpdateDescriptorSetWithTemplate\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalBufferProperties>() {
		c_str("vkGetPhysicalDeviceExternalBufferProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalFenceProperties>() {
		c_str("vkGetPhysicalDeviceExternalFenceProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties>() {
		c_str("vkGetPhysicalDeviceExternalSemaphoreProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutSupport>() {
		c_str("vkGetDescriptorSetLayoutSupport\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCount>() {
		c_str("vkCmdDrawIndirectCount\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCount>() {
		c_str("vkCmdDrawIndexedIndirectCount\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass2>() {
		c_str("vkCreateRenderPass2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass2>() {
		c_str("vkCmdBeginRenderPass2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass2>() {
		c_str("vkCmdNextSubpass2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass2>() {
		c_str("vkCmdEndRenderPass2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetQueryPool>() {
		c_str("vkResetQueryPool\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreCounterValue>() {
		c_str("vkGetSemaphoreCounterValue\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitSemaphores>() {
		c_str("vkWaitSemaphores\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSignalSemaphore>() {
		c_str("vkSignalSemaphore\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddress>() {
		c_str("vkGetBufferDeviceAddress\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferOpaqueCaptureAddress>() {
		c_str("vkGetBufferOpaqueCaptureAddress\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryOpaqueCaptureAddress>() {
		c_str("vkGetDeviceMemoryOpaqueCaptureAddress\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceToolProperties>() {
		c_str("vkGetPhysicalDeviceToolProperties\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePrivateDataSlot>() {
		c_str("vkCreatePrivateDataSlot\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPrivateDataSlot>() {
		c_str("vkDestroyPrivateDataSlot\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetPrivateData>() {
		c_str("vkSetPrivateData\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPrivateData>() {
		c_str("vkGetPrivateData\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent2>() {
		c_str("vkCmdSetEvent2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent2>() {
		c_str("vkCmdResetEvent2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents2>() {
		c_str("vkCmdWaitEvents2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier2>() {
		c_str("vkCmdPipelineBarrier2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp2>() {
		c_str("vkCmdWriteTimestamp2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit2>() {
		c_str("vkQueueSubmit2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer2>() {
		c_str("vkCmdCopyBuffer2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage2>() {
		c_str("vkCmdCopyImage2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage2>() {
		c_str("vkCmdCopyBufferToImage2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer2>() {
		c_str("vkCmdCopyImageToBuffer2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage2>() {
		c_str("vkCmdBlitImage2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage2>() {
		c_str("vkCmdResolveImage2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRendering>() {
		c_str("vkCmdBeginRendering\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRendering>() {
		c_str("vkCmdEndRendering\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCullMode>() {
		c_str("vkCmdSetCullMode\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFrontFace>() {
		c_str("vkCmdSetFrontFace\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveTopology>() {
		c_str("vkCmdSetPrimitiveTopology\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWithCount>() {
		c_str("vkCmdSetViewportWithCount\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissorWithCount>() {
		c_str("vkCmdSetScissorWithCount\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers2>() {
		c_str("vkCmdBindVertexBuffers2\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthTestEnable>() {
		c_str("vkCmdSetDepthTestEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthWriteEnable>() {
		c_str("vkCmdSetDepthWriteEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthCompareOp>() {
		c_str("vkCmdSetDepthCompareOp\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBoundsTestEnable>() {
		c_str("vkCmdSetDepthBoundsTestEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilTestEnable>() {
		c_str("vkCmdSetStencilTestEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilOp>() {
		c_str("vkCmdSetStencilOp\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRasterizerDiscardEnable>() {
		c_str("vkCmdSetRasterizerDiscardEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBiasEnable>() {
		c_str("vkCmdSetDepthBiasEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveRestartEnable>() {
		c_str("vkCmdSetPrimitiveRestartEnable\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceBufferMemoryRequirements>() {
		c_str("vkGetDeviceBufferMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageMemoryRequirements>() {
		c_str("vkGetDeviceImageMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageSparseMemoryRequirements>() {
		c_str("vkGetDeviceImageSparseMemoryRequirements\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySurfaceKHR>() {
		c_str("vkDestroySurfaceKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceSupportKHR>() {
		c_str("vkGetPhysicalDeviceSurfaceSupportKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR>() {
		c_str("vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR>() {
		c_str("vkGetPhysicalDeviceSurfaceFormatsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR>() {
		c_str("vkGetPhysicalDeviceSurfacePresentModesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSwapchainKHR>() {
		c_str("vkCreateSwapchainKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySwapchainKHR>() {
		c_str("vkDestroySwapchainKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainImagesKHR>() {
		c_str("vkGetSwapchainImagesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireNextImageKHR>() {
		c_str("vkAcquireNextImageKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueuePresentKHR>() {
		c_str("vkQueuePresentKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPresentCapabilitiesKHR>() {
		c_str("vkGetDeviceGroupPresentCapabilitiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupSurfacePresentModesKHR>() {
		c_str("vkGetDeviceGroupSurfacePresentModesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDevicePresentRectanglesKHR>() {
		c_str("vkGetPhysicalDevicePresentRectanglesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireNextImage2KHR>() {
		c_str("vkAcquireNextImage2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPropertiesKHR>() {
		c_str("vkGetPhysicalDeviceDisplayPropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPlanePropertiesKHR>() {
		c_str("vkGetPhysicalDeviceDisplayPlanePropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneSupportedDisplaysKHR>() {
		c_str("vkGetDisplayPlaneSupportedDisplaysKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayModePropertiesKHR>() {
		c_str("vkGetDisplayModePropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDisplayModeKHR>() {
		c_str("vkCreateDisplayModeKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneCapabilitiesKHR>() {
		c_str("vkGetDisplayPlaneCapabilitiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDisplayPlaneSurfaceKHR>() {
		c_str("vkCreateDisplayPlaneSurfaceKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSharedSwapchainsKHR>() {
		c_str("vkCreateSharedSwapchainsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderingKHR>() {
		c_str("vkCmdBeginRenderingKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderingKHR>() {
		c_str("vkCmdEndRenderingKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFeatures2KHR>() {
		c_str("vkGetPhysicalDeviceFeatures2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceProperties2KHR>() {
		c_str("vkGetPhysicalDeviceProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFormatProperties2KHR>() {
		c_str("vkGetPhysicalDeviceFormatProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceImageFormatProperties2KHR>() {
		c_str("vkGetPhysicalDeviceImageFormatProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyProperties2KHR>() {
		c_str("vkGetPhysicalDeviceQueueFamilyProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMemoryProperties2KHR>() {
		c_str("vkGetPhysicalDeviceMemoryProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2KHR>() {
		c_str("vkGetPhysicalDeviceSparseImageFormatProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceGroupPeerMemoryFeaturesKHR>() {
		c_str("vkGetDeviceGroupPeerMemoryFeaturesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDeviceMaskKHR>() {
		c_str("vkCmdSetDeviceMaskKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDispatchBaseKHR>() {
		c_str("vkCmdDispatchBaseKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkTrimCommandPoolKHR>() {
		c_str("vkTrimCommandPoolKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceGroupsKHR>() {
		c_str("vkEnumeratePhysicalDeviceGroupsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalBufferPropertiesKHR>() {
		c_str("vkGetPhysicalDeviceExternalBufferPropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryFdKHR>() {
		c_str("vkGetMemoryFdKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryFdPropertiesKHR>() {
		c_str("vkGetMemoryFdPropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>() {
		c_str("vkGetPhysicalDeviceExternalSemaphorePropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkImportSemaphoreFdKHR>() {
		c_str("vkImportSemaphoreFdKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreFdKHR>() {
		c_str("vkGetSemaphoreFdKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushDescriptorSetKHR>() {
		c_str("vkCmdPushDescriptorSetKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPushDescriptorSetWithTemplateKHR>() {
		c_str("vkCmdPushDescriptorSetWithTemplateKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDescriptorUpdateTemplateKHR>() {
		c_str("vkCreateDescriptorUpdateTemplateKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDescriptorUpdateTemplateKHR>() {
		c_str("vkDestroyDescriptorUpdateTemplateKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUpdateDescriptorSetWithTemplateKHR>() {
		c_str("vkUpdateDescriptorSetWithTemplateKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRenderPass2KHR>() {
		c_str("vkCreateRenderPass2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginRenderPass2KHR>() {
		c_str("vkCmdBeginRenderPass2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdNextSubpass2KHR>() {
		c_str("vkCmdNextSubpass2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndRenderPass2KHR>() {
		c_str("vkCmdEndRenderPass2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainStatusKHR>() {
		c_str("vkGetSwapchainStatusKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalFencePropertiesKHR>() {
		c_str("vkGetPhysicalDeviceExternalFencePropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkImportFenceFdKHR>() {
		c_str("vkImportFenceFdKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetFenceFdKHR>() {
		c_str("vkGetFenceFdKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>() {
		c_str("vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>() {
		c_str("vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireProfilingLockKHR>() {
		c_str("vkAcquireProfilingLockKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleaseProfilingLockKHR>() {
		c_str("vkReleaseProfilingLockKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilities2KHR>() {
		c_str("vkGetPhysicalDeviceSurfaceCapabilities2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceFormats2KHR>() {
		c_str("vkGetPhysicalDeviceSurfaceFormats2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayProperties2KHR>() {
		c_str("vkGetPhysicalDeviceDisplayProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceDisplayPlaneProperties2KHR>() {
		c_str("vkGetPhysicalDeviceDisplayPlaneProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayModeProperties2KHR>() {
		c_str("vkGetDisplayModeProperties2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDisplayPlaneCapabilities2KHR>() {
		c_str("vkGetDisplayPlaneCapabilities2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageMemoryRequirements2KHR>() {
		c_str("vkGetImageMemoryRequirements2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferMemoryRequirements2KHR>() {
		c_str("vkGetBufferMemoryRequirements2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageSparseMemoryRequirements2KHR>() {
		c_str("vkGetImageSparseMemoryRequirements2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateSamplerYcbcrConversionKHR>() {
		c_str("vkCreateSamplerYcbcrConversionKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroySamplerYcbcrConversionKHR>() {
		c_str("vkDestroySamplerYcbcrConversionKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindBufferMemory2KHR>() {
		c_str("vkBindBufferMemory2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindImageMemory2KHR>() {
		c_str("vkBindImageMemory2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutSupportKHR>() {
		c_str("vkGetDescriptorSetLayoutSupportKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCountKHR>() {
		c_str("vkCmdDrawIndirectCountKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCountKHR>() {
		c_str("vkCmdDrawIndexedIndirectCountKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSemaphoreCounterValueKHR>() {
		c_str("vkGetSemaphoreCounterValueKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitSemaphoresKHR>() {
		c_str("vkWaitSemaphoresKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSignalSemaphoreKHR>() {
		c_str("vkSignalSemaphoreKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR>() {
		c_str("vkGetPhysicalDeviceFragmentShadingRatesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFragmentShadingRateKHR>() {
		c_str("vkCmdSetFragmentShadingRateKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWaitForPresentKHR>() {
		c_str("vkWaitForPresentKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddressKHR>() {
		c_str("vkGetBufferDeviceAddressKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferOpaqueCaptureAddressKHR>() {
		c_str("vkGetBufferOpaqueCaptureAddressKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceMemoryOpaqueCaptureAddressKHR>() {
		c_str("vkGetDeviceMemoryOpaqueCaptureAddressKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDeferredOperationKHR>() {
		c_str("vkCreateDeferredOperationKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDeferredOperationKHR>() {
		c_str("vkDestroyDeferredOperationKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeferredOperationMaxConcurrencyKHR>() {
		c_str("vkGetDeferredOperationMaxConcurrencyKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeferredOperationResultKHR>() {
		c_str("vkGetDeferredOperationResultKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeferredOperationJoinKHR>() {
		c_str("vkDeferredOperationJoinKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutablePropertiesKHR>() {
		c_str("vkGetPipelineExecutablePropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutableStatisticsKHR>() {
		c_str("vkGetPipelineExecutableStatisticsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPipelineExecutableInternalRepresentationsKHR>() {
		c_str("vkGetPipelineExecutableInternalRepresentationsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetEvent2KHR>() {
		c_str("vkCmdSetEvent2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResetEvent2KHR>() {
		c_str("vkCmdResetEvent2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWaitEvents2KHR>() {
		c_str("vkCmdWaitEvents2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPipelineBarrier2KHR>() {
		c_str("vkCmdPipelineBarrier2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteTimestamp2KHR>() {
		c_str("vkCmdWriteTimestamp2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSubmit2KHR>() {
		c_str("vkQueueSubmit2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteBufferMarker2AMD>() {
		c_str("vkCmdWriteBufferMarker2AMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueueCheckpointData2NV>() {
		c_str("vkGetQueueCheckpointData2NV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBuffer2KHR>() {
		c_str("vkCmdCopyBuffer2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImage2KHR>() {
		c_str("vkCmdCopyImage2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyBufferToImage2KHR>() {
		c_str("vkCmdCopyBufferToImage2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyImageToBuffer2KHR>() {
		c_str("vkCmdCopyImageToBuffer2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBlitImage2KHR>() {
		c_str("vkCmdBlitImage2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdResolveImage2KHR>() {
		c_str("vkCmdResolveImage2KHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceBufferMemoryRequirementsKHR>() {
		c_str("vkGetDeviceBufferMemoryRequirementsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageMemoryRequirementsKHR>() {
		c_str("vkGetDeviceImageMemoryRequirementsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceImageSparseMemoryRequirementsKHR>() {
		c_str("vkGetDeviceImageSparseMemoryRequirementsKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugReportCallbackEXT>() {
		c_str("vkDebugReportCallbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDebugReportCallbackEXT>() {
		c_str("vkCreateDebugReportCallbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDebugReportCallbackEXT>() {
		c_str("vkDestroyDebugReportCallbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugReportMessageEXT>() {
		c_str("vkDebugReportMessageEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugMarkerSetObjectTagEXT>() {
		c_str("vkDebugMarkerSetObjectTagEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugMarkerSetObjectNameEXT>() {
		c_str("vkDebugMarkerSetObjectNameEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerBeginEXT>() {
		c_str("vkCmdDebugMarkerBeginEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerEndEXT>() {
		c_str("vkCmdDebugMarkerEndEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDebugMarkerInsertEXT>() {
		c_str("vkCmdDebugMarkerInsertEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindTransformFeedbackBuffersEXT>() {
		c_str("vkCmdBindTransformFeedbackBuffersEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginTransformFeedbackEXT>() {
		c_str("vkCmdBeginTransformFeedbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndTransformFeedbackEXT>() {
		c_str("vkCmdEndTransformFeedbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginQueryIndexedEXT>() {
		c_str("vkCmdBeginQueryIndexedEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndQueryIndexedEXT>() {
		c_str("vkCmdEndQueryIndexedEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectByteCountEXT>() {
		c_str("vkCmdDrawIndirectByteCountEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCuModuleNVX>() {
		c_str("vkCreateCuModuleNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateCuFunctionNVX>() {
		c_str("vkCreateCuFunctionNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCuModuleNVX>() {
		c_str("vkDestroyCuModuleNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyCuFunctionNVX>() {
		c_str("vkDestroyCuFunctionNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCuLaunchKernelNVX>() {
		c_str("vkCmdCuLaunchKernelNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageViewHandleNVX>() {
		c_str("vkGetImageViewHandleNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageViewAddressNVX>() {
		c_str("vkGetImageViewAddressNVX\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndirectCountAMD>() {
		c_str("vkCmdDrawIndirectCountAMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawIndexedIndirectCountAMD>() {
		c_str("vkCmdDrawIndexedIndirectCountAMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetShaderInfoAMD>() {
		c_str("vkGetShaderInfoAMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceExternalImageFormatPropertiesNV>() {
		c_str("vkGetPhysicalDeviceExternalImageFormatPropertiesNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginConditionalRenderingEXT>() {
		c_str("vkCmdBeginConditionalRenderingEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndConditionalRenderingEXT>() {
		c_str("vkCmdEndConditionalRenderingEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWScalingNV>() {
		c_str("vkCmdSetViewportWScalingNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleaseDisplayEXT>() {
		c_str("vkReleaseDisplayEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT>() {
		c_str("vkGetPhysicalDeviceSurfaceCapabilities2EXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDisplayPowerControlEXT>() {
		c_str("vkDisplayPowerControlEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkRegisterDeviceEventEXT>() {
		c_str("vkRegisterDeviceEventEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkRegisterDisplayEventEXT>() {
		c_str("vkRegisterDisplayEventEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetSwapchainCounterEXT>() {
		c_str("vkGetSwapchainCounterEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRefreshCycleDurationGOOGLE>() {
		c_str("vkGetRefreshCycleDurationGOOGLE\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPastPresentationTimingGOOGLE>() {
		c_str("vkGetPastPresentationTimingGOOGLE\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDiscardRectangleEXT>() {
		c_str("vkCmdSetDiscardRectangleEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetHdrMetadataEXT>() {
		c_str("vkSetHdrMetadataEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDebugUtilsMessengerCallbackEXT>() {
		c_str("vkDebugUtilsMessengerCallbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDebugUtilsObjectNameEXT>() {
		c_str("vkSetDebugUtilsObjectNameEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDebugUtilsObjectTagEXT>() {
		c_str("vkSetDebugUtilsObjectTagEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueBeginDebugUtilsLabelEXT>() {
		c_str("vkQueueBeginDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueEndDebugUtilsLabelEXT>() {
		c_str("vkQueueEndDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueInsertDebugUtilsLabelEXT>() {
		c_str("vkQueueInsertDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBeginDebugUtilsLabelEXT>() {
		c_str("vkCmdBeginDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdEndDebugUtilsLabelEXT>() {
		c_str("vkCmdEndDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdInsertDebugUtilsLabelEXT>() {
		c_str("vkCmdInsertDebugUtilsLabelEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateDebugUtilsMessengerEXT>() {
		c_str("vkCreateDebugUtilsMessengerEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyDebugUtilsMessengerEXT>() {
		c_str("vkDestroyDebugUtilsMessengerEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSubmitDebugUtilsMessageEXT>() {
		c_str("vkSubmitDebugUtilsMessageEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetSampleLocationsEXT>() {
		c_str("vkCmdSetSampleLocationsEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT>() {
		c_str("vkGetPhysicalDeviceMultisamplePropertiesEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetImageDrmFormatModifierPropertiesEXT>() {
		c_str("vkGetImageDrmFormatModifierPropertiesEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateValidationCacheEXT>() {
		c_str("vkCreateValidationCacheEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyValidationCacheEXT>() {
		c_str("vkDestroyValidationCacheEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkMergeValidationCachesEXT>() {
		c_str("vkMergeValidationCachesEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetValidationCacheDataEXT>() {
		c_str("vkGetValidationCacheDataEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindShadingRateImageNV>() {
		c_str("vkCmdBindShadingRateImageNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportShadingRatePaletteNV>() {
		c_str("vkCmdSetViewportShadingRatePaletteNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCoarseSampleOrderNV>() {
		c_str("vkCmdSetCoarseSampleOrderNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateAccelerationStructureNV>() {
		c_str("vkCreateAccelerationStructureNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyAccelerationStructureNV>() {
		c_str("vkDestroyAccelerationStructureNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureMemoryRequirementsNV>() {
		c_str("vkGetAccelerationStructureMemoryRequirementsNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBindAccelerationStructureMemoryNV>() {
		c_str("vkBindAccelerationStructureMemoryNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructureNV>() {
		c_str("vkCmdBuildAccelerationStructureNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureNV>() {
		c_str("vkCmdCopyAccelerationStructureNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysNV>() {
		c_str("vkCmdTraceRaysNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRayTracingPipelinesNV>() {
		c_str("vkCreateRayTracingPipelinesNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupHandlesKHR>() {
		c_str("vkGetRayTracingShaderGroupHandlesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupHandlesNV>() {
		c_str("vkGetRayTracingShaderGroupHandlesNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureHandleNV>() {
		c_str("vkGetAccelerationStructureHandleNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteAccelerationStructuresPropertiesNV>() {
		c_str("vkCmdWriteAccelerationStructuresPropertiesNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCompileDeferredNV>() {
		c_str("vkCompileDeferredNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryHostPointerPropertiesEXT>() {
		c_str("vkGetMemoryHostPointerPropertiesEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteBufferMarkerAMD>() {
		c_str("vkCmdWriteBufferMarkerAMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceCalibrateableTimeDomainsEXT>() {
		c_str("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetCalibratedTimestampsEXT>() {
		c_str("vkGetCalibratedTimestampsEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksNV>() {
		c_str("vkCmdDrawMeshTasksNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksIndirectNV>() {
		c_str("vkCmdDrawMeshTasksIndirectNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMeshTasksIndirectCountNV>() {
		c_str("vkCmdDrawMeshTasksIndirectCountNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetExclusiveScissorNV>() {
		c_str("vkCmdSetExclusiveScissorNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCheckpointNV>() {
		c_str("vkCmdSetCheckpointNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetQueueCheckpointDataNV>() {
		c_str("vkGetQueueCheckpointDataNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkInitializePerformanceApiINTEL>() {
		c_str("vkInitializePerformanceApiINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkUninitializePerformanceApiINTEL>() {
		c_str("vkUninitializePerformanceApiINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceMarkerINTEL>() {
		c_str("vkCmdSetPerformanceMarkerINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceStreamMarkerINTEL>() {
		c_str("vkCmdSetPerformanceStreamMarkerINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPerformanceOverrideINTEL>() {
		c_str("vkCmdSetPerformanceOverrideINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquirePerformanceConfigurationINTEL>() {
		c_str("vkAcquirePerformanceConfigurationINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkReleasePerformanceConfigurationINTEL>() {
		c_str("vkReleasePerformanceConfigurationINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkQueueSetPerformanceConfigurationINTEL>() {
		c_str("vkQueueSetPerformanceConfigurationINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPerformanceParameterINTEL>() {
		c_str("vkGetPerformanceParameterINTEL\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetLocalDimmingAMD>() {
		c_str("vkSetLocalDimmingAMD\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetBufferDeviceAddressEXT>() {
		c_str("vkGetBufferDeviceAddressEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceToolPropertiesEXT>() {
		c_str("vkGetPhysicalDeviceToolPropertiesEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>() {
		c_str("vkGetPhysicalDeviceCooperativeMatrixPropertiesNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>() {
		c_str("vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateHeadlessSurfaceEXT>() {
		c_str("vkCreateHeadlessSurfaceEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLineStippleEXT>() {
		c_str("vkCmdSetLineStippleEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkResetQueryPoolEXT>() {
		c_str("vkResetQueryPoolEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetCullModeEXT>() {
		c_str("vkCmdSetCullModeEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFrontFaceEXT>() {
		c_str("vkCmdSetFrontFaceEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveTopologyEXT>() {
		c_str("vkCmdSetPrimitiveTopologyEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetViewportWithCountEXT>() {
		c_str("vkCmdSetViewportWithCountEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetScissorWithCountEXT>() {
		c_str("vkCmdSetScissorWithCountEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindVertexBuffers2EXT>() {
		c_str("vkCmdBindVertexBuffers2EXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthTestEnableEXT>() {
		c_str("vkCmdSetDepthTestEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthWriteEnableEXT>() {
		c_str("vkCmdSetDepthWriteEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthCompareOpEXT>() {
		c_str("vkCmdSetDepthCompareOpEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBoundsTestEnableEXT>() {
		c_str("vkCmdSetDepthBoundsTestEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilTestEnableEXT>() {
		c_str("vkCmdSetStencilTestEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetStencilOpEXT>() {
		c_str("vkCmdSetStencilOpEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetGeneratedCommandsMemoryRequirementsNV>() {
		c_str("vkGetGeneratedCommandsMemoryRequirementsNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdPreprocessGeneratedCommandsNV>() {
		c_str("vkCmdPreprocessGeneratedCommandsNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdExecuteGeneratedCommandsNV>() {
		c_str("vkCmdExecuteGeneratedCommandsNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindPipelineShaderGroupNV>() {
		c_str("vkCmdBindPipelineShaderGroupNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateIndirectCommandsLayoutNV>() {
		c_str("vkCreateIndirectCommandsLayoutNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyIndirectCommandsLayoutNV>() {
		c_str("vkDestroyIndirectCommandsLayoutNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDeviceMemoryReportCallbackEXT>() {
		c_str("vkDeviceMemoryReportCallbackEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireDrmDisplayEXT>() {
		c_str("vkAcquireDrmDisplayEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDrmDisplayEXT>() {
		c_str("vkGetDrmDisplayEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreatePrivateDataSlotEXT>() {
		c_str("vkCreatePrivateDataSlotEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyPrivateDataSlotEXT>() {
		c_str("vkDestroyPrivateDataSlotEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetPrivateDataEXT>() {
		c_str("vkSetPrivateDataEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetPrivateDataEXT>() {
		c_str("vkGetPrivateDataEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetFragmentShadingRateEnumNV>() {
		c_str("vkCmdSetFragmentShadingRateEnumNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkAcquireWinrtDisplayNV>() {
		c_str("vkAcquireWinrtDisplayNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetWinrtDisplayNV>() {
		c_str("vkGetWinrtDisplayNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetVertexInputEXT>() {
		c_str("vkCmdSetVertexInputEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI>() {
		c_str("vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSubpassShadingHUAWEI>() {
		c_str("vkCmdSubpassShadingHUAWEI\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBindInvocationMaskHUAWEI>() {
		c_str("vkCmdBindInvocationMaskHUAWEI\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetMemoryRemoteAddressNV>() {
		c_str("vkGetMemoryRemoteAddressNV\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPatchControlPointsEXT>() {
		c_str("vkCmdSetPatchControlPointsEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRasterizerDiscardEnableEXT>() {
		c_str("vkCmdSetRasterizerDiscardEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetDepthBiasEnableEXT>() {
		c_str("vkCmdSetDepthBiasEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetLogicOpEXT>() {
		c_str("vkCmdSetLogicOpEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetPrimitiveRestartEnableEXT>() {
		c_str("vkCmdSetPrimitiveRestartEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetColorWriteEnableEXT>() {
		c_str("vkCmdSetColorWriteEnableEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMultiEXT>() {
		c_str("vkCmdDrawMultiEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdDrawMultiIndexedEXT>() {
		c_str("vkCmdDrawMultiIndexedEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkSetDeviceMemoryPriorityEXT>() {
		c_str("vkSetDeviceMemoryPriorityEXT\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetLayoutHostMappingInfoVALVE>() {
		c_str("vkGetDescriptorSetLayoutHostMappingInfoVALVE\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDescriptorSetHostMappingVALVE>() {
		c_str("vkGetDescriptorSetHostMappingVALVE\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateAccelerationStructureKHR>() {
		c_str("vkCreateAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkDestroyAccelerationStructureKHR>() {
		c_str("vkDestroyAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructuresKHR>() {
		c_str("vkCmdBuildAccelerationStructuresKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdBuildAccelerationStructuresIndirectKHR>() {
		c_str("vkCmdBuildAccelerationStructuresIndirectKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkBuildAccelerationStructuresKHR>() {
		c_str("vkBuildAccelerationStructuresKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyAccelerationStructureKHR>() {
		c_str("vkCopyAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyAccelerationStructureToMemoryKHR>() {
		c_str("vkCopyAccelerationStructureToMemoryKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCopyMemoryToAccelerationStructureKHR>() {
		c_str("vkCopyMemoryToAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkWriteAccelerationStructuresPropertiesKHR>() {
		c_str("vkWriteAccelerationStructuresPropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureKHR>() {
		c_str("vkCmdCopyAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyAccelerationStructureToMemoryKHR>() {
		c_str("vkCmdCopyAccelerationStructureToMemoryKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdCopyMemoryToAccelerationStructureKHR>() {
		c_str("vkCmdCopyMemoryToAccelerationStructureKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureDeviceAddressKHR>() {
		c_str("vkGetAccelerationStructureDeviceAddressKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdWriteAccelerationStructuresPropertiesKHR>() {
		c_str("vkCmdWriteAccelerationStructuresPropertiesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetDeviceAccelerationStructureCompatibilityKHR>() {
		c_str("vkGetDeviceAccelerationStructureCompatibilityKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetAccelerationStructureBuildSizesKHR>() {
		c_str("vkGetAccelerationStructureBuildSizesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysKHR>() {
		c_str("vkCmdTraceRaysKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCreateRayTracingPipelinesKHR>() {
		c_str("vkCreateRayTracingPipelinesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>() {
		c_str("vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdTraceRaysIndirectKHR>() {
		c_str("vkCmdTraceRaysIndirectKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkGetRayTracingShaderGroupStackSizeKHR>() {
		c_str("vkGetRayTracingShaderGroupStackSizeKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<core::PFN_vkCmdSetRayTracingPipelineStackSizeKHR>() {
		c_str("vkCmdSetRayTracingPipelineStackSizeKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<xcb::PFN_vkCreateXcbSurfaceKHR>() {
		c_str("vkCreateXcbSurfaceKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<xcb::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR>() {
		c_str("vkGetPhysicalDeviceXcbPresentationSupportKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<wayland::PFN_vkCreateWaylandSurfaceKHR>() {
		c_str("vkCreateWaylandSurfaceKHR\0")
	}
	//
	else if type_id == any::TypeId::of::<wayland::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR>() {
		c_str("vkGetPhysicalDeviceWaylandPresentationSupportKHR\0")
	}
	//
	else {
		crate::panic!("Unable to convert the pointer function {} into a C string", any::type_name::<Fn>());
	}
}
