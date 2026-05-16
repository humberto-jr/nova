use ::core::cmp;

use crate::{
	ffi::c_str, //
	ffi::unix::wayland::client::protocol as wl,
	ffi::unix::x11,
	mem,
};

use super::{
	super::spirv, //
	core,
	wayland,
	xcb,
};

//
// Wrapper:
//

pub trait Wrapper {
	type Inner;
}

macro_rules! impl_Wrapper {
	($outer:ident, $inner:ty) => {
		#[repr(transparent)]
		#[derive(Clone)]
		pub struct $outer($inner);

		impl Wrapper for $outer {
			type Inner = $inner;
		}

		impl ::core::ops::Deref for $outer {
			type Target = $inner;

			#[inline(always)]
			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl ::core::ops::DerefMut for $outer {
			#[inline(always)]
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl $outer {
			#[inline(always)]
			pub const fn zeroed() -> Self {
				// NOTE: Inner types with function pointers cannot be zeroed in Rust and
				// therefore cannot have an outer type here. There are only just a few
				// occurrences.
				unsafe { Self(mem::zeroed()) }
			}

			#[inline(always)]
			pub const fn from(inner: $inner) -> Self {
				Self(inner)
			}
		}
	};
}

impl_Wrapper!(Extent2D, core::VkExtent2D);

impl_Wrapper!(Extent3D, core::VkExtent3D);

impl_Wrapper!(Offset2D, core::VkOffset2D);

impl_Wrapper!(Offset3D, core::VkOffset3D);

impl_Wrapper!(Rect2D, core::VkRect2D);

impl_Wrapper!(BaseInStructure, core::VkBaseInStructure);

impl_Wrapper!(BaseOutStructure, core::VkBaseOutStructure);

impl_Wrapper!(BufferMemoryBarrier, core::VkBufferMemoryBarrier);

impl_Wrapper!(DispatchIndirectCommand, core::VkDispatchIndirectCommand);

impl_Wrapper!(DrawIndexedIndirectCommand, core::VkDrawIndexedIndirectCommand);

impl_Wrapper!(DrawIndirectCommand, core::VkDrawIndirectCommand);

impl_Wrapper!(ImageSubresourceRange, core::VkImageSubresourceRange);

impl_Wrapper!(ImageMemoryBarrier, core::VkImageMemoryBarrier);

impl_Wrapper!(MemoryBarrier, core::VkMemoryBarrier);

impl_Wrapper!(PipelineCacheHeaderVersionOne, core::VkPipelineCacheHeaderVersionOne);

impl_Wrapper!(ApplicationInfo, core::VkApplicationInfo);

impl_Wrapper!(FormatProperties, core::VkFormatProperties);

impl_Wrapper!(ImageFormatProperties, core::VkImageFormatProperties);

impl_Wrapper!(InstanceCreateInfo, core::VkInstanceCreateInfo);

impl_Wrapper!(MemoryHeap, core::VkMemoryHeap);

impl_Wrapper!(MemoryType, core::VkMemoryType);

impl_Wrapper!(PhysicalDeviceFeatures, core::VkPhysicalDeviceFeatures);

impl_Wrapper!(PhysicalDeviceLimits, core::VkPhysicalDeviceLimits);

impl_Wrapper!(PhysicalDeviceMemoryProperties, core::VkPhysicalDeviceMemoryProperties);

impl_Wrapper!(PhysicalDeviceSparseProperties, core::VkPhysicalDeviceSparseProperties);

impl_Wrapper!(PhysicalDeviceProperties, core::VkPhysicalDeviceProperties);

impl_Wrapper!(QueueFamilyProperties, core::VkQueueFamilyProperties);

impl_Wrapper!(DeviceQueueCreateInfo, core::VkDeviceQueueCreateInfo);

impl_Wrapper!(DeviceCreateInfo, core::VkDeviceCreateInfo);

impl_Wrapper!(ExtensionProperties, core::VkExtensionProperties);

impl_Wrapper!(LayerProperties, core::VkLayerProperties);

impl_Wrapper!(SubmitInfo, core::VkSubmitInfo);

impl_Wrapper!(MappedMemoryRange, core::VkMappedMemoryRange);

impl_Wrapper!(MemoryAllocateInfo, core::VkMemoryAllocateInfo);

impl_Wrapper!(MemoryRequirements, core::VkMemoryRequirements);

impl_Wrapper!(SparseMemoryBind, core::VkSparseMemoryBind);

impl_Wrapper!(SparseBufferMemoryBindInfo, core::VkSparseBufferMemoryBindInfo);

impl_Wrapper!(SparseImageOpaqueMemoryBindInfo, core::VkSparseImageOpaqueMemoryBindInfo);

impl_Wrapper!(ImageSubresource, core::VkImageSubresource);

impl_Wrapper!(SparseImageMemoryBind, core::VkSparseImageMemoryBind);

impl_Wrapper!(SparseImageMemoryBindInfo, core::VkSparseImageMemoryBindInfo);

impl_Wrapper!(BindSparseInfo, core::VkBindSparseInfo);

impl_Wrapper!(SparseImageFormatProperties, core::VkSparseImageFormatProperties);

impl_Wrapper!(SparseImageMemoryRequirements, core::VkSparseImageMemoryRequirements);

impl_Wrapper!(FenceCreateInfo, core::VkFenceCreateInfo);

impl_Wrapper!(SemaphoreCreateInfo, core::VkSemaphoreCreateInfo);

impl_Wrapper!(EventCreateInfo, core::VkEventCreateInfo);

impl_Wrapper!(QueryPoolCreateInfo, core::VkQueryPoolCreateInfo);

impl_Wrapper!(BufferCreateInfo, core::VkBufferCreateInfo);

impl_Wrapper!(BufferViewCreateInfo, core::VkBufferViewCreateInfo);

impl_Wrapper!(ImageCreateInfo, core::VkImageCreateInfo);

impl_Wrapper!(SubresourceLayout, core::VkSubresourceLayout);

impl_Wrapper!(ComponentMapping, core::VkComponentMapping);

impl_Wrapper!(ImageViewCreateInfo, core::VkImageViewCreateInfo);

impl_Wrapper!(ShaderModuleCreateInfo, core::VkShaderModuleCreateInfo);

impl_Wrapper!(PipelineCacheCreateInfo, core::VkPipelineCacheCreateInfo);

impl_Wrapper!(SpecializationMapEntry, core::VkSpecializationMapEntry);

impl_Wrapper!(SpecializationInfo, core::VkSpecializationInfo);

impl_Wrapper!(PipelineShaderStageCreateInfo, core::VkPipelineShaderStageCreateInfo);

impl_Wrapper!(ComputePipelineCreateInfo, core::VkComputePipelineCreateInfo);

impl_Wrapper!(VertexInputBindingDescription, core::VkVertexInputBindingDescription);

impl_Wrapper!(VertexInputAttributeDescription, core::VkVertexInputAttributeDescription);

impl_Wrapper!(PipelineVertexInputStateCreateInfo, core::VkPipelineVertexInputStateCreateInfo);

impl_Wrapper!(PipelineInputAssemblyStateCreateInfo, core::VkPipelineInputAssemblyStateCreateInfo);

impl_Wrapper!(PipelineTessellationStateCreateInfo, core::VkPipelineTessellationStateCreateInfo);

impl_Wrapper!(Viewport, core::VkViewport);

impl_Wrapper!(PipelineViewportStateCreateInfo, core::VkPipelineViewportStateCreateInfo);

impl_Wrapper!(PipelineRasterizationStateCreateInfo, core::VkPipelineRasterizationStateCreateInfo);

impl_Wrapper!(PipelineMultisampleStateCreateInfo, core::VkPipelineMultisampleStateCreateInfo);

impl_Wrapper!(StencilOpState, core::VkStencilOpState);

impl_Wrapper!(PipelineDepthStencilStateCreateInfo, core::VkPipelineDepthStencilStateCreateInfo);

impl_Wrapper!(PipelineColorBlendAttachmentState, core::VkPipelineColorBlendAttachmentState);

impl_Wrapper!(PipelineColorBlendStateCreateInfo, core::VkPipelineColorBlendStateCreateInfo);

impl_Wrapper!(PipelineDynamicStateCreateInfo, core::VkPipelineDynamicStateCreateInfo);

impl_Wrapper!(GraphicsPipelineCreateInfo, core::VkGraphicsPipelineCreateInfo);

impl_Wrapper!(PushConstantRange, core::VkPushConstantRange);

impl_Wrapper!(PipelineLayoutCreateInfo, core::VkPipelineLayoutCreateInfo);

impl_Wrapper!(SamplerCreateInfo, core::VkSamplerCreateInfo);

impl_Wrapper!(CopyDescriptorSet, core::VkCopyDescriptorSet);

impl_Wrapper!(DescriptorBufferInfo, core::VkDescriptorBufferInfo);

impl_Wrapper!(DescriptorImageInfo, core::VkDescriptorImageInfo);

impl_Wrapper!(DescriptorPoolSize, core::VkDescriptorPoolSize);

impl_Wrapper!(DescriptorPoolCreateInfo, core::VkDescriptorPoolCreateInfo);

impl_Wrapper!(DescriptorSetAllocateInfo, core::VkDescriptorSetAllocateInfo);

impl_Wrapper!(DescriptorSetLayoutBinding, core::VkDescriptorSetLayoutBinding);

impl_Wrapper!(DescriptorSetLayoutCreateInfo, core::VkDescriptorSetLayoutCreateInfo);

impl_Wrapper!(WriteDescriptorSet, core::VkWriteDescriptorSet);

impl_Wrapper!(AttachmentDescription, core::VkAttachmentDescription);

impl_Wrapper!(AttachmentReference, core::VkAttachmentReference);

impl_Wrapper!(FramebufferCreateInfo, core::VkFramebufferCreateInfo);

impl_Wrapper!(SubpassDescription, core::VkSubpassDescription);

impl_Wrapper!(SubpassDependency, core::VkSubpassDependency);

impl_Wrapper!(RenderPassCreateInfo, core::VkRenderPassCreateInfo);

impl_Wrapper!(CommandPoolCreateInfo, core::VkCommandPoolCreateInfo);

impl_Wrapper!(CommandBufferAllocateInfo, core::VkCommandBufferAllocateInfo);

impl_Wrapper!(CommandBufferInheritanceInfo, core::VkCommandBufferInheritanceInfo);

impl_Wrapper!(CommandBufferBeginInfo, core::VkCommandBufferBeginInfo);

impl_Wrapper!(BufferCopy, core::VkBufferCopy);

impl_Wrapper!(ImageSubresourceLayers, core::VkImageSubresourceLayers);

impl_Wrapper!(BufferImageCopy, core::VkBufferImageCopy);

impl_Wrapper!(ClearDepthStencilValue, core::VkClearDepthStencilValue);

impl_Wrapper!(ClearAttachment, core::VkClearAttachment);

impl_Wrapper!(ClearRect, core::VkClearRect);

impl_Wrapper!(ImageBlit, core::VkImageBlit);

impl_Wrapper!(ImageCopy, core::VkImageCopy);

impl_Wrapper!(ImageResolve, core::VkImageResolve);

impl_Wrapper!(RenderPassBeginInfo, core::VkRenderPassBeginInfo);

impl_Wrapper!(PhysicalDeviceSubgroupProperties, core::VkPhysicalDeviceSubgroupProperties);

impl_Wrapper!(BindBufferMemoryInfo, core::VkBindBufferMemoryInfo);

impl_Wrapper!(BindImageMemoryInfo, core::VkBindImageMemoryInfo);

impl_Wrapper!(PhysicalDevice16BitStorageFeatures, core::VkPhysicalDevice16BitStorageFeatures);

impl_Wrapper!(MemoryDedicatedRequirements, core::VkMemoryDedicatedRequirements);

impl_Wrapper!(MemoryDedicatedAllocateInfo, core::VkMemoryDedicatedAllocateInfo);

impl_Wrapper!(MemoryAllocateFlagsInfo, core::VkMemoryAllocateFlagsInfo);

impl_Wrapper!(DeviceGroupRenderPassBeginInfo, core::VkDeviceGroupRenderPassBeginInfo);

impl_Wrapper!(DeviceGroupCommandBufferBeginInfo, core::VkDeviceGroupCommandBufferBeginInfo);

impl_Wrapper!(DeviceGroupSubmitInfo, core::VkDeviceGroupSubmitInfo);

impl_Wrapper!(DeviceGroupBindSparseInfo, core::VkDeviceGroupBindSparseInfo);

impl_Wrapper!(BindBufferMemoryDeviceGroupInfo, core::VkBindBufferMemoryDeviceGroupInfo);

impl_Wrapper!(BindImageMemoryDeviceGroupInfo, core::VkBindImageMemoryDeviceGroupInfo);

impl_Wrapper!(PhysicalDeviceGroupProperties, core::VkPhysicalDeviceGroupProperties);

impl_Wrapper!(DeviceGroupDeviceCreateInfo, core::VkDeviceGroupDeviceCreateInfo);

impl_Wrapper!(BufferMemoryRequirementsInfo2, core::VkBufferMemoryRequirementsInfo2);

impl_Wrapper!(ImageMemoryRequirementsInfo2, core::VkImageMemoryRequirementsInfo2);

impl_Wrapper!(ImageSparseMemoryRequirementsInfo2, core::VkImageSparseMemoryRequirementsInfo2);

impl_Wrapper!(MemoryRequirements2, core::VkMemoryRequirements2);

impl_Wrapper!(SparseImageMemoryRequirements2, core::VkSparseImageMemoryRequirements2);

impl_Wrapper!(PhysicalDeviceFeatures2, core::VkPhysicalDeviceFeatures2);

impl_Wrapper!(PhysicalDeviceProperties2, core::VkPhysicalDeviceProperties2);

impl_Wrapper!(FormatProperties2, core::VkFormatProperties2);

impl_Wrapper!(ImageFormatProperties2, core::VkImageFormatProperties2);

impl_Wrapper!(PhysicalDeviceImageFormatInfo2, core::VkPhysicalDeviceImageFormatInfo2);

impl_Wrapper!(QueueFamilyProperties2, core::VkQueueFamilyProperties2);

impl_Wrapper!(PhysicalDeviceMemoryProperties2, core::VkPhysicalDeviceMemoryProperties2);

impl_Wrapper!(SparseImageFormatProperties2, core::VkSparseImageFormatProperties2);

impl_Wrapper!(PhysicalDeviceSparseImageFormatInfo2, core::VkPhysicalDeviceSparseImageFormatInfo2);

impl_Wrapper!(PhysicalDevicePointClippingProperties, core::VkPhysicalDevicePointClippingProperties);

impl_Wrapper!(InputAttachmentAspectReference, core::VkInputAttachmentAspectReference);

impl_Wrapper!(RenderPassInputAttachmentAspectCreateInfo, core::VkRenderPassInputAttachmentAspectCreateInfo);

impl_Wrapper!(ImageViewUsageCreateInfo, core::VkImageViewUsageCreateInfo);

impl_Wrapper!(PipelineTessellationDomainOriginStateCreateInfo, core::VkPipelineTessellationDomainOriginStateCreateInfo);

impl_Wrapper!(RenderPassMultiviewCreateInfo, core::VkRenderPassMultiviewCreateInfo);

impl_Wrapper!(PhysicalDeviceMultiviewFeatures, core::VkPhysicalDeviceMultiviewFeatures);

impl_Wrapper!(PhysicalDeviceMultiviewProperties, core::VkPhysicalDeviceMultiviewProperties);

impl_Wrapper!(PhysicalDeviceVariablePointersFeatures, core::VkPhysicalDeviceVariablePointersFeatures);

impl_Wrapper!(PhysicalDeviceProtectedMemoryFeatures, core::VkPhysicalDeviceProtectedMemoryFeatures);

impl_Wrapper!(PhysicalDeviceProtectedMemoryProperties, core::VkPhysicalDeviceProtectedMemoryProperties);

impl_Wrapper!(DeviceQueueInfo2, core::VkDeviceQueueInfo2);

impl_Wrapper!(ProtectedSubmitInfo, core::VkProtectedSubmitInfo);

impl_Wrapper!(SamplerYcbcrConversionCreateInfo, core::VkSamplerYcbcrConversionCreateInfo);

impl_Wrapper!(SamplerYcbcrConversionInfo, core::VkSamplerYcbcrConversionInfo);

impl_Wrapper!(BindImagePlaneMemoryInfo, core::VkBindImagePlaneMemoryInfo);

impl_Wrapper!(ImagePlaneMemoryRequirementsInfo, core::VkImagePlaneMemoryRequirementsInfo);

impl_Wrapper!(PhysicalDeviceSamplerYcbcrConversionFeatures, core::VkPhysicalDeviceSamplerYcbcrConversionFeatures);

impl_Wrapper!(SamplerYcbcrConversionImageFormatProperties, core::VkSamplerYcbcrConversionImageFormatProperties);

impl_Wrapper!(DescriptorUpdateTemplateEntry, core::VkDescriptorUpdateTemplateEntry);

impl_Wrapper!(DescriptorUpdateTemplateCreateInfo, core::VkDescriptorUpdateTemplateCreateInfo);

impl_Wrapper!(ExternalMemoryProperties, core::VkExternalMemoryProperties);

impl_Wrapper!(PhysicalDeviceExternalImageFormatInfo, core::VkPhysicalDeviceExternalImageFormatInfo);

impl_Wrapper!(ExternalImageFormatProperties, core::VkExternalImageFormatProperties);

impl_Wrapper!(PhysicalDeviceExternalBufferInfo, core::VkPhysicalDeviceExternalBufferInfo);

impl_Wrapper!(ExternalBufferProperties, core::VkExternalBufferProperties);

impl_Wrapper!(PhysicalDeviceIDProperties, core::VkPhysicalDeviceIDProperties);

impl_Wrapper!(ExternalMemoryImageCreateInfo, core::VkExternalMemoryImageCreateInfo);

impl_Wrapper!(ExternalMemoryBufferCreateInfo, core::VkExternalMemoryBufferCreateInfo);

impl_Wrapper!(ExportMemoryAllocateInfo, core::VkExportMemoryAllocateInfo);

impl_Wrapper!(PhysicalDeviceExternalFenceInfo, core::VkPhysicalDeviceExternalFenceInfo);

impl_Wrapper!(ExternalFenceProperties, core::VkExternalFenceProperties);

impl_Wrapper!(ExportFenceCreateInfo, core::VkExportFenceCreateInfo);

impl_Wrapper!(ExportSemaphoreCreateInfo, core::VkExportSemaphoreCreateInfo);

impl_Wrapper!(PhysicalDeviceExternalSemaphoreInfo, core::VkPhysicalDeviceExternalSemaphoreInfo);

impl_Wrapper!(ExternalSemaphoreProperties, core::VkExternalSemaphoreProperties);

impl_Wrapper!(PhysicalDeviceMaintenance3Properties, core::VkPhysicalDeviceMaintenance3Properties);

impl_Wrapper!(DescriptorSetLayoutSupport, core::VkDescriptorSetLayoutSupport);

impl_Wrapper!(PhysicalDeviceShaderDrawParametersFeatures, core::VkPhysicalDeviceShaderDrawParametersFeatures);

impl_Wrapper!(PhysicalDeviceVulkan11Features, core::VkPhysicalDeviceVulkan11Features);

impl_Wrapper!(PhysicalDeviceVulkan11Properties, core::VkPhysicalDeviceVulkan11Properties);

impl_Wrapper!(PhysicalDeviceVulkan12Features, core::VkPhysicalDeviceVulkan12Features);

impl_Wrapper!(ConformanceVersion, core::VkConformanceVersion);

impl_Wrapper!(PhysicalDeviceVulkan12Properties, core::VkPhysicalDeviceVulkan12Properties);

impl_Wrapper!(ImageFormatListCreateInfo, core::VkImageFormatListCreateInfo);

impl_Wrapper!(AttachmentDescription2, core::VkAttachmentDescription2);

impl_Wrapper!(AttachmentReference2, core::VkAttachmentReference2);

impl_Wrapper!(SubpassDescription2, core::VkSubpassDescription2);

impl_Wrapper!(SubpassDependency2, core::VkSubpassDependency2);

impl_Wrapper!(RenderPassCreateInfo2, core::VkRenderPassCreateInfo2);

impl_Wrapper!(SubpassBeginInfo, core::VkSubpassBeginInfo);

impl_Wrapper!(SubpassEndInfo, core::VkSubpassEndInfo);

impl_Wrapper!(PhysicalDevice8BitStorageFeatures, core::VkPhysicalDevice8BitStorageFeatures);

impl_Wrapper!(PhysicalDeviceDriverProperties, core::VkPhysicalDeviceDriverProperties);

impl_Wrapper!(PhysicalDeviceShaderAtomicInt64Features, core::VkPhysicalDeviceShaderAtomicInt64Features);

impl_Wrapper!(PhysicalDeviceShaderFloat16Int8Features, core::VkPhysicalDeviceShaderFloat16Int8Features);

impl_Wrapper!(PhysicalDeviceFloatControlsProperties, core::VkPhysicalDeviceFloatControlsProperties);

impl_Wrapper!(DescriptorSetLayoutBindingFlagsCreateInfo, core::VkDescriptorSetLayoutBindingFlagsCreateInfo);

impl_Wrapper!(PhysicalDeviceDescriptorIndexingFeatures, core::VkPhysicalDeviceDescriptorIndexingFeatures);

impl_Wrapper!(PhysicalDeviceDescriptorIndexingProperties, core::VkPhysicalDeviceDescriptorIndexingProperties);

impl_Wrapper!(DescriptorSetVariableDescriptorCountAllocateInfo, core::VkDescriptorSetVariableDescriptorCountAllocateInfo);

impl_Wrapper!(DescriptorSetVariableDescriptorCountLayoutSupport, core::VkDescriptorSetVariableDescriptorCountLayoutSupport);

impl_Wrapper!(SubpassDescriptionDepthStencilResolve, core::VkSubpassDescriptionDepthStencilResolve);

impl_Wrapper!(PhysicalDeviceDepthStencilResolveProperties, core::VkPhysicalDeviceDepthStencilResolveProperties);

impl_Wrapper!(PhysicalDeviceScalarBlockLayoutFeatures, core::VkPhysicalDeviceScalarBlockLayoutFeatures);

impl_Wrapper!(ImageStencilUsageCreateInfo, core::VkImageStencilUsageCreateInfo);

impl_Wrapper!(SamplerReductionModeCreateInfo, core::VkSamplerReductionModeCreateInfo);

impl_Wrapper!(PhysicalDeviceSamplerFilterMinmaxProperties, core::VkPhysicalDeviceSamplerFilterMinmaxProperties);

impl_Wrapper!(PhysicalDeviceVulkanMemoryModelFeatures, core::VkPhysicalDeviceVulkanMemoryModelFeatures);

impl_Wrapper!(PhysicalDeviceImagelessFramebufferFeatures, core::VkPhysicalDeviceImagelessFramebufferFeatures);

impl_Wrapper!(FramebufferAttachmentImageInfo, core::VkFramebufferAttachmentImageInfo);

impl_Wrapper!(FramebufferAttachmentsCreateInfo, core::VkFramebufferAttachmentsCreateInfo);

impl_Wrapper!(RenderPassAttachmentBeginInfo, core::VkRenderPassAttachmentBeginInfo);

impl_Wrapper!(PhysicalDeviceUniformBufferStandardLayoutFeatures, core::VkPhysicalDeviceUniformBufferStandardLayoutFeatures);

impl_Wrapper!(PhysicalDeviceShaderSubgroupExtendedTypesFeatures, core::VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures);

impl_Wrapper!(PhysicalDeviceSeparateDepthStencilLayoutsFeatures, core::VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures);

impl_Wrapper!(AttachmentReferenceStencilLayout, core::VkAttachmentReferenceStencilLayout);

impl_Wrapper!(AttachmentDescriptionStencilLayout, core::VkAttachmentDescriptionStencilLayout);

impl_Wrapper!(PhysicalDeviceHostQueryResetFeatures, core::VkPhysicalDeviceHostQueryResetFeatures);

impl_Wrapper!(PhysicalDeviceTimelineSemaphoreFeatures, core::VkPhysicalDeviceTimelineSemaphoreFeatures);

impl_Wrapper!(PhysicalDeviceTimelineSemaphoreProperties, core::VkPhysicalDeviceTimelineSemaphoreProperties);

impl_Wrapper!(SemaphoreTypeCreateInfo, core::VkSemaphoreTypeCreateInfo);

impl_Wrapper!(TimelineSemaphoreSubmitInfo, core::VkTimelineSemaphoreSubmitInfo);

impl_Wrapper!(SemaphoreWaitInfo, core::VkSemaphoreWaitInfo);

impl_Wrapper!(SemaphoreSignalInfo, core::VkSemaphoreSignalInfo);

impl_Wrapper!(PhysicalDeviceBufferDeviceAddressFeatures, core::VkPhysicalDeviceBufferDeviceAddressFeatures);

impl_Wrapper!(BufferDeviceAddressInfo, core::VkBufferDeviceAddressInfo);

impl_Wrapper!(BufferOpaqueCaptureAddressCreateInfo, core::VkBufferOpaqueCaptureAddressCreateInfo);

impl_Wrapper!(MemoryOpaqueCaptureAddressAllocateInfo, core::VkMemoryOpaqueCaptureAddressAllocateInfo);

impl_Wrapper!(DeviceMemoryOpaqueCaptureAddressInfo, core::VkDeviceMemoryOpaqueCaptureAddressInfo);

impl_Wrapper!(PhysicalDeviceVulkan13Features, core::VkPhysicalDeviceVulkan13Features);

impl_Wrapper!(PhysicalDeviceVulkan13Properties, core::VkPhysicalDeviceVulkan13Properties);

impl_Wrapper!(PipelineCreationFeedback, core::VkPipelineCreationFeedback);

impl_Wrapper!(PipelineCreationFeedbackCreateInfo, core::VkPipelineCreationFeedbackCreateInfo);

impl_Wrapper!(PhysicalDeviceShaderTerminateInvocationFeatures, core::VkPhysicalDeviceShaderTerminateInvocationFeatures);

impl_Wrapper!(PhysicalDeviceToolProperties, core::VkPhysicalDeviceToolProperties);

impl_Wrapper!(PhysicalDeviceShaderDemoteToHelperInvocationFeatures, core::VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures);

impl_Wrapper!(PhysicalDevicePrivateDataFeatures, core::VkPhysicalDevicePrivateDataFeatures);

impl_Wrapper!(DevicePrivateDataCreateInfo, core::VkDevicePrivateDataCreateInfo);

impl_Wrapper!(PrivateDataSlotCreateInfo, core::VkPrivateDataSlotCreateInfo);

impl_Wrapper!(PhysicalDevicePipelineCreationCacheControlFeatures, core::VkPhysicalDevicePipelineCreationCacheControlFeatures);

impl_Wrapper!(MemoryBarrier2, core::VkMemoryBarrier2);

impl_Wrapper!(BufferMemoryBarrier2, core::VkBufferMemoryBarrier2);

impl_Wrapper!(ImageMemoryBarrier2, core::VkImageMemoryBarrier2);

impl_Wrapper!(DependencyInfo, core::VkDependencyInfo);

impl_Wrapper!(SemaphoreSubmitInfo, core::VkSemaphoreSubmitInfo);

impl_Wrapper!(CommandBufferSubmitInfo, core::VkCommandBufferSubmitInfo);

impl_Wrapper!(SubmitInfo2, core::VkSubmitInfo2);

impl_Wrapper!(PhysicalDeviceSynchronization2Features, core::VkPhysicalDeviceSynchronization2Features);

impl_Wrapper!(PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures, core::VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures);

impl_Wrapper!(PhysicalDeviceImageRobustnessFeatures, core::VkPhysicalDeviceImageRobustnessFeatures);

impl_Wrapper!(BufferCopy2, core::VkBufferCopy2);

impl_Wrapper!(CopyBufferInfo2, core::VkCopyBufferInfo2);

impl_Wrapper!(ImageCopy2, core::VkImageCopy2);

impl_Wrapper!(CopyImageInfo2, core::VkCopyImageInfo2);

impl_Wrapper!(BufferImageCopy2, core::VkBufferImageCopy2);

impl_Wrapper!(CopyBufferToImageInfo2, core::VkCopyBufferToImageInfo2);

impl_Wrapper!(CopyImageToBufferInfo2, core::VkCopyImageToBufferInfo2);

impl_Wrapper!(ImageBlit2, core::VkImageBlit2);

impl_Wrapper!(BlitImageInfo2, core::VkBlitImageInfo2);

impl_Wrapper!(ImageResolve2, core::VkImageResolve2);

impl_Wrapper!(ResolveImageInfo2, core::VkResolveImageInfo2);

impl_Wrapper!(PhysicalDeviceSubgroupSizeControlFeatures, core::VkPhysicalDeviceSubgroupSizeControlFeatures);

impl_Wrapper!(PhysicalDeviceSubgroupSizeControlProperties, core::VkPhysicalDeviceSubgroupSizeControlProperties);

impl_Wrapper!(PipelineShaderStageRequiredSubgroupSizeCreateInfo, core::VkPipelineShaderStageRequiredSubgroupSizeCreateInfo);

impl_Wrapper!(PhysicalDeviceInlineUniformBlockFeatures, core::VkPhysicalDeviceInlineUniformBlockFeatures);

impl_Wrapper!(PhysicalDeviceInlineUniformBlockProperties, core::VkPhysicalDeviceInlineUniformBlockProperties);

impl_Wrapper!(WriteDescriptorSetInlineUniformBlock, core::VkWriteDescriptorSetInlineUniformBlock);

impl_Wrapper!(DescriptorPoolInlineUniformBlockCreateInfo, core::VkDescriptorPoolInlineUniformBlockCreateInfo);

impl_Wrapper!(PhysicalDeviceTextureCompressionASTCHDRFeatures, core::VkPhysicalDeviceTextureCompressionASTCHDRFeatures);

impl_Wrapper!(RenderingAttachmentInfo, core::VkRenderingAttachmentInfo);

impl_Wrapper!(RenderingInfo, core::VkRenderingInfo);

impl_Wrapper!(PipelineRenderingCreateInfo, core::VkPipelineRenderingCreateInfo);

impl_Wrapper!(PhysicalDeviceDynamicRenderingFeatures, core::VkPhysicalDeviceDynamicRenderingFeatures);

impl_Wrapper!(CommandBufferInheritanceRenderingInfo, core::VkCommandBufferInheritanceRenderingInfo);

impl_Wrapper!(PhysicalDeviceShaderIntegerDotProductFeatures, core::VkPhysicalDeviceShaderIntegerDotProductFeatures);

impl_Wrapper!(PhysicalDeviceShaderIntegerDotProductProperties, core::VkPhysicalDeviceShaderIntegerDotProductProperties);

impl_Wrapper!(PhysicalDeviceTexelBufferAlignmentProperties, core::VkPhysicalDeviceTexelBufferAlignmentProperties);

impl_Wrapper!(FormatProperties3, core::VkFormatProperties3);

impl_Wrapper!(PhysicalDeviceMaintenance4Features, core::VkPhysicalDeviceMaintenance4Features);

impl_Wrapper!(PhysicalDeviceMaintenance4Properties, core::VkPhysicalDeviceMaintenance4Properties);

impl_Wrapper!(DeviceBufferMemoryRequirements, core::VkDeviceBufferMemoryRequirements);

impl_Wrapper!(DeviceImageMemoryRequirements, core::VkDeviceImageMemoryRequirements);

impl_Wrapper!(SurfaceCapabilitiesKHR, core::VkSurfaceCapabilitiesKHR);

impl_Wrapper!(SurfaceFormatKHR, core::VkSurfaceFormatKHR);

impl_Wrapper!(SwapchainCreateInfoKHR, core::VkSwapchainCreateInfoKHR);

impl_Wrapper!(PresentInfoKHR, core::VkPresentInfoKHR);

impl_Wrapper!(ImageSwapchainCreateInfoKHR, core::VkImageSwapchainCreateInfoKHR);

impl_Wrapper!(BindImageMemorySwapchainInfoKHR, core::VkBindImageMemorySwapchainInfoKHR);

impl_Wrapper!(AcquireNextImageInfoKHR, core::VkAcquireNextImageInfoKHR);

impl_Wrapper!(DeviceGroupPresentCapabilitiesKHR, core::VkDeviceGroupPresentCapabilitiesKHR);

impl_Wrapper!(DeviceGroupPresentInfoKHR, core::VkDeviceGroupPresentInfoKHR);

impl_Wrapper!(DeviceGroupSwapchainCreateInfoKHR, core::VkDeviceGroupSwapchainCreateInfoKHR);

impl_Wrapper!(DisplayModeParametersKHR, core::VkDisplayModeParametersKHR);

impl_Wrapper!(DisplayModeCreateInfoKHR, core::VkDisplayModeCreateInfoKHR);

impl_Wrapper!(DisplayModePropertiesKHR, core::VkDisplayModePropertiesKHR);

impl_Wrapper!(DisplayPlaneCapabilitiesKHR, core::VkDisplayPlaneCapabilitiesKHR);

impl_Wrapper!(DisplayPlanePropertiesKHR, core::VkDisplayPlanePropertiesKHR);

impl_Wrapper!(DisplayPropertiesKHR, core::VkDisplayPropertiesKHR);

impl_Wrapper!(DisplaySurfaceCreateInfoKHR, core::VkDisplaySurfaceCreateInfoKHR);

impl_Wrapper!(DisplayPresentInfoKHR, core::VkDisplayPresentInfoKHR);

impl_Wrapper!(RenderingFragmentShadingRateAttachmentInfoKHR, core::VkRenderingFragmentShadingRateAttachmentInfoKHR);

impl_Wrapper!(RenderingFragmentDensityMapAttachmentInfoEXT, core::VkRenderingFragmentDensityMapAttachmentInfoEXT);

impl_Wrapper!(AttachmentSampleCountInfoAMD, core::VkAttachmentSampleCountInfoAMD);

impl_Wrapper!(MultiviewPerViewAttributesInfoNVX, core::VkMultiviewPerViewAttributesInfoNVX);

impl_Wrapper!(ImportMemoryFdInfoKHR, core::VkImportMemoryFdInfoKHR);

impl_Wrapper!(MemoryFdPropertiesKHR, core::VkMemoryFdPropertiesKHR);

impl_Wrapper!(MemoryGetFdInfoKHR, core::VkMemoryGetFdInfoKHR);

impl_Wrapper!(ImportSemaphoreFdInfoKHR, core::VkImportSemaphoreFdInfoKHR);

impl_Wrapper!(SemaphoreGetFdInfoKHR, core::VkSemaphoreGetFdInfoKHR);

impl_Wrapper!(PhysicalDevicePushDescriptorPropertiesKHR, core::VkPhysicalDevicePushDescriptorPropertiesKHR);

impl_Wrapper!(RectLayerKHR, core::VkRectLayerKHR);

impl_Wrapper!(PresentRegionKHR, core::VkPresentRegionKHR);

impl_Wrapper!(PresentRegionsKHR, core::VkPresentRegionsKHR);

impl_Wrapper!(SharedPresentSurfaceCapabilitiesKHR, core::VkSharedPresentSurfaceCapabilitiesKHR);

impl_Wrapper!(ImportFenceFdInfoKHR, core::VkImportFenceFdInfoKHR);

impl_Wrapper!(FenceGetFdInfoKHR, core::VkFenceGetFdInfoKHR);

impl_Wrapper!(PhysicalDevicePerformanceQueryFeaturesKHR, core::VkPhysicalDevicePerformanceQueryFeaturesKHR);

impl_Wrapper!(PhysicalDevicePerformanceQueryPropertiesKHR, core::VkPhysicalDevicePerformanceQueryPropertiesKHR);

impl_Wrapper!(PerformanceCounterKHR, core::VkPerformanceCounterKHR);

impl_Wrapper!(PerformanceCounterDescriptionKHR, core::VkPerformanceCounterDescriptionKHR);

impl_Wrapper!(QueryPoolPerformanceCreateInfoKHR, core::VkQueryPoolPerformanceCreateInfoKHR);

impl_Wrapper!(AcquireProfilingLockInfoKHR, core::VkAcquireProfilingLockInfoKHR);

impl_Wrapper!(PerformanceQuerySubmitInfoKHR, core::VkPerformanceQuerySubmitInfoKHR);

impl_Wrapper!(PhysicalDeviceSurfaceInfo2KHR, core::VkPhysicalDeviceSurfaceInfo2KHR);

impl_Wrapper!(SurfaceCapabilities2KHR, core::VkSurfaceCapabilities2KHR);

impl_Wrapper!(SurfaceFormat2KHR, core::VkSurfaceFormat2KHR);

impl_Wrapper!(DisplayProperties2KHR, core::VkDisplayProperties2KHR);

impl_Wrapper!(DisplayPlaneProperties2KHR, core::VkDisplayPlaneProperties2KHR);

impl_Wrapper!(DisplayModeProperties2KHR, core::VkDisplayModeProperties2KHR);

impl_Wrapper!(DisplayPlaneInfo2KHR, core::VkDisplayPlaneInfo2KHR);

impl_Wrapper!(DisplayPlaneCapabilities2KHR, core::VkDisplayPlaneCapabilities2KHR);

impl_Wrapper!(PhysicalDeviceShaderClockFeaturesKHR, core::VkPhysicalDeviceShaderClockFeaturesKHR);

impl_Wrapper!(DeviceQueueGlobalPriorityCreateInfoKHR, core::VkDeviceQueueGlobalPriorityCreateInfoKHR);

impl_Wrapper!(PhysicalDeviceGlobalPriorityQueryFeaturesKHR, core::VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR);

impl_Wrapper!(QueueFamilyGlobalPriorityPropertiesKHR, core::VkQueueFamilyGlobalPriorityPropertiesKHR);

impl_Wrapper!(FragmentShadingRateAttachmentInfoKHR, core::VkFragmentShadingRateAttachmentInfoKHR);

impl_Wrapper!(PipelineFragmentShadingRateStateCreateInfoKHR, core::VkPipelineFragmentShadingRateStateCreateInfoKHR);

impl_Wrapper!(PhysicalDeviceFragmentShadingRateFeaturesKHR, core::VkPhysicalDeviceFragmentShadingRateFeaturesKHR);

impl_Wrapper!(PhysicalDeviceFragmentShadingRatePropertiesKHR, core::VkPhysicalDeviceFragmentShadingRatePropertiesKHR);

impl_Wrapper!(PhysicalDeviceFragmentShadingRateKHR, core::VkPhysicalDeviceFragmentShadingRateKHR);

impl_Wrapper!(SurfaceProtectedCapabilitiesKHR, core::VkSurfaceProtectedCapabilitiesKHR);

impl_Wrapper!(PhysicalDevicePresentWaitFeaturesKHR, core::VkPhysicalDevicePresentWaitFeaturesKHR);

impl_Wrapper!(PhysicalDevicePipelineExecutablePropertiesFeaturesKHR, core::VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR);

impl_Wrapper!(PipelineInfoKHR, core::VkPipelineInfoKHR);

impl_Wrapper!(PipelineExecutablePropertiesKHR, core::VkPipelineExecutablePropertiesKHR);

impl_Wrapper!(PipelineExecutableInfoKHR, core::VkPipelineExecutableInfoKHR);

impl_Wrapper!(PipelineExecutableStatisticKHR, core::VkPipelineExecutableStatisticKHR);

impl_Wrapper!(PipelineExecutableInternalRepresentationKHR, core::VkPipelineExecutableInternalRepresentationKHR);

impl_Wrapper!(PipelineLibraryCreateInfoKHR, core::VkPipelineLibraryCreateInfoKHR);

impl_Wrapper!(PresentIdKHR, core::VkPresentIdKHR);

impl_Wrapper!(PhysicalDevicePresentIdFeaturesKHR, core::VkPhysicalDevicePresentIdFeaturesKHR);

impl_Wrapper!(QueueFamilyCheckpointProperties2NV, core::VkQueueFamilyCheckpointProperties2NV);

impl_Wrapper!(CheckpointData2NV, core::VkCheckpointData2NV);

impl_Wrapper!(PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR, core::VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR);

impl_Wrapper!(PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR, core::VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR);

impl_Wrapper!(PipelineRasterizationStateRasterizationOrderAMD, core::VkPipelineRasterizationStateRasterizationOrderAMD);

impl_Wrapper!(DebugMarkerObjectNameInfoEXT, core::VkDebugMarkerObjectNameInfoEXT);

impl_Wrapper!(DebugMarkerObjectTagInfoEXT, core::VkDebugMarkerObjectTagInfoEXT);

impl_Wrapper!(DebugMarkerMarkerInfoEXT, core::VkDebugMarkerMarkerInfoEXT);

impl_Wrapper!(DedicatedAllocationImageCreateInfoNV, core::VkDedicatedAllocationImageCreateInfoNV);

impl_Wrapper!(DedicatedAllocationBufferCreateInfoNV, core::VkDedicatedAllocationBufferCreateInfoNV);

impl_Wrapper!(DedicatedAllocationMemoryAllocateInfoNV, core::VkDedicatedAllocationMemoryAllocateInfoNV);

impl_Wrapper!(PhysicalDeviceTransformFeedbackFeaturesEXT, core::VkPhysicalDeviceTransformFeedbackFeaturesEXT);

impl_Wrapper!(PhysicalDeviceTransformFeedbackPropertiesEXT, core::VkPhysicalDeviceTransformFeedbackPropertiesEXT);

impl_Wrapper!(PipelineRasterizationStateStreamCreateInfoEXT, core::VkPipelineRasterizationStateStreamCreateInfoEXT);

impl_Wrapper!(CuModuleCreateInfoNVX, core::VkCuModuleCreateInfoNVX);

impl_Wrapper!(CuFunctionCreateInfoNVX, core::VkCuFunctionCreateInfoNVX);

impl_Wrapper!(CuLaunchInfoNVX, core::VkCuLaunchInfoNVX);

impl_Wrapper!(ImageViewHandleInfoNVX, core::VkImageViewHandleInfoNVX);

impl_Wrapper!(ImageViewAddressPropertiesNVX, core::VkImageViewAddressPropertiesNVX);

impl_Wrapper!(TextureLODGatherFormatPropertiesAMD, core::VkTextureLODGatherFormatPropertiesAMD);

impl_Wrapper!(ShaderResourceUsageAMD, core::VkShaderResourceUsageAMD);

impl_Wrapper!(ShaderStatisticsInfoAMD, core::VkShaderStatisticsInfoAMD);

impl_Wrapper!(PhysicalDeviceCornerSampledImageFeaturesNV, core::VkPhysicalDeviceCornerSampledImageFeaturesNV);

impl_Wrapper!(ExternalImageFormatPropertiesNV, core::VkExternalImageFormatPropertiesNV);

impl_Wrapper!(ExternalMemoryImageCreateInfoNV, core::VkExternalMemoryImageCreateInfoNV);

impl_Wrapper!(ExportMemoryAllocateInfoNV, core::VkExportMemoryAllocateInfoNV);

impl_Wrapper!(ValidationFlagsEXT, core::VkValidationFlagsEXT);

impl_Wrapper!(ImageViewASTCDecodeModeEXT, core::VkImageViewASTCDecodeModeEXT);

impl_Wrapper!(PhysicalDeviceASTCDecodeFeaturesEXT, core::VkPhysicalDeviceASTCDecodeFeaturesEXT);

impl_Wrapper!(ConditionalRenderingBeginInfoEXT, core::VkConditionalRenderingBeginInfoEXT);

impl_Wrapper!(PhysicalDeviceConditionalRenderingFeaturesEXT, core::VkPhysicalDeviceConditionalRenderingFeaturesEXT);

impl_Wrapper!(CommandBufferInheritanceConditionalRenderingInfoEXT, core::VkCommandBufferInheritanceConditionalRenderingInfoEXT);

impl_Wrapper!(ViewportWScalingNV, core::VkViewportWScalingNV);

impl_Wrapper!(PipelineViewportWScalingStateCreateInfoNV, core::VkPipelineViewportWScalingStateCreateInfoNV);

impl_Wrapper!(SurfaceCapabilities2EXT, core::VkSurfaceCapabilities2EXT);

impl_Wrapper!(DisplayPowerInfoEXT, core::VkDisplayPowerInfoEXT);

impl_Wrapper!(DeviceEventInfoEXT, core::VkDeviceEventInfoEXT);

impl_Wrapper!(DisplayEventInfoEXT, core::VkDisplayEventInfoEXT);

impl_Wrapper!(SwapchainCounterCreateInfoEXT, core::VkSwapchainCounterCreateInfoEXT);

impl_Wrapper!(RefreshCycleDurationGOOGLE, core::VkRefreshCycleDurationGOOGLE);

impl_Wrapper!(PastPresentationTimingGOOGLE, core::VkPastPresentationTimingGOOGLE);

impl_Wrapper!(PresentTimeGOOGLE, core::VkPresentTimeGOOGLE);

impl_Wrapper!(PresentTimesInfoGOOGLE, core::VkPresentTimesInfoGOOGLE);

impl_Wrapper!(PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX, core::VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX);

impl_Wrapper!(ViewportSwizzleNV, core::VkViewportSwizzleNV);

impl_Wrapper!(PipelineViewportSwizzleStateCreateInfoNV, core::VkPipelineViewportSwizzleStateCreateInfoNV);

impl_Wrapper!(PhysicalDeviceDiscardRectanglePropertiesEXT, core::VkPhysicalDeviceDiscardRectanglePropertiesEXT);

impl_Wrapper!(PipelineDiscardRectangleStateCreateInfoEXT, core::VkPipelineDiscardRectangleStateCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceConservativeRasterizationPropertiesEXT, core::VkPhysicalDeviceConservativeRasterizationPropertiesEXT);

impl_Wrapper!(PipelineRasterizationConservativeStateCreateInfoEXT, core::VkPipelineRasterizationConservativeStateCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceDepthClipEnableFeaturesEXT, core::VkPhysicalDeviceDepthClipEnableFeaturesEXT);

impl_Wrapper!(PipelineRasterizationDepthClipStateCreateInfoEXT, core::VkPipelineRasterizationDepthClipStateCreateInfoEXT);

impl_Wrapper!(XYColorEXT, core::VkXYColorEXT);

impl_Wrapper!(HdrMetadataEXT, core::VkHdrMetadataEXT);

impl_Wrapper!(DebugUtilsLabelEXT, core::VkDebugUtilsLabelEXT);

impl_Wrapper!(DebugUtilsObjectNameInfoEXT, core::VkDebugUtilsObjectNameInfoEXT);

impl_Wrapper!(DebugUtilsMessengerCallbackDataEXT, core::VkDebugUtilsMessengerCallbackDataEXT);

impl_Wrapper!(DebugUtilsObjectTagInfoEXT, core::VkDebugUtilsObjectTagInfoEXT);

impl_Wrapper!(SampleLocationEXT, core::VkSampleLocationEXT);

impl_Wrapper!(SampleLocationsInfoEXT, core::VkSampleLocationsInfoEXT);

impl_Wrapper!(AttachmentSampleLocationsEXT, core::VkAttachmentSampleLocationsEXT);

impl_Wrapper!(SubpassSampleLocationsEXT, core::VkSubpassSampleLocationsEXT);

impl_Wrapper!(RenderPassSampleLocationsBeginInfoEXT, core::VkRenderPassSampleLocationsBeginInfoEXT);

impl_Wrapper!(PipelineSampleLocationsStateCreateInfoEXT, core::VkPipelineSampleLocationsStateCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceSampleLocationsPropertiesEXT, core::VkPhysicalDeviceSampleLocationsPropertiesEXT);

impl_Wrapper!(MultisamplePropertiesEXT, core::VkMultisamplePropertiesEXT);

impl_Wrapper!(PhysicalDeviceBlendOperationAdvancedFeaturesEXT, core::VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT);

impl_Wrapper!(PhysicalDeviceBlendOperationAdvancedPropertiesEXT, core::VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT);

impl_Wrapper!(PipelineColorBlendAdvancedStateCreateInfoEXT, core::VkPipelineColorBlendAdvancedStateCreateInfoEXT);

impl_Wrapper!(PipelineCoverageToColorStateCreateInfoNV, core::VkPipelineCoverageToColorStateCreateInfoNV);

impl_Wrapper!(PipelineCoverageModulationStateCreateInfoNV, core::VkPipelineCoverageModulationStateCreateInfoNV);

impl_Wrapper!(PhysicalDeviceShaderSMBuiltinsPropertiesNV, core::VkPhysicalDeviceShaderSMBuiltinsPropertiesNV);

impl_Wrapper!(PhysicalDeviceShaderSMBuiltinsFeaturesNV, core::VkPhysicalDeviceShaderSMBuiltinsFeaturesNV);

impl_Wrapper!(DrmFormatModifierPropertiesEXT, core::VkDrmFormatModifierPropertiesEXT);

impl_Wrapper!(DrmFormatModifierPropertiesListEXT, core::VkDrmFormatModifierPropertiesListEXT);

impl_Wrapper!(PhysicalDeviceImageDrmFormatModifierInfoEXT, core::VkPhysicalDeviceImageDrmFormatModifierInfoEXT);

impl_Wrapper!(ImageDrmFormatModifierListCreateInfoEXT, core::VkImageDrmFormatModifierListCreateInfoEXT);

impl_Wrapper!(ImageDrmFormatModifierExplicitCreateInfoEXT, core::VkImageDrmFormatModifierExplicitCreateInfoEXT);

impl_Wrapper!(ImageDrmFormatModifierPropertiesEXT, core::VkImageDrmFormatModifierPropertiesEXT);

impl_Wrapper!(DrmFormatModifierProperties2EXT, core::VkDrmFormatModifierProperties2EXT);

impl_Wrapper!(DrmFormatModifierPropertiesList2EXT, core::VkDrmFormatModifierPropertiesList2EXT);

impl_Wrapper!(ValidationCacheCreateInfoEXT, core::VkValidationCacheCreateInfoEXT);

impl_Wrapper!(ShaderModuleValidationCacheCreateInfoEXT, core::VkShaderModuleValidationCacheCreateInfoEXT);

impl_Wrapper!(ShadingRatePaletteNV, core::VkShadingRatePaletteNV);

impl_Wrapper!(PipelineViewportShadingRateImageStateCreateInfoNV, core::VkPipelineViewportShadingRateImageStateCreateInfoNV);

impl_Wrapper!(PhysicalDeviceShadingRateImageFeaturesNV, core::VkPhysicalDeviceShadingRateImageFeaturesNV);

impl_Wrapper!(PhysicalDeviceShadingRateImagePropertiesNV, core::VkPhysicalDeviceShadingRateImagePropertiesNV);

impl_Wrapper!(CoarseSampleLocationNV, core::VkCoarseSampleLocationNV);

impl_Wrapper!(CoarseSampleOrderCustomNV, core::VkCoarseSampleOrderCustomNV);

impl_Wrapper!(PipelineViewportCoarseSampleOrderStateCreateInfoNV, core::VkPipelineViewportCoarseSampleOrderStateCreateInfoNV);

impl_Wrapper!(RayTracingShaderGroupCreateInfoNV, core::VkRayTracingShaderGroupCreateInfoNV);

impl_Wrapper!(RayTracingPipelineCreateInfoNV, core::VkRayTracingPipelineCreateInfoNV);

impl_Wrapper!(GeometryTrianglesNV, core::VkGeometryTrianglesNV);

impl_Wrapper!(GeometryAABBNV, core::VkGeometryAABBNV);

impl_Wrapper!(GeometryDataNV, core::VkGeometryDataNV);

impl_Wrapper!(GeometryNV, core::VkGeometryNV);

impl_Wrapper!(AccelerationStructureInfoNV, core::VkAccelerationStructureInfoNV);

impl_Wrapper!(AccelerationStructureCreateInfoNV, core::VkAccelerationStructureCreateInfoNV);

impl_Wrapper!(BindAccelerationStructureMemoryInfoNV, core::VkBindAccelerationStructureMemoryInfoNV);

impl_Wrapper!(WriteDescriptorSetAccelerationStructureNV, core::VkWriteDescriptorSetAccelerationStructureNV);

impl_Wrapper!(AccelerationStructureMemoryRequirementsInfoNV, core::VkAccelerationStructureMemoryRequirementsInfoNV);

impl_Wrapper!(PhysicalDeviceRayTracingPropertiesNV, core::VkPhysicalDeviceRayTracingPropertiesNV);

impl_Wrapper!(TransformMatrixKHR, core::VkTransformMatrixKHR);

impl_Wrapper!(AabbPositionsKHR, core::VkAabbPositionsKHR);

impl_Wrapper!(AccelerationStructureInstanceKHR, core::VkAccelerationStructureInstanceKHR);

impl_Wrapper!(PhysicalDeviceRepresentativeFragmentTestFeaturesNV, core::VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV);

impl_Wrapper!(PipelineRepresentativeFragmentTestStateCreateInfoNV, core::VkPipelineRepresentativeFragmentTestStateCreateInfoNV);

impl_Wrapper!(PhysicalDeviceImageViewImageFormatInfoEXT, core::VkPhysicalDeviceImageViewImageFormatInfoEXT);

impl_Wrapper!(FilterCubicImageViewImageFormatPropertiesEXT, core::VkFilterCubicImageViewImageFormatPropertiesEXT);

impl_Wrapper!(ImportMemoryHostPointerInfoEXT, core::VkImportMemoryHostPointerInfoEXT);

impl_Wrapper!(MemoryHostPointerPropertiesEXT, core::VkMemoryHostPointerPropertiesEXT);

impl_Wrapper!(PhysicalDeviceExternalMemoryHostPropertiesEXT, core::VkPhysicalDeviceExternalMemoryHostPropertiesEXT);

impl_Wrapper!(PipelineCompilerControlCreateInfoAMD, core::VkPipelineCompilerControlCreateInfoAMD);

impl_Wrapper!(CalibratedTimestampInfoEXT, core::VkCalibratedTimestampInfoEXT);

impl_Wrapper!(PhysicalDeviceShaderCorePropertiesAMD, core::VkPhysicalDeviceShaderCorePropertiesAMD);

impl_Wrapper!(DeviceMemoryOverallocationCreateInfoAMD, core::VkDeviceMemoryOverallocationCreateInfoAMD);

impl_Wrapper!(PhysicalDeviceVertexAttributeDivisorPropertiesEXT, core::VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT);

impl_Wrapper!(VertexInputBindingDivisorDescriptionEXT, core::VkVertexInputBindingDivisorDescriptionEXT);

impl_Wrapper!(PipelineVertexInputDivisorStateCreateInfoEXT, core::VkPipelineVertexInputDivisorStateCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceVertexAttributeDivisorFeaturesEXT, core::VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT);

impl_Wrapper!(PhysicalDeviceComputeShaderDerivativesFeaturesNV, core::VkPhysicalDeviceComputeShaderDerivativesFeaturesNV);

impl_Wrapper!(PhysicalDeviceMeshShaderFeaturesNV, core::VkPhysicalDeviceMeshShaderFeaturesNV);

impl_Wrapper!(PhysicalDeviceMeshShaderPropertiesNV, core::VkPhysicalDeviceMeshShaderPropertiesNV);

impl_Wrapper!(DrawMeshTasksIndirectCommandNV, core::VkDrawMeshTasksIndirectCommandNV);

impl_Wrapper!(PhysicalDeviceFragmentShaderBarycentricFeaturesNV, core::VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV);

impl_Wrapper!(PhysicalDeviceShaderImageFootprintFeaturesNV, core::VkPhysicalDeviceShaderImageFootprintFeaturesNV);

impl_Wrapper!(PipelineViewportExclusiveScissorStateCreateInfoNV, core::VkPipelineViewportExclusiveScissorStateCreateInfoNV);

impl_Wrapper!(PhysicalDeviceExclusiveScissorFeaturesNV, core::VkPhysicalDeviceExclusiveScissorFeaturesNV);

impl_Wrapper!(QueueFamilyCheckpointPropertiesNV, core::VkQueueFamilyCheckpointPropertiesNV);

impl_Wrapper!(CheckpointDataNV, core::VkCheckpointDataNV);

impl_Wrapper!(PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL, core::VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL);

impl_Wrapper!(PerformanceValueINTEL, core::VkPerformanceValueINTEL);

impl_Wrapper!(InitializePerformanceApiInfoINTEL, core::VkInitializePerformanceApiInfoINTEL);

impl_Wrapper!(QueryPoolPerformanceQueryCreateInfoINTEL, core::VkQueryPoolPerformanceQueryCreateInfoINTEL);

impl_Wrapper!(PerformanceMarkerInfoINTEL, core::VkPerformanceMarkerInfoINTEL);

impl_Wrapper!(PerformanceStreamMarkerInfoINTEL, core::VkPerformanceStreamMarkerInfoINTEL);

impl_Wrapper!(PerformanceOverrideInfoINTEL, core::VkPerformanceOverrideInfoINTEL);

impl_Wrapper!(PerformanceConfigurationAcquireInfoINTEL, core::VkPerformanceConfigurationAcquireInfoINTEL);

impl_Wrapper!(PhysicalDevicePCIBusInfoPropertiesEXT, core::VkPhysicalDevicePCIBusInfoPropertiesEXT);

impl_Wrapper!(DisplayNativeHdrSurfaceCapabilitiesAMD, core::VkDisplayNativeHdrSurfaceCapabilitiesAMD);

impl_Wrapper!(SwapchainDisplayNativeHdrCreateInfoAMD, core::VkSwapchainDisplayNativeHdrCreateInfoAMD);

impl_Wrapper!(PhysicalDeviceFragmentDensityMapFeaturesEXT, core::VkPhysicalDeviceFragmentDensityMapFeaturesEXT);

impl_Wrapper!(PhysicalDeviceFragmentDensityMapPropertiesEXT, core::VkPhysicalDeviceFragmentDensityMapPropertiesEXT);

impl_Wrapper!(RenderPassFragmentDensityMapCreateInfoEXT, core::VkRenderPassFragmentDensityMapCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceShaderCoreProperties2AMD, core::VkPhysicalDeviceShaderCoreProperties2AMD);

impl_Wrapper!(PhysicalDeviceCoherentMemoryFeaturesAMD, core::VkPhysicalDeviceCoherentMemoryFeaturesAMD);

impl_Wrapper!(PhysicalDeviceShaderImageAtomicInt64FeaturesEXT, core::VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT);

impl_Wrapper!(PhysicalDeviceMemoryBudgetPropertiesEXT, core::VkPhysicalDeviceMemoryBudgetPropertiesEXT);

impl_Wrapper!(PhysicalDeviceMemoryPriorityFeaturesEXT, core::VkPhysicalDeviceMemoryPriorityFeaturesEXT);

impl_Wrapper!(MemoryPriorityAllocateInfoEXT, core::VkMemoryPriorityAllocateInfoEXT);

impl_Wrapper!(PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV, core::VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV);

impl_Wrapper!(PhysicalDeviceBufferDeviceAddressFeaturesEXT, core::VkPhysicalDeviceBufferDeviceAddressFeaturesEXT);

impl_Wrapper!(BufferDeviceAddressCreateInfoEXT, core::VkBufferDeviceAddressCreateInfoEXT);

impl_Wrapper!(ValidationFeaturesEXT, core::VkValidationFeaturesEXT);

impl_Wrapper!(CooperativeMatrixPropertiesNV, core::VkCooperativeMatrixPropertiesNV);

impl_Wrapper!(PhysicalDeviceCooperativeMatrixFeaturesNV, core::VkPhysicalDeviceCooperativeMatrixFeaturesNV);

impl_Wrapper!(PhysicalDeviceCooperativeMatrixPropertiesNV, core::VkPhysicalDeviceCooperativeMatrixPropertiesNV);

impl_Wrapper!(PhysicalDeviceCoverageReductionModeFeaturesNV, core::VkPhysicalDeviceCoverageReductionModeFeaturesNV);

impl_Wrapper!(PipelineCoverageReductionStateCreateInfoNV, core::VkPipelineCoverageReductionStateCreateInfoNV);

impl_Wrapper!(FramebufferMixedSamplesCombinationNV, core::VkFramebufferMixedSamplesCombinationNV);

impl_Wrapper!(PhysicalDeviceFragmentShaderInterlockFeaturesEXT, core::VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT);

impl_Wrapper!(PhysicalDeviceYcbcrImageArraysFeaturesEXT, core::VkPhysicalDeviceYcbcrImageArraysFeaturesEXT);

impl_Wrapper!(PhysicalDeviceProvokingVertexFeaturesEXT, core::VkPhysicalDeviceProvokingVertexFeaturesEXT);

impl_Wrapper!(PhysicalDeviceProvokingVertexPropertiesEXT, core::VkPhysicalDeviceProvokingVertexPropertiesEXT);

impl_Wrapper!(PipelineRasterizationProvokingVertexStateCreateInfoEXT, core::VkPipelineRasterizationProvokingVertexStateCreateInfoEXT);

impl_Wrapper!(HeadlessSurfaceCreateInfoEXT, core::VkHeadlessSurfaceCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceLineRasterizationFeaturesEXT, core::VkPhysicalDeviceLineRasterizationFeaturesEXT);

impl_Wrapper!(PhysicalDeviceLineRasterizationPropertiesEXT, core::VkPhysicalDeviceLineRasterizationPropertiesEXT);

impl_Wrapper!(PipelineRasterizationLineStateCreateInfoEXT, core::VkPipelineRasterizationLineStateCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceShaderAtomicFloatFeaturesEXT, core::VkPhysicalDeviceShaderAtomicFloatFeaturesEXT);

impl_Wrapper!(PhysicalDeviceIndexTypeUint8FeaturesEXT, core::VkPhysicalDeviceIndexTypeUint8FeaturesEXT);

impl_Wrapper!(PhysicalDeviceExtendedDynamicStateFeaturesEXT, core::VkPhysicalDeviceExtendedDynamicStateFeaturesEXT);

impl_Wrapper!(PhysicalDeviceShaderAtomicFloat2FeaturesEXT, core::VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT);

impl_Wrapper!(PhysicalDeviceDeviceGeneratedCommandsPropertiesNV, core::VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV);

impl_Wrapper!(PhysicalDeviceDeviceGeneratedCommandsFeaturesNV, core::VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV);

impl_Wrapper!(GraphicsShaderGroupCreateInfoNV, core::VkGraphicsShaderGroupCreateInfoNV);

impl_Wrapper!(GraphicsPipelineShaderGroupsCreateInfoNV, core::VkGraphicsPipelineShaderGroupsCreateInfoNV);

impl_Wrapper!(BindShaderGroupIndirectCommandNV, core::VkBindShaderGroupIndirectCommandNV);

impl_Wrapper!(BindIndexBufferIndirectCommandNV, core::VkBindIndexBufferIndirectCommandNV);

impl_Wrapper!(BindVertexBufferIndirectCommandNV, core::VkBindVertexBufferIndirectCommandNV);

impl_Wrapper!(SetStateFlagsIndirectCommandNV, core::VkSetStateFlagsIndirectCommandNV);

impl_Wrapper!(IndirectCommandsStreamNV, core::VkIndirectCommandsStreamNV);

impl_Wrapper!(IndirectCommandsLayoutTokenNV, core::VkIndirectCommandsLayoutTokenNV);

impl_Wrapper!(IndirectCommandsLayoutCreateInfoNV, core::VkIndirectCommandsLayoutCreateInfoNV);

impl_Wrapper!(GeneratedCommandsInfoNV, core::VkGeneratedCommandsInfoNV);

impl_Wrapper!(GeneratedCommandsMemoryRequirementsInfoNV, core::VkGeneratedCommandsMemoryRequirementsInfoNV);

impl_Wrapper!(PhysicalDeviceInheritedViewportScissorFeaturesNV, core::VkPhysicalDeviceInheritedViewportScissorFeaturesNV);

impl_Wrapper!(CommandBufferInheritanceViewportScissorInfoNV, core::VkCommandBufferInheritanceViewportScissorInfoNV);

impl_Wrapper!(PhysicalDeviceTexelBufferAlignmentFeaturesEXT, core::VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT);

impl_Wrapper!(RenderPassTransformBeginInfoQCOM, core::VkRenderPassTransformBeginInfoQCOM);

impl_Wrapper!(CommandBufferInheritanceRenderPassTransformInfoQCOM, core::VkCommandBufferInheritanceRenderPassTransformInfoQCOM);

impl_Wrapper!(PhysicalDeviceDeviceMemoryReportFeaturesEXT, core::VkPhysicalDeviceDeviceMemoryReportFeaturesEXT);

impl_Wrapper!(DeviceMemoryReportCallbackDataEXT, core::VkDeviceMemoryReportCallbackDataEXT);

impl_Wrapper!(PhysicalDeviceRobustness2FeaturesEXT, core::VkPhysicalDeviceRobustness2FeaturesEXT);

impl_Wrapper!(PhysicalDeviceRobustness2PropertiesEXT, core::VkPhysicalDeviceRobustness2PropertiesEXT);

impl_Wrapper!(SamplerCustomBorderColorCreateInfoEXT, core::VkSamplerCustomBorderColorCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceCustomBorderColorPropertiesEXT, core::VkPhysicalDeviceCustomBorderColorPropertiesEXT);

impl_Wrapper!(PhysicalDeviceCustomBorderColorFeaturesEXT, core::VkPhysicalDeviceCustomBorderColorFeaturesEXT);

impl_Wrapper!(PhysicalDeviceDiagnosticsConfigFeaturesNV, core::VkPhysicalDeviceDiagnosticsConfigFeaturesNV);

impl_Wrapper!(DeviceDiagnosticsConfigCreateInfoNV, core::VkDeviceDiagnosticsConfigCreateInfoNV);

impl_Wrapper!(PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT, core::VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT);

impl_Wrapper!(PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT, core::VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT);

impl_Wrapper!(GraphicsPipelineLibraryCreateInfoEXT, core::VkGraphicsPipelineLibraryCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceFragmentShadingRateEnumsFeaturesNV, core::VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV);

impl_Wrapper!(PhysicalDeviceFragmentShadingRateEnumsPropertiesNV, core::VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV);

impl_Wrapper!(PipelineFragmentShadingRateEnumStateCreateInfoNV, core::VkPipelineFragmentShadingRateEnumStateCreateInfoNV);

impl_Wrapper!(AccelerationStructureGeometryMotionTrianglesDataNV, core::VkAccelerationStructureGeometryMotionTrianglesDataNV);

impl_Wrapper!(AccelerationStructureMotionInfoNV, core::VkAccelerationStructureMotionInfoNV);

impl_Wrapper!(AccelerationStructureMatrixMotionInstanceNV, core::VkAccelerationStructureMatrixMotionInstanceNV);

impl_Wrapper!(SRTDataNV, core::VkSRTDataNV);

impl_Wrapper!(AccelerationStructureSRTMotionInstanceNV, core::VkAccelerationStructureSRTMotionInstanceNV);

impl_Wrapper!(AccelerationStructureMotionInstanceNV, core::VkAccelerationStructureMotionInstanceNV);

impl_Wrapper!(PhysicalDeviceRayTracingMotionBlurFeaturesNV, core::VkPhysicalDeviceRayTracingMotionBlurFeaturesNV);

impl_Wrapper!(PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT, core::VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT);

impl_Wrapper!(PhysicalDeviceFragmentDensityMap2FeaturesEXT, core::VkPhysicalDeviceFragmentDensityMap2FeaturesEXT);

impl_Wrapper!(PhysicalDeviceFragmentDensityMap2PropertiesEXT, core::VkPhysicalDeviceFragmentDensityMap2PropertiesEXT);

impl_Wrapper!(CopyCommandTransformInfoQCOM, core::VkCopyCommandTransformInfoQCOM);

impl_Wrapper!(PhysicalDevice4444FormatsFeaturesEXT, core::VkPhysicalDevice4444FormatsFeaturesEXT);

impl_Wrapper!(PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM, core::VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM);

impl_Wrapper!(PhysicalDeviceRGBA10X6FormatsFeaturesEXT, core::VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT);

impl_Wrapper!(PhysicalDeviceMutableDescriptorTypeFeaturesVALVE, core::VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE);

impl_Wrapper!(MutableDescriptorTypeListVALVE, core::VkMutableDescriptorTypeListVALVE);

impl_Wrapper!(MutableDescriptorTypeCreateInfoVALVE, core::VkMutableDescriptorTypeCreateInfoVALVE);

impl_Wrapper!(PhysicalDeviceVertexInputDynamicStateFeaturesEXT, core::VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT);

impl_Wrapper!(VertexInputBindingDescription2EXT, core::VkVertexInputBindingDescription2EXT);

impl_Wrapper!(VertexInputAttributeDescription2EXT, core::VkVertexInputAttributeDescription2EXT);

impl_Wrapper!(PhysicalDeviceDrmPropertiesEXT, core::VkPhysicalDeviceDrmPropertiesEXT);

impl_Wrapper!(PhysicalDeviceDepthClipControlFeaturesEXT, core::VkPhysicalDeviceDepthClipControlFeaturesEXT);

impl_Wrapper!(PipelineViewportDepthClipControlCreateInfoEXT, core::VkPipelineViewportDepthClipControlCreateInfoEXT);

impl_Wrapper!(PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT, core::VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT);

impl_Wrapper!(SubpassShadingPipelineCreateInfoHUAWEI, core::VkSubpassShadingPipelineCreateInfoHUAWEI);

impl_Wrapper!(PhysicalDeviceSubpassShadingFeaturesHUAWEI, core::VkPhysicalDeviceSubpassShadingFeaturesHUAWEI);

impl_Wrapper!(PhysicalDeviceSubpassShadingPropertiesHUAWEI, core::VkPhysicalDeviceSubpassShadingPropertiesHUAWEI);

impl_Wrapper!(PhysicalDeviceInvocationMaskFeaturesHUAWEI, core::VkPhysicalDeviceInvocationMaskFeaturesHUAWEI);

impl_Wrapper!(MemoryGetRemoteAddressInfoNV, core::VkMemoryGetRemoteAddressInfoNV);

impl_Wrapper!(PhysicalDeviceExternalMemoryRDMAFeaturesNV, core::VkPhysicalDeviceExternalMemoryRDMAFeaturesNV);

impl_Wrapper!(PhysicalDeviceExtendedDynamicState2FeaturesEXT, core::VkPhysicalDeviceExtendedDynamicState2FeaturesEXT);

impl_Wrapper!(PhysicalDeviceColorWriteEnableFeaturesEXT, core::VkPhysicalDeviceColorWriteEnableFeaturesEXT);

impl_Wrapper!(PipelineColorWriteCreateInfoEXT, core::VkPipelineColorWriteCreateInfoEXT);

impl_Wrapper!(PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT, core::VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT);

impl_Wrapper!(PhysicalDeviceImageViewMinLodFeaturesEXT, core::VkPhysicalDeviceImageViewMinLodFeaturesEXT);

impl_Wrapper!(ImageViewMinLodCreateInfoEXT, core::VkImageViewMinLodCreateInfoEXT);

impl_Wrapper!(PhysicalDeviceMultiDrawFeaturesEXT, core::VkPhysicalDeviceMultiDrawFeaturesEXT);

impl_Wrapper!(PhysicalDeviceMultiDrawPropertiesEXT, core::VkPhysicalDeviceMultiDrawPropertiesEXT);

impl_Wrapper!(MultiDrawInfoEXT, core::VkMultiDrawInfoEXT);

impl_Wrapper!(MultiDrawIndexedInfoEXT, core::VkMultiDrawIndexedInfoEXT);

impl_Wrapper!(PhysicalDeviceImage2DViewOf3DFeaturesEXT, core::VkPhysicalDeviceImage2DViewOf3DFeaturesEXT);

impl_Wrapper!(PhysicalDeviceBorderColorSwizzleFeaturesEXT, core::VkPhysicalDeviceBorderColorSwizzleFeaturesEXT);

impl_Wrapper!(SamplerBorderColorComponentMappingCreateInfoEXT, core::VkSamplerBorderColorComponentMappingCreateInfoEXT);

impl_Wrapper!(PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT, core::VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT);

impl_Wrapper!(PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE, core::VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE);

impl_Wrapper!(DescriptorSetBindingReferenceVALVE, core::VkDescriptorSetBindingReferenceVALVE);

impl_Wrapper!(DescriptorSetLayoutHostMappingInfoVALVE, core::VkDescriptorSetLayoutHostMappingInfoVALVE);

impl_Wrapper!(PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM, core::VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM);

impl_Wrapper!(PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM, core::VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM);

impl_Wrapper!(SubpassFragmentDensityMapOffsetEndInfoQCOM, core::VkSubpassFragmentDensityMapOffsetEndInfoQCOM);

impl_Wrapper!(PhysicalDeviceLinearColorAttachmentFeaturesNV, core::VkPhysicalDeviceLinearColorAttachmentFeaturesNV);

impl_Wrapper!(AccelerationStructureBuildRangeInfoKHR, core::VkAccelerationStructureBuildRangeInfoKHR);

impl_Wrapper!(AccelerationStructureGeometryTrianglesDataKHR, core::VkAccelerationStructureGeometryTrianglesDataKHR);

impl_Wrapper!(AccelerationStructureGeometryAabbsDataKHR, core::VkAccelerationStructureGeometryAabbsDataKHR);

impl_Wrapper!(AccelerationStructureGeometryInstancesDataKHR, core::VkAccelerationStructureGeometryInstancesDataKHR);

impl_Wrapper!(AccelerationStructureGeometryKHR, core::VkAccelerationStructureGeometryKHR);

impl_Wrapper!(AccelerationStructureBuildGeometryInfoKHR, core::VkAccelerationStructureBuildGeometryInfoKHR);

impl_Wrapper!(AccelerationStructureCreateInfoKHR, core::VkAccelerationStructureCreateInfoKHR);

impl_Wrapper!(WriteDescriptorSetAccelerationStructureKHR, core::VkWriteDescriptorSetAccelerationStructureKHR);

impl_Wrapper!(PhysicalDeviceAccelerationStructureFeaturesKHR, core::VkPhysicalDeviceAccelerationStructureFeaturesKHR);

impl_Wrapper!(PhysicalDeviceAccelerationStructurePropertiesKHR, core::VkPhysicalDeviceAccelerationStructurePropertiesKHR);

impl_Wrapper!(AccelerationStructureDeviceAddressInfoKHR, core::VkAccelerationStructureDeviceAddressInfoKHR);

impl_Wrapper!(AccelerationStructureVersionInfoKHR, core::VkAccelerationStructureVersionInfoKHR);

impl_Wrapper!(CopyAccelerationStructureToMemoryInfoKHR, core::VkCopyAccelerationStructureToMemoryInfoKHR);

impl_Wrapper!(CopyMemoryToAccelerationStructureInfoKHR, core::VkCopyMemoryToAccelerationStructureInfoKHR);

impl_Wrapper!(CopyAccelerationStructureInfoKHR, core::VkCopyAccelerationStructureInfoKHR);

impl_Wrapper!(AccelerationStructureBuildSizesInfoKHR, core::VkAccelerationStructureBuildSizesInfoKHR);

impl_Wrapper!(RayTracingShaderGroupCreateInfoKHR, core::VkRayTracingShaderGroupCreateInfoKHR);

impl_Wrapper!(RayTracingPipelineInterfaceCreateInfoKHR, core::VkRayTracingPipelineInterfaceCreateInfoKHR);

impl_Wrapper!(RayTracingPipelineCreateInfoKHR, core::VkRayTracingPipelineCreateInfoKHR);

impl_Wrapper!(PhysicalDeviceRayTracingPipelineFeaturesKHR, core::VkPhysicalDeviceRayTracingPipelineFeaturesKHR);

impl_Wrapper!(PhysicalDeviceRayTracingPipelinePropertiesKHR, core::VkPhysicalDeviceRayTracingPipelinePropertiesKHR);

impl_Wrapper!(StridedDeviceAddressRegionKHR, core::VkStridedDeviceAddressRegionKHR);

impl_Wrapper!(TraceRaysIndirectCommandKHR, core::VkTraceRaysIndirectCommandKHR);

impl_Wrapper!(PhysicalDeviceRayQueryFeaturesKHR, core::VkPhysicalDeviceRayQueryFeaturesKHR);

impl_Wrapper!(ClearColorValue, core::VkClearColorValue);

impl_Wrapper!(ClearValue, core::VkClearValue);

impl_Wrapper!(PerformanceCounterResultKHR, core::VkPerformanceCounterResultKHR);

impl_Wrapper!(PipelineExecutableStatisticValueKHR, core::VkPipelineExecutableStatisticValueKHR);

impl_Wrapper!(PerformanceValueDataINTEL, core::VkPerformanceValueDataINTEL);

impl_Wrapper!(DeviceOrHostAddressConstKHR, core::VkDeviceOrHostAddressConstKHR);

impl_Wrapper!(AccelerationStructureMotionInstanceDataNV, core::VkAccelerationStructureMotionInstanceDataNV);

impl_Wrapper!(DeviceOrHostAddressKHR, core::VkDeviceOrHostAddressKHR);

impl_Wrapper!(AccelerationStructureGeometryDataKHR, core::VkAccelerationStructureGeometryDataKHR);

impl_Wrapper!(XcbSurfaceCreateInfoKHR, xcb::VkXcbSurfaceCreateInfoKHR);

impl_Wrapper!(WaylandSurfaceCreateInfoKHR, wayland::VkWaylandSurfaceCreateInfoKHR);

//
// Helper methods:
//

impl XcbSurfaceCreateInfoKHR {
	pub const fn new(connection: *mut x11::xcb_connection_t, window: x11::xproto::xcb_window_t) -> Self {
		Self(xcb::VkXcbSurfaceCreateInfoKHR {
			sType: core::VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR,
			pNext: mem::null(),
			flags: 0,
			connection,
			window,
		})
	}
}

impl WaylandSurfaceCreateInfoKHR {
	pub const fn new(display: *mut wl::wl_display, surface: *mut wl::wl_surface) -> Self {
		Self(wayland::VkWaylandSurfaceCreateInfoKHR {
			sType: core::VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
			pNext: mem::null(),
			flags: 0,
			display: display as _,
			surface: surface as _,
		})
	}
}

impl Extent2D {
	#[inline]
	pub const fn new(width: u32, height: u32) -> Self {
		Self(core::VkExtent2D {
			width,
			height,
		})
	}
}

impl Extent3D {
	#[inline]
	pub const fn new(width: u32, height: u32, depth: u32) -> Self {
		Self(core::VkExtent3D {
			width,
			height,
			depth,
		})
	}
}

impl Offset2D {
	#[inline]
	pub const fn new(x: i32, y: i32) -> Self {
		Self(core::VkOffset2D {
			x,
			y,
		})
	}
}

impl Offset3D {
	#[inline]
	pub const fn new(x: i32, y: i32, z: i32) -> Self {
		Self(core::VkOffset3D {
			x,
			y,
			z,
		})
	}
}

impl Rect2D {
	#[inline]
	pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
		Self(core::VkRect2D {
			offset: core::VkOffset2D {
				x,
				y,
			},

			extent: core::VkExtent2D {
				width,
				height,
			},
		})
	}
}

impl BaseInStructure {}

impl BaseOutStructure {}

impl BufferMemoryBarrier {}

impl DispatchIndirectCommand {}

impl DrawIndexedIndirectCommand {}

impl DrawIndirectCommand {}

impl ImageSubresourceRange {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkImageSubresourceRange {
			aspectMask: core::VK_IMAGE_ASPECT_COLOR_BIT,
			baseMipLevel: 0,
			levelCount: 1,
			baseArrayLayer: 0,
			layerCount: 1,
		})
	}
}

impl ImageMemoryBarrier {}

impl MemoryBarrier {}

impl PipelineCacheHeaderVersionOne {}

impl ApplicationInfo {
	#[inline]
	pub const fn new(app_name: &str, app_version: u32, api_version: u32) -> Self {
		Self(core::VkApplicationInfo {
			sType: core::VK_STRUCTURE_TYPE_APPLICATION_INFO,
			pNext: mem::null(),
			pApplicationName: c_str(app_name),
			applicationVersion: app_version,
			pEngineName: mem::null(),
			engineVersion: 0,
			apiVersion: api_version,
		})
	}
}

impl FormatProperties {}

impl ImageFormatProperties {}

impl InstanceCreateInfo {
	#[inline]
	pub const fn new(application_info: &core::VkApplicationInfo) -> Self {
		Self(core::VkInstanceCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			pApplicationInfo: application_info,
			enabledLayerCount: 0,
			ppEnabledLayerNames: mem::null(),
			enabledExtensionCount: 0,
			ppEnabledExtensionNames: mem::null(),
		})
	}
}

impl MemoryHeap {}

impl MemoryType {}

impl PhysicalDeviceFeatures {}

impl PhysicalDeviceLimits {}

impl PhysicalDeviceMemoryProperties {}

impl PhysicalDeviceSparseProperties {}

impl PhysicalDeviceProperties {}

impl QueueFamilyProperties {
	#[inline(always)]
	pub const fn has_graphics_queues(&self) -> bool {
		(self.0.queueFlags & core::VK_QUEUE_GRAPHICS_BIT) > 0
	}

	#[inline(always)]
	pub const fn has_compute_queues(&self) -> bool {
		(self.0.queueFlags & core::VK_QUEUE_COMPUTE_BIT) > 0
	}

	#[inline(always)]
	pub const fn has_transfer_queues(&self) -> bool {
		(self.0.queueFlags & core::VK_QUEUE_TRANSFER_BIT) > 0
	}

	#[inline(always)]
	pub const fn has_dedicated_graphics_queues(&self) -> bool {
		self.has_graphics_queues() && !self.has_compute_queues()
	}

	#[inline(always)]
	pub const fn has_dedicated_compute_queues(&self) -> bool {
		!self.has_graphics_queues() && self.has_compute_queues()
	}

	#[inline(always)]
	pub const fn has_dedicated_transfer_queues(&self) -> bool {
		!self.has_graphics_queues() && !self.has_compute_queues() && self.has_transfer_queues()
	}
}

impl DeviceQueueCreateInfo {
	#[inline]
	pub const fn new(queue_family_index: u32, queue_priorities: &[f32]) -> Self {
		Self(core::VkDeviceQueueCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			queueFamilyIndex: queue_family_index,
			queueCount: queue_priorities.len() as _,
			pQueuePriorities: queue_priorities.as_ptr(),
		})
	}
}

impl DeviceCreateInfo {
	#[inline]
	pub const fn new(queue_create_infos: &[core::VkDeviceQueueCreateInfo]) -> Self {
		Self(core::VkDeviceCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			queueCreateInfoCount: queue_create_infos.len() as _,
			pQueueCreateInfos: queue_create_infos.as_ptr(),
			enabledLayerCount: 0,
			ppEnabledLayerNames: mem::null(),
			enabledExtensionCount: 0,
			ppEnabledExtensionNames: mem::null(),
			pEnabledFeatures: mem::null(),
		})
	}
}

impl ExtensionProperties {
	pub fn match_name(&self, name: &[u8]) -> bool {
		if name.len() == 0 {
			return false;
		}

		for n in 0..cmp::min(self.extensionName.len(), name.len()) {
			if (self.extensionName[n] as u8) != name[n] {
				return false;
			}
		}

		true
	}
}

impl LayerProperties {}

impl SubmitInfo {
	pub const fn new(command_buffers: &[core::VkCommandBuffer], wait_semaphores: &[core::VkSemaphore], signal_semaphores: &[core::VkSemaphore], wait_dst_stage: &[core::VkPipelineStageFlags]) -> Self {
		Self(core::VkSubmitInfo {
			sType: core::VK_STRUCTURE_TYPE_SUBMIT_INFO,
			pNext: mem::null(),
			waitSemaphoreCount: wait_semaphores.len() as _,
			pWaitSemaphores: wait_semaphores.as_ptr(),
			pWaitDstStageMask: wait_dst_stage.as_ptr(),
			commandBufferCount: command_buffers.len() as _,
			pCommandBuffers: command_buffers.as_ptr(),
			signalSemaphoreCount: signal_semaphores.len() as _,
			pSignalSemaphores: signal_semaphores.as_ptr(),
		})
	}
}

impl MappedMemoryRange {}

impl MemoryAllocateInfo {
	#[inline]
	pub const fn new(allocation_size: core::VkDeviceSize, memory_type_index: u32) -> Self {
		Self(core::VkMemoryAllocateInfo {
			sType: core::VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
			pNext: mem::null(),
			allocationSize: allocation_size,
			memoryTypeIndex: memory_type_index,
		})
	}
}

impl MemoryRequirements {}

impl SparseMemoryBind {}

impl SparseBufferMemoryBindInfo {}

impl SparseImageOpaqueMemoryBindInfo {}

impl ImageSubresource {}

impl SparseImageMemoryBind {}

impl SparseImageMemoryBindInfo {}

impl BindSparseInfo {}

impl SparseImageFormatProperties {}

impl SparseImageMemoryRequirements {}

impl FenceCreateInfo {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkFenceCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_FENCE_CREATE_INFO,
			pNext: mem::null(),
			flags: core::VK_FENCE_CREATE_SIGNALED_BIT,
		})
	}
}

impl SemaphoreCreateInfo {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkSemaphoreCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
		})
	}
}

impl EventCreateInfo {}

impl QueryPoolCreateInfo {}

impl BufferCreateInfo {
	#[inline]
	pub const fn new(size: core::VkDeviceSize, usage: core::VkBufferUsageFlags, queue_family_indices: &[u32]) -> Self {
		Self(core::VkBufferCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			size,
			usage,

			sharingMode: if queue_family_indices.len() > 1 {
				core::VK_SHARING_MODE_CONCURRENT
			} else {
				core::VK_SHARING_MODE_EXCLUSIVE
			},

			queueFamilyIndexCount: queue_family_indices.len() as _,
			pQueueFamilyIndices: queue_family_indices.as_ptr(),
		})
	}

	#[inline]
	pub const fn for_vertex_buffers(size: core::VkDeviceSize, queue_family_indices: &[u32]) -> Self {
		let usage = core::VK_BUFFER_USAGE_VERTEX_BUFFER_BIT | core::VK_BUFFER_USAGE_TRANSFER_DST_BIT;

		Self::new(size, usage, queue_family_indices)
	}

	#[inline]
	pub const fn for_index_buffers(size: core::VkDeviceSize, queue_family_indices: &[u32]) -> Self {
		let usage = core::VK_BUFFER_USAGE_INDEX_BUFFER_BIT | core::VK_BUFFER_USAGE_TRANSFER_DST_BIT;

		Self::new(size, usage, queue_family_indices)
	}

	#[inline]
	pub const fn for_transfer_buffers(size: core::VkDeviceSize, queue_family_indices: &[u32]) -> Self {
		let usage = core::VK_BUFFER_USAGE_TRANSFER_SRC_BIT | core::VK_BUFFER_USAGE_TRANSFER_DST_BIT;

		Self::new(size, usage, queue_family_indices)
	}

	#[inline]
	pub const fn for_uniform_buffers(size: core::VkDeviceSize, queue_family_indices: &[u32]) -> Self {
		let usage = core::VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT;

		Self::new(size, usage, queue_family_indices)
	}
}

impl BufferViewCreateInfo {}

impl ImageCreateInfo {
	#[inline]
	pub const fn new(format: core::VkFormat, extent: &core::VkExtent3D, usage: core::VkImageUsageFlagBits, mip_levels: u32) -> Self {
		Self(core::VkImageCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,

			imageType: if extent.depth > 1 {
				core::VK_IMAGE_TYPE_3D
			} else {
				core::VK_IMAGE_TYPE_2D
			},

			format,
			extent: *extent,
			mipLevels: mip_levels,
			arrayLayers: 1,
			samples: core::VK_SAMPLE_COUNT_1_BIT,
			tiling: core::VK_IMAGE_TILING_OPTIMAL,
			usage,
			sharingMode: core::VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices: mem::null(),
			initialLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
		})
	}

	#[inline]
	pub const fn for_sampled_textures(format: core::VkFormat, extent: &core::VkExtent3D, mip_levels: u32) -> Self {
		let usage = core::VK_IMAGE_USAGE_SAMPLED_BIT | core::VK_IMAGE_USAGE_TRANSFER_DST_BIT;

		Self::new(format, extent, usage, mip_levels)
	}

	#[inline]
	pub const fn for_render_targets(format: core::VkFormat, extent: &core::VkExtent3D) -> Self {
		let usage = core::VK_IMAGE_USAGE_SAMPLED_BIT | core::VK_IMAGE_USAGE_TRANSFER_SRC_BIT | core::VK_IMAGE_USAGE_TRANSFER_DST_BIT | core::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT;

		Self::new(format, extent, usage, 1)
	}

	#[inline]
	pub const fn for_depth_buffers(format: core::VkFormat, extent: &core::VkExtent3D) -> Self {
		Self::new(format, extent, core::VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT, 1)
	}
}

impl SubresourceLayout {}

impl ComponentMapping {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkComponentMapping {
			r: core::VK_COMPONENT_SWIZZLE_IDENTITY,
			g: core::VK_COMPONENT_SWIZZLE_IDENTITY,
			b: core::VK_COMPONENT_SWIZZLE_IDENTITY,
			a: core::VK_COMPONENT_SWIZZLE_IDENTITY,
		})
	}
}

impl ImageViewCreateInfo {
	#[inline]
	pub const fn as_2d(image: core::VkImage, format: core::VkFormat) -> Self {
		Self(core::VkImageViewCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			image,
			viewType: core::VK_IMAGE_VIEW_TYPE_2D,
			format,
			components: ComponentMapping::new().0,
			subresourceRange: ImageSubresourceRange::new().0,
		})
	}
}

impl ShaderModuleCreateInfo {
	#[inline]
	pub const fn new(code: &[spirv::Word]) -> Self {
		let size = code.len() * mem::size_of::<spirv::Word>();

		Self(core::VkShaderModuleCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			codeSize: size,
			pCode: code.as_ptr(),
		})
	}
}

impl PipelineCacheCreateInfo {}

impl SpecializationMapEntry {}

impl SpecializationInfo {}

impl PipelineShaderStageCreateInfo {
	#[inline]
	pub const fn new(module: core::VkShaderModule, stage: core::VkShaderStageFlagBits, name: &str) -> Self {
		Self(core::VkPipelineShaderStageCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			stage,
			module,
			pName: c_str(name),
			pSpecializationInfo: mem::null(),
		})
	}

	#[inline]
	pub const fn for_vertex_stages(module: core::VkShaderModule, name: &str) -> Self {
		Self::new(module, core::VK_SHADER_STAGE_VERTEX_BIT, name)
	}

	#[inline]
	pub const fn for_fragment_stages(module: core::VkShaderModule, name: &str) -> Self {
		Self::new(module, core::VK_SHADER_STAGE_FRAGMENT_BIT, name)
	}

	#[inline]
	pub const fn for_compute_stages(module: core::VkShaderModule, name: &str) -> Self {
		Self::new(module, core::VK_SHADER_STAGE_COMPUTE_BIT, name)
	}
}

impl ComputePipelineCreateInfo {}

impl VertexInputBindingDescription {}

impl VertexInputAttributeDescription {}

impl PipelineVertexInputStateCreateInfo {
	#[inline]
	pub const fn new(vertex_binding_descriptions: &[core::VkVertexInputBindingDescription], vertex_attribute_descriptions: &[core::VkVertexInputAttributeDescription]) -> Self {
		Self(core::VkPipelineVertexInputStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			vertexBindingDescriptionCount: vertex_binding_descriptions.len() as _,
			pVertexBindingDescriptions: vertex_binding_descriptions.as_ptr(),
			vertexAttributeDescriptionCount: vertex_attribute_descriptions.len() as _,
			pVertexAttributeDescriptions: vertex_attribute_descriptions.as_ptr(),
		})
	}

	#[inline]
	pub const fn empty() -> Self {
		Self(core::VkPipelineVertexInputStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			vertexBindingDescriptionCount: 0,
			pVertexBindingDescriptions: mem::null(),
			vertexAttributeDescriptionCount: 0,
			pVertexAttributeDescriptions: mem::null(),
		})
	}
}

impl PipelineInputAssemblyStateCreateInfo {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkPipelineInputAssemblyStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			topology: core::VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
			primitiveRestartEnable: core::VK_FALSE,
		})
	}
}

impl PipelineTessellationStateCreateInfo {}

impl Viewport {
	#[inline]
	pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
		// NOTE: The default viewport is built such that it is compatible with the OpenGL equivalent.
		// The origin of the normalized device coordinate used by OpenGL (-1, -1) maps to the bottom
		// left corner, whereas the top left corner is used for Vulkan. Thus, VkViewport is populated
		// in a manner to mimic the OpenGL case and no modifications in the shaders are needed if the
		// graphics API changes.
		Self(core::VkViewport {
			x,
			y: y + height,
			width,
			height: -height,
			minDepth: 0.0,
			maxDepth: 1.0,
		})
	}
}

impl PipelineViewportStateCreateInfo {
	#[inline]
	pub const fn new(viewports: &[core::VkViewport], scissors: &[core::VkRect2D]) -> Self {
		Self(core::VkPipelineViewportStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			viewportCount: viewports.len() as _,
			pViewports: viewports.as_ptr(),
			scissorCount: scissors.len() as _,
			pScissors: scissors.as_ptr(),
		})
	}

	#[inline]
	pub const fn for_dynamic_states(viewport_count: u32, scissor_count: u32) -> Self {
		Self(core::VkPipelineViewportStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			viewportCount: viewport_count,
			pViewports: mem::null(),
			scissorCount: scissor_count,
			pScissors: mem::null(),
		})
	}
}

impl PipelineRasterizationStateCreateInfo {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkPipelineRasterizationStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			depthClampEnable: core::VK_FALSE,
			rasterizerDiscardEnable: core::VK_FALSE,
			polygonMode: core::VK_POLYGON_MODE_FILL,
			cullMode: core::VK_CULL_MODE_BACK_BIT,
			frontFace: core::VK_FRONT_FACE_CLOCKWISE,
			depthBiasEnable: core::VK_FALSE,
			depthBiasConstantFactor: 0.0,
			depthBiasClamp: 0.0,
			depthBiasSlopeFactor: 0.0,
			lineWidth: 1.0,
		})
	}
}

impl PipelineMultisampleStateCreateInfo {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkPipelineMultisampleStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			rasterizationSamples: core::VK_SAMPLE_COUNT_1_BIT,
			sampleShadingEnable: core::VK_FALSE,
			minSampleShading: 1.0,
			pSampleMask: mem::null(),
			alphaToCoverageEnable: core::VK_FALSE,
			alphaToOneEnable: core::VK_FALSE,
		})
	}
}

impl StencilOpState {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkStencilOpState {
			failOp: core::VK_STENCIL_OP_KEEP,
			passOp: core::VK_STENCIL_OP_KEEP,
			depthFailOp: core::VK_STENCIL_OP_KEEP,
			compareOp: core::VK_COMPARE_OP_NEVER,
			compareMask: 0,
			writeMask: 0,
			reference: 0,
		})
	}
}

impl PipelineDepthStencilStateCreateInfo {
	#[inline]
	pub const fn for_depth_test() -> Self {
		Self(core::VkPipelineDepthStencilStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			depthTestEnable: core::VK_TRUE,
			depthWriteEnable: core::VK_TRUE,
			depthCompareOp: core::VK_COMPARE_OP_LESS_OR_EQUAL,
			depthBoundsTestEnable: core::VK_FALSE,
			stencilTestEnable: core::VK_FALSE,
			front: StencilOpState::new().0,
			back: StencilOpState::new().0,
			minDepthBounds: 0.0,
			maxDepthBounds: 0.0,
		})
	}
}

impl PipelineColorBlendAttachmentState {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkPipelineColorBlendAttachmentState {
			blendEnable: core::VK_TRUE,
			srcColorBlendFactor: core::VK_BLEND_FACTOR_SRC_ALPHA,
			dstColorBlendFactor: core::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			colorBlendOp: core::VK_BLEND_OP_ADD,
			srcAlphaBlendFactor: core::VK_BLEND_FACTOR_SRC_ALPHA,
			dstAlphaBlendFactor: core::VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA,
			alphaBlendOp: core::VK_BLEND_OP_ADD,
			colorWriteMask: core::VK_COLOR_COMPONENT_R_BIT | core::VK_COLOR_COMPONENT_G_BIT | core::VK_COLOR_COMPONENT_B_BIT | core::VK_COLOR_COMPONENT_A_BIT,
		})
	}
}

impl PipelineColorBlendStateCreateInfo {
	#[inline]
	pub const fn empty() -> Self {
		Self(core::VkPipelineColorBlendStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			logicOpEnable: core::VK_FALSE,
			logicOp: core::VK_LOGIC_OP_COPY,
			attachmentCount: 0,
			pAttachments: mem::null(),
			blendConstants: [0.0; 4],
		})
	}

	#[inline]
	pub const fn new(attachments: &[core::VkPipelineColorBlendAttachmentState]) -> Self {
		Self(core::VkPipelineColorBlendStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			logicOpEnable: core::VK_FALSE,
			logicOp: core::VK_LOGIC_OP_COPY,
			attachmentCount: attachments.len() as _,
			pAttachments: attachments.as_ptr(),
			blendConstants: [0.0; 4],
		})
	}
}

impl PipelineDynamicStateCreateInfo {
	#[inline]
	pub const fn empty() -> Self {
		Self(core::VkPipelineDynamicStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			dynamicStateCount: 0,
			pDynamicStates: mem::null(),
		})
	}

	#[inline]
	pub const fn new(dynamic_states: &[core::VkDynamicState]) -> Self {
		Self(core::VkPipelineDynamicStateCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			dynamicStateCount: dynamic_states.len() as _,
			pDynamicStates: dynamic_states.as_ptr(),
		})
	}
}

impl GraphicsPipelineCreateInfo {
	#[inline]
	pub const fn empty() -> Self {
		Self(core::VkGraphicsPipelineCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			stageCount: 0,
			pStages: mem::null(),
			pVertexInputState: mem::null(),
			pInputAssemblyState: mem::null(),
			pTessellationState: mem::null(),
			pViewportState: mem::null(),
			pRasterizationState: mem::null(),
			pMultisampleState: mem::null(),
			pDepthStencilState: mem::null(),
			pColorBlendState: mem::null(),
			pDynamicState: mem::null(),
			layout: mem::null(),
			renderPass: mem::null(),
			subpass: 0,
			basePipelineHandle: mem::null(),
			basePipelineIndex: -1,
		})
	}
}

impl PushConstantRange {
	#[inline]
	pub const fn for_vertex_stages(offset: u32, size: u32) -> Self {
		Self(core::VkPushConstantRange {
			stageFlags: core::VK_SHADER_STAGE_VERTEX_BIT,
			offset,
			size,
		})
	}

	#[inline]
	pub const fn for_fragment_stages(offset: u32, size: u32) -> Self {
		Self(core::VkPushConstantRange {
			stageFlags: core::VK_SHADER_STAGE_FRAGMENT_BIT,
			offset,
			size,
		})
	}

	#[inline]
	pub const fn for_compute_stages(offset: u32, size: u32) -> Self {
		Self(core::VkPushConstantRange {
			stageFlags: core::VK_SHADER_STAGE_COMPUTE_BIT,
			offset,
			size,
		})
	}

	#[inline]
	pub const fn for_graphics_stages(offset: u32, size: u32) -> Self {
		Self(core::VkPushConstantRange {
			stageFlags: core::VK_SHADER_STAGE_VERTEX_BIT | core::VK_SHADER_STAGE_FRAGMENT_BIT,
			offset,
			size,
		})
	}
}

impl PipelineLayoutCreateInfo {
	pub const fn new(set_layouts: &[core::VkDescriptorSetLayout]) -> Self {
		Self(core::VkPipelineLayoutCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			setLayoutCount: set_layouts.len() as _,
			pSetLayouts: set_layouts.as_ptr(),
			pushConstantRangeCount: 0,
			pPushConstantRanges: mem::null(),
		})
	}
}

impl SamplerCreateInfo {
	#[inline]
	pub const fn linear() -> Self {
		Self(core::VkSamplerCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			magFilter: core::VK_FILTER_LINEAR,
			minFilter: core::VK_FILTER_LINEAR,
			mipmapMode: core::VK_SAMPLER_MIPMAP_MODE_LINEAR,
			addressModeU: core::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeV: core::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			addressModeW: core::VK_SAMPLER_ADDRESS_MODE_REPEAT,
			mipLodBias: 0.0,

			// NOTE: This can be set to VK_TRUE only if VkPhysicalDeviceFeatures::samplerAnisotropy
			// is VK_TRUE. Otherwise, VK_FALSE is the correct and safe default. Anisotropy improves
			// texture quality at oblique angles but may incur a performance cost.
			anisotropyEnable: core::VK_FALSE,

			// NOTE: Ignored if anisotropyEnable = VK_FALSE. If anisotropy is enabled, this value
			// should be clamped to VkPhysicalDeviceLimits::maxSamplerAnisotropy. Typical values
			// are 4, 8, and 16.
			maxAnisotropy: 16.0,

			compareEnable: core::VK_FALSE,
			compareOp: core::VK_COMPARE_OP_ALWAYS,
			minLod: 0.0,
			maxLod: 0.0,
			borderColor: core::VK_BORDER_COLOR_INT_OPAQUE_BLACK,
			unnormalizedCoordinates: core::VK_FALSE,
		})
	}
}

impl CopyDescriptorSet {}

impl DescriptorBufferInfo {}

impl DescriptorImageInfo {
	#[inline]
	pub const fn for_read_only_layouts(sampler: core::VkSampler, image_view: core::VkImageView) -> Self {
		Self(core::VkDescriptorImageInfo {
			sampler,
			imageView: image_view,
			imageLayout: core::VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL,
		})
	}
}

impl DescriptorPoolSize {}

impl DescriptorPoolCreateInfo {
	#[inline]
	pub const fn new(max_sets: u32, pool_sizes: &[core::VkDescriptorPoolSize]) -> Self {
		Self(core::VkDescriptorPoolCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			maxSets: max_sets,
			poolSizeCount: pool_sizes.len() as _,
			pPoolSizes: pool_sizes.as_ptr(),
		})
	}
}

impl DescriptorSetAllocateInfo {
	#[inline]
	pub const fn new(descriptor_pool: core::VkDescriptorPool, set_layouts: &[core::VkDescriptorSetLayout]) -> Self {
		Self(core::VkDescriptorSetAllocateInfo {
			sType: core::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
			pNext: mem::null(),
			descriptorPool: descriptor_pool,
			descriptorSetCount: set_layouts.len() as _,
			pSetLayouts: set_layouts.as_ptr(),
		})
	}
}

impl DescriptorSetLayoutBinding {}

impl DescriptorSetLayoutCreateInfo {
	pub const fn new(bindings: &[core::VkDescriptorSetLayoutBinding]) -> Self {
		Self(core::VkDescriptorSetLayoutCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			bindingCount: bindings.len() as _,
			pBindings: bindings.as_ptr(),
		})
	}
}

impl WriteDescriptorSet {}

impl AttachmentDescription {}

impl AttachmentReference {}

impl FramebufferCreateInfo {
	pub const fn new(width: u32, height: u32, render_pass: core::VkRenderPass, attachments: &[core::VkImageView]) -> Self {
		Self(core::VkFramebufferCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			renderPass: render_pass,
			attachmentCount: attachments.len() as _,
			pAttachments: attachments.as_ptr(),
			width,
			height,
			layers: 1,
		})
	}
}

impl SubpassDescription {}

impl SubpassDependency {}

impl RenderPassCreateInfo {
	pub const fn new(attachments: &[core::VkAttachmentDescription], subpasses: &[core::VkSubpassDescription], dependencies: &[core::VkSubpassDependency]) -> Self {
		Self(core::VkRenderPassCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
			pNext: mem::null(),
			flags: 0,
			attachmentCount: attachments.len() as _,
			pAttachments: attachments.as_ptr(),
			subpassCount: subpasses.len() as _,
			pSubpasses: subpasses.as_ptr(),
			dependencyCount: dependencies.len() as _,
			pDependencies: dependencies.as_ptr(),
		})
	}
}

impl CommandPoolCreateInfo {
	#[inline]
	pub const fn new(queue_family_index: u32) -> Self {
		Self(core::VkCommandPoolCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
			pNext: mem::null(),
			flags: core::VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
			queueFamilyIndex: queue_family_index,
		})
	}
}

impl CommandBufferAllocateInfo {
	#[inline]
	pub const fn new(command_pool: core::VkCommandPool, command_buffer_count: u32) -> Self {
		Self(core::VkCommandBufferAllocateInfo {
			sType: core::VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
			pNext: mem::null(),
			commandPool: command_pool,
			level: core::VK_COMMAND_BUFFER_LEVEL_PRIMARY,
			commandBufferCount: command_buffer_count,
		})
	}
}

impl CommandBufferInheritanceInfo {}

impl CommandBufferBeginInfo {
	#[inline]
	pub const fn new(flags: core::VkCommandBufferUsageFlagBits) -> Self {
		Self(core::VkCommandBufferBeginInfo {
			sType: core::VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
			pNext: mem::null(),
			flags,
			pInheritanceInfo: mem::null(),
		})
	}

	#[inline]
	pub const fn for_one_time_submit() -> Self {
		Self::new(core::VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT)
	}
}

impl BufferCopy {
	#[inline]
	pub const fn new(src_offset: core::VkDeviceSize, dst_offset: core::VkDeviceSize, size: core::VkDeviceSize) -> Self {
		Self(core::VkBufferCopy {
			srcOffset: src_offset,
			dstOffset: dst_offset,
			size,
		})
	}
}

impl ImageSubresourceLayers {
	#[inline]
	pub const fn new(mip_level: u32) -> Self {
		Self(core::VkImageSubresourceLayers {
			aspectMask: core::VK_IMAGE_ASPECT_COLOR_BIT,
			mipLevel: mip_level,
			baseArrayLayer: 0,
			layerCount: 1,
		})
	}
}

impl BufferImageCopy {
	#[inline]
	pub const fn new(width: u32, height: u32, depth: u32, mip_level: u32) -> Self {
		Self(core::VkBufferImageCopy {
			bufferOffset: 0,
			bufferRowLength: 0,
			bufferImageHeight: 0,
			imageSubresource: ImageSubresourceLayers::new(mip_level).0,
			imageOffset: Offset3D::new(0, 0, 0).0,
			imageExtent: Extent3D::new(width, height, depth).0,
		})
	}
}

impl ClearDepthStencilValue {}

impl ClearAttachment {}

impl ClearRect {}

impl ImageBlit {}

impl ImageCopy {}

impl ImageResolve {}

impl RenderPassBeginInfo {
	#[inline]
	pub const fn new(render_pass: core::VkRenderPass, framebuffer: core::VkFramebuffer, render_area: &core::VkRect2D) -> Self {
		Self(core::VkRenderPassBeginInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
			pNext: mem::null(),
			renderPass: render_pass,
			framebuffer,
			renderArea: *render_area,
			clearValueCount: 0,
			pClearValues: mem::null(),
		})
	}
}

impl PhysicalDeviceSubgroupProperties {}

impl BindBufferMemoryInfo {}

impl BindImageMemoryInfo {}

impl PhysicalDevice16BitStorageFeatures {}

impl MemoryDedicatedRequirements {}

impl MemoryDedicatedAllocateInfo {}

impl MemoryAllocateFlagsInfo {}

impl DeviceGroupRenderPassBeginInfo {}

impl DeviceGroupCommandBufferBeginInfo {}

impl DeviceGroupSubmitInfo {}

impl DeviceGroupBindSparseInfo {}

impl BindBufferMemoryDeviceGroupInfo {}

impl BindImageMemoryDeviceGroupInfo {}

impl PhysicalDeviceGroupProperties {}

impl DeviceGroupDeviceCreateInfo {}

impl BufferMemoryRequirementsInfo2 {}

impl ImageMemoryRequirementsInfo2 {}

impl ImageSparseMemoryRequirementsInfo2 {}

impl MemoryRequirements2 {}

impl SparseImageMemoryRequirements2 {}

impl PhysicalDeviceFeatures2 {}

impl PhysicalDeviceProperties2 {}

impl FormatProperties2 {}

impl ImageFormatProperties2 {}

impl PhysicalDeviceImageFormatInfo2 {}

impl QueueFamilyProperties2 {}

impl PhysicalDeviceMemoryProperties2 {}

impl SparseImageFormatProperties2 {}

impl PhysicalDeviceSparseImageFormatInfo2 {}

impl PhysicalDevicePointClippingProperties {}

impl InputAttachmentAspectReference {}

impl RenderPassInputAttachmentAspectCreateInfo {}

impl ImageViewUsageCreateInfo {}

impl PipelineTessellationDomainOriginStateCreateInfo {}

impl RenderPassMultiviewCreateInfo {}

impl PhysicalDeviceMultiviewFeatures {}

impl PhysicalDeviceMultiviewProperties {}

impl PhysicalDeviceVariablePointersFeatures {}

impl PhysicalDeviceProtectedMemoryFeatures {}

impl PhysicalDeviceProtectedMemoryProperties {}

impl DeviceQueueInfo2 {}

impl ProtectedSubmitInfo {}

impl SamplerYcbcrConversionCreateInfo {}

impl SamplerYcbcrConversionInfo {}

impl BindImagePlaneMemoryInfo {}

impl ImagePlaneMemoryRequirementsInfo {}

impl PhysicalDeviceSamplerYcbcrConversionFeatures {}

impl SamplerYcbcrConversionImageFormatProperties {}

impl DescriptorUpdateTemplateEntry {}

impl DescriptorUpdateTemplateCreateInfo {}

impl ExternalMemoryProperties {}

impl PhysicalDeviceExternalImageFormatInfo {}

impl ExternalImageFormatProperties {}

impl PhysicalDeviceExternalBufferInfo {}

impl ExternalBufferProperties {}

impl PhysicalDeviceIDProperties {}

impl ExternalMemoryImageCreateInfo {}

impl ExternalMemoryBufferCreateInfo {}

impl ExportMemoryAllocateInfo {}

impl PhysicalDeviceExternalFenceInfo {}

impl ExternalFenceProperties {}

impl ExportFenceCreateInfo {}

impl ExportSemaphoreCreateInfo {}

impl PhysicalDeviceExternalSemaphoreInfo {}

impl ExternalSemaphoreProperties {}

impl PhysicalDeviceMaintenance3Properties {}

impl DescriptorSetLayoutSupport {}

impl PhysicalDeviceShaderDrawParametersFeatures {}

impl PhysicalDeviceVulkan11Features {}

impl PhysicalDeviceVulkan11Properties {}

impl PhysicalDeviceVulkan12Features {}

impl ConformanceVersion {}

impl PhysicalDeviceVulkan12Properties {}

impl ImageFormatListCreateInfo {}

impl AttachmentDescription2 {}

impl AttachmentReference2 {}

impl SubpassDescription2 {}

impl SubpassDependency2 {}

impl RenderPassCreateInfo2 {}

impl SubpassBeginInfo {}

impl SubpassEndInfo {}

impl PhysicalDevice8BitStorageFeatures {}

impl PhysicalDeviceDriverProperties {}

impl PhysicalDeviceShaderAtomicInt64Features {}

impl PhysicalDeviceShaderFloat16Int8Features {}

impl PhysicalDeviceFloatControlsProperties {}

impl DescriptorSetLayoutBindingFlagsCreateInfo {}

impl PhysicalDeviceDescriptorIndexingFeatures {}

impl PhysicalDeviceDescriptorIndexingProperties {}

impl DescriptorSetVariableDescriptorCountAllocateInfo {}

impl DescriptorSetVariableDescriptorCountLayoutSupport {}

impl SubpassDescriptionDepthStencilResolve {}

impl PhysicalDeviceDepthStencilResolveProperties {}

impl PhysicalDeviceScalarBlockLayoutFeatures {}

impl ImageStencilUsageCreateInfo {}

impl SamplerReductionModeCreateInfo {}

impl PhysicalDeviceSamplerFilterMinmaxProperties {}

impl PhysicalDeviceVulkanMemoryModelFeatures {}

impl PhysicalDeviceImagelessFramebufferFeatures {}

impl FramebufferAttachmentImageInfo {}

impl FramebufferAttachmentsCreateInfo {}

impl RenderPassAttachmentBeginInfo {}

impl PhysicalDeviceUniformBufferStandardLayoutFeatures {}

impl PhysicalDeviceShaderSubgroupExtendedTypesFeatures {}

impl PhysicalDeviceSeparateDepthStencilLayoutsFeatures {}

impl AttachmentReferenceStencilLayout {}

impl AttachmentDescriptionStencilLayout {}

impl PhysicalDeviceHostQueryResetFeatures {}

impl PhysicalDeviceTimelineSemaphoreFeatures {}

impl PhysicalDeviceTimelineSemaphoreProperties {}

impl SemaphoreTypeCreateInfo {}

impl TimelineSemaphoreSubmitInfo {}

impl SemaphoreWaitInfo {
	#[inline]
	pub const fn new(semaphores: &[core::VkSemaphore]) -> Self {
		Self(core::VkSemaphoreWaitInfo {
			sType: core::VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO,
			pNext: mem::null(),
			flags: core::VK_SEMAPHORE_WAIT_ANY_BIT,
			semaphoreCount: semaphores.len() as _,
			pSemaphores: semaphores.as_ptr(),
			pValues: mem::null(),
		})
	}
}

impl SemaphoreSignalInfo {}

impl PhysicalDeviceBufferDeviceAddressFeatures {}

impl BufferDeviceAddressInfo {}

impl BufferOpaqueCaptureAddressCreateInfo {}

impl MemoryOpaqueCaptureAddressAllocateInfo {}

impl DeviceMemoryOpaqueCaptureAddressInfo {}

impl PhysicalDeviceVulkan13Features {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkPhysicalDeviceVulkan13Features {
			sType: core::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
			pNext: mem::null(),

			// Prevents out-of-bounds image accesses from causing undefined behavior.
			// Slight performance cost; rely on validation + correct shaders instead.
			#[cfg(not(debug_assertions))]
			robustImageAccess: core::VK_FALSE,

			#[cfg(debug_assertions)]
			robustImageAccess: core::VK_TRUE,

			// Allows small uniform data embedded directly in descriptors.
			// Push constants already cover this use case more cleanly.
			inlineUniformBlock: core::VK_FALSE,

			// Allows updating inline uniform blocks after descriptor binding.
			// Only relevant if inlineUniformBlock is enabled.
			descriptorBindingInlineUniformBlockUpdateAfterBind: core::VK_FALSE,

			// Enables pipeline creation control flags (fail fast, early return).
			// Useful for avoiding runtime stalls and controlling compilation.
			pipelineCreationCacheControl: core::VK_TRUE,

			// Allows attaching arbitrary engine data to Vulkan objects.
			// Extremely useful for debugging, validation, and tooling.
			privateData: core::VK_TRUE,

			// Allows fragment shaders to demote fragments without killing derivatives.
			// Useful for advanced transparency and MSAA correctness.
			shaderDemoteToHelperInvocation: core::VK_TRUE,

			// Allows shaders to explicitly terminate execution.
			// Enables modern control-flow patterns.
			shaderTerminateInvocation: core::VK_TRUE,

			// Allows explicit control over subgroup (wave) size in shaders.
			// Important for high-performance compute and SIMD tuning.
			subgroupSizeControl: core::VK_TRUE,

			// Guarantees full subgroups in compute shaders.
			// Makes subgroup algorithms deterministic and easier to write.
			computeFullSubgroups: core::VK_TRUE,

			// Enables the modern synchronization API (vkCmdPipelineBarrier2, etc.).
			// Strongly recommended over legacy synchronization.
			synchronization2: core::VK_TRUE,

			// Enables ASTC HDR texture formats.
			// Primarily relevant for mobile; desktop engines usually skip this.
			textureCompressionASTC_HDR: core::VK_FALSE,

			// Automatically zero-initializes workgroup memory.
			// Safer but slightly slower; prefer explicit initialization.
			shaderZeroInitializeWorkgroupMemory: core::VK_FALSE,

			// Enables dynamic rendering (no render passes or framebuffers).
			// Core requirement for modern Vulkan rendering.
			dynamicRendering: core::VK_TRUE,

			// Enables integer dot-product instructions.
			// Only needed for ML and specialized math workloads.
			shaderIntegerDotProduct: core::VK_FALSE,

			// Collection of small but important API and correctness improvements.
			// No downsides; always enable.
			maintenance4: core::VK_TRUE,
		})
	}
}

impl PhysicalDeviceVulkan13Properties {}

impl PipelineCreationFeedback {}

impl PipelineCreationFeedbackCreateInfo {}

impl PhysicalDeviceShaderTerminateInvocationFeatures {}

impl PhysicalDeviceToolProperties {}

impl PhysicalDeviceShaderDemoteToHelperInvocationFeatures {}

impl PhysicalDevicePrivateDataFeatures {}

impl DevicePrivateDataCreateInfo {}

impl PrivateDataSlotCreateInfo {}

impl PhysicalDevicePipelineCreationCacheControlFeatures {}

impl MemoryBarrier2 {}

impl BufferMemoryBarrier2 {}

impl ImageMemoryBarrier2 {}

impl DependencyInfo {}

impl SemaphoreSubmitInfo {}

impl CommandBufferSubmitInfo {}

impl SubmitInfo2 {}

impl PhysicalDeviceSynchronization2Features {}

impl PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {}

impl PhysicalDeviceImageRobustnessFeatures {}

impl BufferCopy2 {}

impl CopyBufferInfo2 {}

impl ImageCopy2 {}

impl CopyImageInfo2 {}

impl BufferImageCopy2 {}

impl CopyBufferToImageInfo2 {}

impl CopyImageToBufferInfo2 {}

impl ImageBlit2 {}

impl BlitImageInfo2 {}

impl ImageResolve2 {}

impl ResolveImageInfo2 {}

impl PhysicalDeviceSubgroupSizeControlFeatures {}

impl PhysicalDeviceSubgroupSizeControlProperties {}

impl PipelineShaderStageRequiredSubgroupSizeCreateInfo {}

impl PhysicalDeviceInlineUniformBlockFeatures {}

impl PhysicalDeviceInlineUniformBlockProperties {}

impl WriteDescriptorSetInlineUniformBlock {}

impl DescriptorPoolInlineUniformBlockCreateInfo {}

impl PhysicalDeviceTextureCompressionASTCHDRFeatures {}

impl RenderingAttachmentInfo {
	#[inline]
	pub const fn invalid() -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: mem::null(),
			imageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			storeOp: core::VK_ATTACHMENT_STORE_OP_DONT_CARE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_cleared_single_sampled_targets(target: core::VkImageView, clear_value: &core::VkClearValue) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: *clear_value,
		})
	}

	#[inline]
	pub const fn for_loaded_single_sampled_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_LOAD,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_discarded_single_sampled_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_multisampled_averaged_targets(msaa_attachment: core::VkImageView, target: core::VkImageView, clear_value: &core::VkClearValue) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: msaa_attachment,
			imageLayout: core::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_AVERAGE_BIT,
			resolveImageView: target,
			resolveImageLayout: core::VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: core::VK_ATTACHMENT_STORE_OP_DONT_CARE,
			clearValue: *clear_value,
		})
	}

	#[inline]
	pub const fn for_cleared_depth_targets(target: core::VkImageView, clear_value: &core::VkClearValue) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: *clear_value,
		})
	}

	#[inline]
	pub const fn for_loaded_depth_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_LOAD,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_discarded_depth_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_cleared_stencil_targets(target: core::VkImageView, clear_value: &core::VkClearValue) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: *clear_value,
		})
	}

	#[inline]
	pub const fn for_loaded_stencil_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_LOAD,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_discarded_stencil_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_cleared_depth_and_stencil_targets(target: core::VkImageView, clear_value: &core::VkClearValue) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_CLEAR,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: *clear_value,
		})
	}

	#[inline]
	pub const fn for_loaded_depth_and_stencil_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_LOAD,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}

	#[inline]
	pub const fn for_discarded_depth_and_stencil_targets(target: core::VkImageView) -> Self {
		Self(core::VkRenderingAttachmentInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO,
			pNext: mem::null(),
			imageView: target,
			imageLayout: core::VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			resolveMode: core::VK_RESOLVE_MODE_NONE,
			resolveImageView: mem::null(),
			resolveImageLayout: core::VK_IMAGE_LAYOUT_UNDEFINED,
			loadOp: core::VK_ATTACHMENT_LOAD_OP_DONT_CARE,
			storeOp: core::VK_ATTACHMENT_STORE_OP_STORE,
			clearValue: unsafe { mem::zeroed() },
		})
	}
}

impl RenderingInfo {
	pub const fn empty() -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: unsafe { mem::zeroed() },
			layerCount: 0,
			viewMask: 0,
			colorAttachmentCount: 0,
			pColorAttachments: mem::null(),
			pDepthAttachment: mem::null(),
			pStencilAttachment: mem::null(),
		})
	}

	pub const fn with_colors(render_area: &core::VkRect2D, color_attachments: &[core::VkRenderingAttachmentInfo]) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: color_attachments.len() as _,
			pColorAttachments: color_attachments.as_ptr(),
			pDepthAttachment: mem::null(),
			pStencilAttachment: mem::null(),
		})
	}

	pub const fn with_colors_and_depth(render_area: &core::VkRect2D, color_attachments: &[core::VkRenderingAttachmentInfo], depth_attachment: &core::VkRenderingAttachmentInfo) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: color_attachments.len() as _,
			pColorAttachments: color_attachments.as_ptr(),
			pDepthAttachment: depth_attachment,
			pStencilAttachment: mem::null(),
		})
	}

	pub const fn with_colors_and_stencil(render_area: &core::VkRect2D, color_attachments: &[core::VkRenderingAttachmentInfo], stencil_attachment: &core::VkRenderingAttachmentInfo) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: color_attachments.len() as _,
			pColorAttachments: color_attachments.as_ptr(),
			pDepthAttachment: mem::null(),
			pStencilAttachment: stencil_attachment,
		})
	}

	pub const fn with_depth(render_area: &core::VkRect2D, depth_attachment: &core::VkRenderingAttachmentInfo) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: 0,
			pColorAttachments: mem::null(),
			pDepthAttachment: depth_attachment,
			pStencilAttachment: mem::null(),
		})
	}

	pub const fn with_stencil(render_area: &core::VkRect2D, stencil_attachment: &core::VkRenderingAttachmentInfo) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: 0,
			pColorAttachments: mem::null(),
			pDepthAttachment: mem::null(),
			pStencilAttachment: stencil_attachment,
		})
	}

	pub const fn with_depth_and_stencil(render_area: &core::VkRect2D, depth_attachment: &core::VkRenderingAttachmentInfo, stencil_attachment: &core::VkRenderingAttachmentInfo) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: 0,
			pColorAttachments: mem::null(),
			pDepthAttachment: depth_attachment,
			pStencilAttachment: stencil_attachment,
		})
	}

	pub const fn with_all(
		render_area: &core::VkRect2D,
		color_attachments: &[core::VkRenderingAttachmentInfo],
		depth_attachment: &core::VkRenderingAttachmentInfo,
		stencil_attachment: &core::VkRenderingAttachmentInfo,
	) -> Self {
		Self(core::VkRenderingInfo {
			sType: core::VK_STRUCTURE_TYPE_RENDERING_INFO,
			pNext: mem::null(),
			flags: 0,
			renderArea: *render_area,
			layerCount: 1,
			viewMask: 0,
			colorAttachmentCount: color_attachments.len() as _,
			pColorAttachments: color_attachments.as_ptr(),
			pDepthAttachment: depth_attachment,
			pStencilAttachment: stencil_attachment,
		})
	}
}

impl PipelineRenderingCreateInfo {
	#[inline]
	pub const fn new(color_attachments: &[core::VkFormat], depth_attachment: core::VkFormat, stencil_attachment: core::VkFormat) -> Self {
		Self(core::VkPipelineRenderingCreateInfo {
			sType: core::VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO,
			pNext: mem::null(),
			viewMask: 0,
			colorAttachmentCount: color_attachments.len() as _,
			pColorAttachmentFormats: color_attachments.as_ptr(),
			depthAttachmentFormat: depth_attachment,
			stencilAttachmentFormat: stencil_attachment,
		})
	}
}

impl PhysicalDeviceDynamicRenderingFeatures {}

impl CommandBufferInheritanceRenderingInfo {}

impl PhysicalDeviceShaderIntegerDotProductFeatures {}

impl PhysicalDeviceShaderIntegerDotProductProperties {}

impl PhysicalDeviceTexelBufferAlignmentProperties {}

impl FormatProperties3 {}

impl PhysicalDeviceMaintenance4Features {}

impl PhysicalDeviceMaintenance4Properties {}

impl DeviceBufferMemoryRequirements {}

impl DeviceImageMemoryRequirements {}

impl SurfaceCapabilitiesKHR {}

impl SurfaceFormatKHR {
	#[inline]
	pub const fn undefined() -> Self {
		Self(core::VkSurfaceFormatKHR {
			format: core::VK_FORMAT_UNDEFINED,
			colorSpace: core::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR,
		})
	}
}

impl SwapchainCreateInfoKHR {
	#[inline]
	pub const fn new() -> Self {
		Self(core::VkSwapchainCreateInfoKHR {
			sType: core::VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
			pNext: mem::null(),
			flags: 0,
			surface: mem::null(),
			minImageCount: 0,
			imageFormat: core::VK_FORMAT_B8G8R8A8_UNORM,
			imageColorSpace: core::VK_COLORSPACE_SRGB_NONLINEAR_KHR,

			imageExtent: core::VkExtent2D {
				width: 0,
				height: 0,
			},

			imageArrayLayers: 1,
			imageUsage: core::VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
			imageSharingMode: core::VK_SHARING_MODE_EXCLUSIVE,
			queueFamilyIndexCount: 0,
			pQueueFamilyIndices: mem::null(),
			preTransform: core::VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
			compositeAlpha: core::VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR,
			presentMode: core::VK_PRESENT_MODE_FIFO_KHR,
			clipped: core::VK_TRUE,
			oldSwapchain: mem::null(),
		})
	}
}

impl PresentInfoKHR {
	#[inline]
	pub const fn new(image_indices: &[u32], swapchains: &[core::VkSwapchainKHR], wait_semaphores: &[core::VkSemaphore]) -> Self {
		Self(core::VkPresentInfoKHR {
			sType: core::VK_STRUCTURE_TYPE_PRESENT_INFO_KHR,
			pNext: mem::null(),
			waitSemaphoreCount: wait_semaphores.len() as _,
			pWaitSemaphores: wait_semaphores.as_ptr(),
			swapchainCount: swapchains.len() as _,
			pSwapchains: swapchains.as_ptr(),
			pImageIndices: image_indices.as_ptr(),
			pResults: mem::null(),
		})
	}
}

impl ImageSwapchainCreateInfoKHR {}

impl BindImageMemorySwapchainInfoKHR {}

impl AcquireNextImageInfoKHR {}

impl DeviceGroupPresentCapabilitiesKHR {}

impl DeviceGroupPresentInfoKHR {}

impl DeviceGroupSwapchainCreateInfoKHR {}

impl DisplayModeParametersKHR {}

impl DisplayModeCreateInfoKHR {}

impl DisplayModePropertiesKHR {}

impl DisplayPlaneCapabilitiesKHR {}

impl DisplayPlanePropertiesKHR {}

impl DisplayPropertiesKHR {}

impl DisplaySurfaceCreateInfoKHR {}

impl DisplayPresentInfoKHR {}

impl RenderingFragmentShadingRateAttachmentInfoKHR {}

impl RenderingFragmentDensityMapAttachmentInfoEXT {}

impl AttachmentSampleCountInfoAMD {}

impl MultiviewPerViewAttributesInfoNVX {}

impl ImportMemoryFdInfoKHR {}

impl MemoryFdPropertiesKHR {}

impl MemoryGetFdInfoKHR {}

impl ImportSemaphoreFdInfoKHR {}

impl SemaphoreGetFdInfoKHR {}

impl PhysicalDevicePushDescriptorPropertiesKHR {}

impl RectLayerKHR {}

impl PresentRegionKHR {}

impl PresentRegionsKHR {}

impl SharedPresentSurfaceCapabilitiesKHR {}

impl ImportFenceFdInfoKHR {}

impl FenceGetFdInfoKHR {}

impl PhysicalDevicePerformanceQueryFeaturesKHR {}

impl PhysicalDevicePerformanceQueryPropertiesKHR {}

impl PerformanceCounterKHR {}

impl PerformanceCounterDescriptionKHR {}

impl QueryPoolPerformanceCreateInfoKHR {}

impl AcquireProfilingLockInfoKHR {}

impl PerformanceQuerySubmitInfoKHR {}

impl PhysicalDeviceSurfaceInfo2KHR {}

impl SurfaceCapabilities2KHR {}

impl SurfaceFormat2KHR {}

impl DisplayProperties2KHR {}

impl DisplayPlaneProperties2KHR {}

impl DisplayModeProperties2KHR {}

impl DisplayPlaneInfo2KHR {}

impl DisplayPlaneCapabilities2KHR {}

impl PhysicalDeviceShaderClockFeaturesKHR {}

impl DeviceQueueGlobalPriorityCreateInfoKHR {}

impl PhysicalDeviceGlobalPriorityQueryFeaturesKHR {}

impl QueueFamilyGlobalPriorityPropertiesKHR {}

impl FragmentShadingRateAttachmentInfoKHR {}

impl PipelineFragmentShadingRateStateCreateInfoKHR {}

impl PhysicalDeviceFragmentShadingRateFeaturesKHR {}

impl PhysicalDeviceFragmentShadingRatePropertiesKHR {}

impl PhysicalDeviceFragmentShadingRateKHR {}

impl SurfaceProtectedCapabilitiesKHR {}

impl PhysicalDevicePresentWaitFeaturesKHR {}

impl PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}

impl PipelineInfoKHR {}

impl PipelineExecutablePropertiesKHR {}

impl PipelineExecutableInfoKHR {}

impl PipelineExecutableStatisticKHR {}

impl PipelineExecutableInternalRepresentationKHR {}

impl PipelineLibraryCreateInfoKHR {}

impl PresentIdKHR {}

impl PhysicalDevicePresentIdFeaturesKHR {}

impl QueueFamilyCheckpointProperties2NV {}

impl CheckpointData2NV {}

impl PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {}

impl PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {}

impl PipelineRasterizationStateRasterizationOrderAMD {}

impl DebugMarkerObjectNameInfoEXT {}

impl DebugMarkerObjectTagInfoEXT {}

impl DebugMarkerMarkerInfoEXT {}

impl DedicatedAllocationImageCreateInfoNV {}

impl DedicatedAllocationBufferCreateInfoNV {}

impl DedicatedAllocationMemoryAllocateInfoNV {}

impl PhysicalDeviceTransformFeedbackFeaturesEXT {}

impl PhysicalDeviceTransformFeedbackPropertiesEXT {}

impl PipelineRasterizationStateStreamCreateInfoEXT {}

impl CuModuleCreateInfoNVX {}

impl CuFunctionCreateInfoNVX {}

impl CuLaunchInfoNVX {}

impl ImageViewHandleInfoNVX {}

impl ImageViewAddressPropertiesNVX {}

impl TextureLODGatherFormatPropertiesAMD {}

impl ShaderResourceUsageAMD {}

impl ShaderStatisticsInfoAMD {}

impl PhysicalDeviceCornerSampledImageFeaturesNV {}

impl ExternalImageFormatPropertiesNV {}

impl ExternalMemoryImageCreateInfoNV {}

impl ExportMemoryAllocateInfoNV {}

impl ValidationFlagsEXT {}

impl ImageViewASTCDecodeModeEXT {}

impl PhysicalDeviceASTCDecodeFeaturesEXT {}

impl ConditionalRenderingBeginInfoEXT {}

impl PhysicalDeviceConditionalRenderingFeaturesEXT {}

impl CommandBufferInheritanceConditionalRenderingInfoEXT {}

impl ViewportWScalingNV {}

impl PipelineViewportWScalingStateCreateInfoNV {}

impl SurfaceCapabilities2EXT {}

impl DisplayPowerInfoEXT {}

impl DeviceEventInfoEXT {}

impl DisplayEventInfoEXT {}

impl SwapchainCounterCreateInfoEXT {}

impl RefreshCycleDurationGOOGLE {}

impl PastPresentationTimingGOOGLE {}

impl PresentTimeGOOGLE {}

impl PresentTimesInfoGOOGLE {}

impl PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {}

impl ViewportSwizzleNV {}

impl PipelineViewportSwizzleStateCreateInfoNV {}

impl PhysicalDeviceDiscardRectanglePropertiesEXT {}

impl PipelineDiscardRectangleStateCreateInfoEXT {}

impl PhysicalDeviceConservativeRasterizationPropertiesEXT {}

impl PipelineRasterizationConservativeStateCreateInfoEXT {}

impl PhysicalDeviceDepthClipEnableFeaturesEXT {}

impl PipelineRasterizationDepthClipStateCreateInfoEXT {}

impl XYColorEXT {}

impl HdrMetadataEXT {}

impl DebugUtilsLabelEXT {}

impl DebugUtilsObjectNameInfoEXT {}

impl DebugUtilsMessengerCallbackDataEXT {}

impl DebugUtilsObjectTagInfoEXT {}

impl SampleLocationEXT {}

impl SampleLocationsInfoEXT {}

impl AttachmentSampleLocationsEXT {}

impl SubpassSampleLocationsEXT {}

impl RenderPassSampleLocationsBeginInfoEXT {}

impl PipelineSampleLocationsStateCreateInfoEXT {}

impl PhysicalDeviceSampleLocationsPropertiesEXT {}

impl MultisamplePropertiesEXT {}

impl PhysicalDeviceBlendOperationAdvancedFeaturesEXT {}

impl PhysicalDeviceBlendOperationAdvancedPropertiesEXT {}

impl PipelineColorBlendAdvancedStateCreateInfoEXT {}

impl PipelineCoverageToColorStateCreateInfoNV {}

impl PipelineCoverageModulationStateCreateInfoNV {}

impl PhysicalDeviceShaderSMBuiltinsPropertiesNV {}

impl PhysicalDeviceShaderSMBuiltinsFeaturesNV {}

impl DrmFormatModifierPropertiesEXT {}

impl DrmFormatModifierPropertiesListEXT {}

impl PhysicalDeviceImageDrmFormatModifierInfoEXT {}

impl ImageDrmFormatModifierListCreateInfoEXT {}

impl ImageDrmFormatModifierExplicitCreateInfoEXT {}

impl ImageDrmFormatModifierPropertiesEXT {}

impl DrmFormatModifierProperties2EXT {}

impl DrmFormatModifierPropertiesList2EXT {}

impl ValidationCacheCreateInfoEXT {}

impl ShaderModuleValidationCacheCreateInfoEXT {}

impl ShadingRatePaletteNV {}

impl PipelineViewportShadingRateImageStateCreateInfoNV {}

impl PhysicalDeviceShadingRateImageFeaturesNV {}

impl PhysicalDeviceShadingRateImagePropertiesNV {}

impl CoarseSampleLocationNV {}

impl CoarseSampleOrderCustomNV {}

impl PipelineViewportCoarseSampleOrderStateCreateInfoNV {}

impl RayTracingShaderGroupCreateInfoNV {}

impl RayTracingPipelineCreateInfoNV {}

impl GeometryTrianglesNV {}

impl GeometryAABBNV {}

impl GeometryDataNV {}

impl GeometryNV {}

impl AccelerationStructureInfoNV {}

impl AccelerationStructureCreateInfoNV {}

impl BindAccelerationStructureMemoryInfoNV {}

impl WriteDescriptorSetAccelerationStructureNV {}

impl AccelerationStructureMemoryRequirementsInfoNV {}

impl PhysicalDeviceRayTracingPropertiesNV {}

impl TransformMatrixKHR {}

impl AabbPositionsKHR {}

impl AccelerationStructureInstanceKHR {}

impl PhysicalDeviceRepresentativeFragmentTestFeaturesNV {}

impl PipelineRepresentativeFragmentTestStateCreateInfoNV {}

impl PhysicalDeviceImageViewImageFormatInfoEXT {}

impl FilterCubicImageViewImageFormatPropertiesEXT {}

impl ImportMemoryHostPointerInfoEXT {}

impl MemoryHostPointerPropertiesEXT {}

impl PhysicalDeviceExternalMemoryHostPropertiesEXT {}

impl PipelineCompilerControlCreateInfoAMD {}

impl CalibratedTimestampInfoEXT {}

impl PhysicalDeviceShaderCorePropertiesAMD {}

impl DeviceMemoryOverallocationCreateInfoAMD {}

impl PhysicalDeviceVertexAttributeDivisorPropertiesEXT {}

impl VertexInputBindingDivisorDescriptionEXT {}

impl PipelineVertexInputDivisorStateCreateInfoEXT {}

impl PhysicalDeviceVertexAttributeDivisorFeaturesEXT {}

impl PhysicalDeviceComputeShaderDerivativesFeaturesNV {}

impl PhysicalDeviceMeshShaderFeaturesNV {}

impl PhysicalDeviceMeshShaderPropertiesNV {}

impl DrawMeshTasksIndirectCommandNV {}

impl PhysicalDeviceFragmentShaderBarycentricFeaturesNV {}

impl PhysicalDeviceShaderImageFootprintFeaturesNV {}

impl PipelineViewportExclusiveScissorStateCreateInfoNV {}

impl PhysicalDeviceExclusiveScissorFeaturesNV {}

impl QueueFamilyCheckpointPropertiesNV {}

impl CheckpointDataNV {}

impl PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}

impl PerformanceValueINTEL {}

impl InitializePerformanceApiInfoINTEL {}

impl QueryPoolPerformanceQueryCreateInfoINTEL {}

impl PerformanceMarkerInfoINTEL {}

impl PerformanceStreamMarkerInfoINTEL {}

impl PerformanceOverrideInfoINTEL {}

impl PerformanceConfigurationAcquireInfoINTEL {}

impl PhysicalDevicePCIBusInfoPropertiesEXT {}

impl DisplayNativeHdrSurfaceCapabilitiesAMD {}

impl SwapchainDisplayNativeHdrCreateInfoAMD {}

impl PhysicalDeviceFragmentDensityMapFeaturesEXT {}

impl PhysicalDeviceFragmentDensityMapPropertiesEXT {}

impl RenderPassFragmentDensityMapCreateInfoEXT {}

impl PhysicalDeviceShaderCoreProperties2AMD {}

impl PhysicalDeviceCoherentMemoryFeaturesAMD {}

impl PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {}

impl PhysicalDeviceMemoryBudgetPropertiesEXT {}

impl PhysicalDeviceMemoryPriorityFeaturesEXT {}

impl MemoryPriorityAllocateInfoEXT {}

impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}

impl PhysicalDeviceBufferDeviceAddressFeaturesEXT {}

impl BufferDeviceAddressCreateInfoEXT {}

impl ValidationFeaturesEXT {}

impl CooperativeMatrixPropertiesNV {}

impl PhysicalDeviceCooperativeMatrixFeaturesNV {}

impl PhysicalDeviceCooperativeMatrixPropertiesNV {}

impl PhysicalDeviceCoverageReductionModeFeaturesNV {}

impl PipelineCoverageReductionStateCreateInfoNV {}

impl FramebufferMixedSamplesCombinationNV {}

impl PhysicalDeviceFragmentShaderInterlockFeaturesEXT {}

impl PhysicalDeviceYcbcrImageArraysFeaturesEXT {}

impl PhysicalDeviceProvokingVertexFeaturesEXT {}

impl PhysicalDeviceProvokingVertexPropertiesEXT {}

impl PipelineRasterizationProvokingVertexStateCreateInfoEXT {}

impl HeadlessSurfaceCreateInfoEXT {}

impl PhysicalDeviceLineRasterizationFeaturesEXT {}

impl PhysicalDeviceLineRasterizationPropertiesEXT {}

impl PipelineRasterizationLineStateCreateInfoEXT {}

impl PhysicalDeviceShaderAtomicFloatFeaturesEXT {}

impl PhysicalDeviceIndexTypeUint8FeaturesEXT {}

impl PhysicalDeviceExtendedDynamicStateFeaturesEXT {}

impl PhysicalDeviceShaderAtomicFloat2FeaturesEXT {}

impl PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {}

impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {}

impl GraphicsShaderGroupCreateInfoNV {}

impl GraphicsPipelineShaderGroupsCreateInfoNV {}

impl BindShaderGroupIndirectCommandNV {}

impl BindIndexBufferIndirectCommandNV {}

impl BindVertexBufferIndirectCommandNV {}

impl SetStateFlagsIndirectCommandNV {}

impl IndirectCommandsStreamNV {}

impl IndirectCommandsLayoutTokenNV {}

impl IndirectCommandsLayoutCreateInfoNV {}

impl GeneratedCommandsInfoNV {}

impl GeneratedCommandsMemoryRequirementsInfoNV {}

impl PhysicalDeviceInheritedViewportScissorFeaturesNV {}

impl CommandBufferInheritanceViewportScissorInfoNV {}

impl PhysicalDeviceTexelBufferAlignmentFeaturesEXT {}

impl RenderPassTransformBeginInfoQCOM {}

impl CommandBufferInheritanceRenderPassTransformInfoQCOM {}

impl PhysicalDeviceDeviceMemoryReportFeaturesEXT {}

impl DeviceMemoryReportCallbackDataEXT {}

impl PhysicalDeviceRobustness2FeaturesEXT {}

impl PhysicalDeviceRobustness2PropertiesEXT {}

impl SamplerCustomBorderColorCreateInfoEXT {}

impl PhysicalDeviceCustomBorderColorPropertiesEXT {}

impl PhysicalDeviceCustomBorderColorFeaturesEXT {}

impl PhysicalDeviceDiagnosticsConfigFeaturesNV {}

impl DeviceDiagnosticsConfigCreateInfoNV {}

impl PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {}

impl PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {}

impl GraphicsPipelineLibraryCreateInfoEXT {}

impl PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {}

impl PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {}

impl PipelineFragmentShadingRateEnumStateCreateInfoNV {}

impl AccelerationStructureGeometryMotionTrianglesDataNV {}

impl AccelerationStructureMotionInfoNV {}

impl AccelerationStructureMatrixMotionInstanceNV {}

impl SRTDataNV {}

impl AccelerationStructureSRTMotionInstanceNV {}

impl AccelerationStructureMotionInstanceNV {}

impl PhysicalDeviceRayTracingMotionBlurFeaturesNV {}

impl PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {}

impl PhysicalDeviceFragmentDensityMap2FeaturesEXT {}

impl PhysicalDeviceFragmentDensityMap2PropertiesEXT {}

impl CopyCommandTransformInfoQCOM {}

impl PhysicalDevice4444FormatsFeaturesEXT {}

impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {}

impl PhysicalDeviceRGBA10X6FormatsFeaturesEXT {}

impl PhysicalDeviceMutableDescriptorTypeFeaturesVALVE {}

impl MutableDescriptorTypeListVALVE {}

impl MutableDescriptorTypeCreateInfoVALVE {}

impl PhysicalDeviceVertexInputDynamicStateFeaturesEXT {}

impl VertexInputBindingDescription2EXT {}

impl VertexInputAttributeDescription2EXT {}

impl PhysicalDeviceDrmPropertiesEXT {}

impl PhysicalDeviceDepthClipControlFeaturesEXT {}

impl PipelineViewportDepthClipControlCreateInfoEXT {}

impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {}

impl SubpassShadingPipelineCreateInfoHUAWEI {}

impl PhysicalDeviceSubpassShadingFeaturesHUAWEI {}

impl PhysicalDeviceSubpassShadingPropertiesHUAWEI {}

impl PhysicalDeviceInvocationMaskFeaturesHUAWEI {}

impl MemoryGetRemoteAddressInfoNV {}

impl PhysicalDeviceExternalMemoryRDMAFeaturesNV {}

impl PhysicalDeviceExtendedDynamicState2FeaturesEXT {}

impl PhysicalDeviceColorWriteEnableFeaturesEXT {}

impl PipelineColorWriteCreateInfoEXT {}

impl PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {}

impl PhysicalDeviceImageViewMinLodFeaturesEXT {}

impl ImageViewMinLodCreateInfoEXT {}

impl PhysicalDeviceMultiDrawFeaturesEXT {}

impl PhysicalDeviceMultiDrawPropertiesEXT {}

impl MultiDrawInfoEXT {}

impl MultiDrawIndexedInfoEXT {}

impl PhysicalDeviceImage2DViewOf3DFeaturesEXT {}

impl PhysicalDeviceBorderColorSwizzleFeaturesEXT {}

impl SamplerBorderColorComponentMappingCreateInfoEXT {}

impl PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {}

impl PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {}

impl DescriptorSetBindingReferenceVALVE {}

impl DescriptorSetLayoutHostMappingInfoVALVE {}

impl PhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {}

impl PhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {}

impl SubpassFragmentDensityMapOffsetEndInfoQCOM {}

impl PhysicalDeviceLinearColorAttachmentFeaturesNV {}

impl AccelerationStructureBuildRangeInfoKHR {}

impl AccelerationStructureGeometryTrianglesDataKHR {}

impl AccelerationStructureGeometryAabbsDataKHR {}

impl AccelerationStructureGeometryInstancesDataKHR {}

impl AccelerationStructureGeometryKHR {}

impl AccelerationStructureBuildGeometryInfoKHR {}

impl AccelerationStructureCreateInfoKHR {}

impl WriteDescriptorSetAccelerationStructureKHR {}

impl PhysicalDeviceAccelerationStructureFeaturesKHR {}

impl PhysicalDeviceAccelerationStructurePropertiesKHR {}

impl AccelerationStructureDeviceAddressInfoKHR {}

impl AccelerationStructureVersionInfoKHR {}

impl CopyAccelerationStructureToMemoryInfoKHR {}

impl CopyMemoryToAccelerationStructureInfoKHR {}

impl CopyAccelerationStructureInfoKHR {}

impl AccelerationStructureBuildSizesInfoKHR {}

impl RayTracingShaderGroupCreateInfoKHR {}

impl RayTracingPipelineInterfaceCreateInfoKHR {}

impl RayTracingPipelineCreateInfoKHR {}

impl PhysicalDeviceRayTracingPipelineFeaturesKHR {}

impl PhysicalDeviceRayTracingPipelinePropertiesKHR {}

impl StridedDeviceAddressRegionKHR {}

impl TraceRaysIndirectCommandKHR {}

impl PhysicalDeviceRayQueryFeaturesKHR {}

impl ClearColorValue {
	#[inline]
	pub const fn as_f32x4(color: [f32; 4]) -> Self {
		Self(core::VkClearColorValue {
			float32: color,
		})
	}

	#[inline]
	pub const fn as_i32x4(color: [i32; 4]) -> Self {
		Self(core::VkClearColorValue {
			int32: color,
		})
	}

	#[inline]
	pub const fn as_u32x4(color: [u32; 4]) -> Self {
		Self(core::VkClearColorValue {
			uint32: color,
		})
	}
}

impl ClearValue {
	#[inline]
	pub const fn for_color(float32: [f32; 4]) -> Self {
		Self(core::VkClearValue {
			color: core::VkClearColorValue {
				float32,
			},
		})
	}

	#[inline]
	pub const fn for_depth(depth: f32) -> Self {
		Self(core::VkClearValue {
			depthStencil: core::VkClearDepthStencilValue {
				depth,
				stencil: 0,
			},
		})
	}

	#[inline]
	pub const fn for_stencil(stencil: u32) -> Self {
		Self(core::VkClearValue {
			depthStencil: core::VkClearDepthStencilValue {
				depth: 1.0,
				stencil,
			},
		})
	}

	#[inline]
	pub const fn for_depth_and_stencil(depth: f32, stencil: u32) -> Self {
		Self(core::VkClearValue {
			depthStencil: core::VkClearDepthStencilValue {
				depth,
				stencil,
			},
		})
	}
}

impl PerformanceCounterResultKHR {}

impl PipelineExecutableStatisticValueKHR {}

impl PerformanceValueDataINTEL {}

impl DeviceOrHostAddressConstKHR {}

impl AccelerationStructureMotionInstanceDataNV {}

impl DeviceOrHostAddressKHR {}

impl AccelerationStructureGeometryDataKHR {}
