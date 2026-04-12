use ::core::{
	cmp, //
	iter::Iterator,
	marker,
	ops,
};

use crate::{
	ffi::unix::x11, //
	mem,
	spec,
};

use super::{
	core, //
	utils,
	wayland as wl,
	xcb,
};

//
// get_proc_addr helper macro:
//

macro_rules! get_proc_addr {
	($fn_loader:path, $fn_name:literal) => {{
		unsafe {
			let fn_addr = ($fn_loader)(mem::null(), crate::ffi::c_str($fn_name));

			if fn_addr.is_null() {
				crate::panic!("Unable to load the Vulkan command {}", $fn_name);
			}

			crate::mem::transmute(fn_addr)
		}
	}};

	($fn_loader:path, $fn_handler:expr, $fn_name:literal) => {{
		unsafe {
			let fn_addr = ($fn_loader)($fn_handler, crate::ffi::c_str($fn_name));

			if fn_addr.is_null() {
				crate::panic!("Unable to load the Vulkan command {}", $fn_name);
			}

			crate::mem::transmute(fn_addr)
		}
	}};
}

//
// InstanceExtensionName and DeviceExtensionName:
//

pub struct InstanceExtensionName;

pub struct DeviceExtensionName;

impl InstanceExtensionName {
	pub const COUNT: usize = 6;

	pub const SURFACE: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_SURFACE_EXTENSION_NAME) };

	pub const XCB_SURFACE: &str = unsafe { str::from_utf8_unchecked(xcb::VK_KHR_XCB_SURFACE_EXTENSION_NAME) };

	pub const WAYLAND_SURFACE: &str = unsafe { str::from_utf8_unchecked(wl::VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME) };

	pub const HEADLESS_SURFACE: &str = unsafe { str::from_utf8_unchecked(core::VK_EXT_HEADLESS_SURFACE_EXTENSION_NAME) };

	pub const DEBUG_UTILS: &str = unsafe { str::from_utf8_unchecked(core::VK_EXT_DEBUG_UTILS_EXTENSION_NAME) };

	pub const DISPLAY: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_DISPLAY_EXTENSION_NAME) };
}

impl DeviceExtensionName {
	pub const COUNT: usize = 4;

	pub const SWAPCHAIN: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_SWAPCHAIN_EXTENSION_NAME) };

	pub const PUSH_DESCRIPTOR: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_PUSH_DESCRIPTOR_EXTENSION_NAME) };

	pub const RAY_TRACING_PIPELINE: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_RAY_TRACING_PIPELINE_EXTENSION_NAME) };

	pub const ACCELERATION_STRUCTURE: &str = unsafe { str::from_utf8_unchecked(core::VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME) };
}

//
// AllocationCallbacks:
//

#[derive(Debug, Clone)]
pub enum AllocationCallbacks {
	None,
	Some(core::VkAllocationCallbacks),
}

impl AllocationCallbacks {
	#[inline]
	pub const fn as_ptr(&self) -> *const core::VkAllocationCallbacks {
		match self {
			Self::None => mem::null(),

			Self::Some(callbacks) => callbacks,
		}
	}
}

//
// Loader:
//

pub struct Loader {
	lib: crate::DynamicLibrary,

	pub get_instance_proc_addr: core::PFN_vkGetInstanceProcAddr,

	pub create_instance: core::PFN_vkCreateInstance,

	pub enumerate_instance_extension_properties: core::PFN_vkEnumerateInstanceExtensionProperties,

	pub enumerate_instance_layer_properties: core::PFN_vkEnumerateInstanceLayerProperties,

	pub enumerate_instance_version: core::PFN_vkEnumerateInstanceVersion,
}

impl Loader {
	pub fn new(filename: &str) -> spec::Result<Self> {
		let mut lib = crate::DynamicLibrary::new();

		lib.load(filename)?;

		let fn_addr = lib.find_symbol("vkGetInstanceProcAddr\0")?;

		let get_instance_proc_addr: core::PFN_vkGetInstanceProcAddr = unsafe { mem::transmute(fn_addr) };

		spec::Result::Ok(Self {
			lib,

			get_instance_proc_addr,

			create_instance: get_proc_addr!(get_instance_proc_addr, "vkCreateInstance\0"),

			enumerate_instance_extension_properties: get_proc_addr!(get_instance_proc_addr, "vkEnumerateInstanceExtensionProperties\0"),

			enumerate_instance_layer_properties: get_proc_addr!(get_instance_proc_addr, "vkEnumerateInstanceLayerProperties\0"),

			enumerate_instance_version: get_proc_addr!(get_instance_proc_addr, "vkEnumerateInstanceVersion\0"),
		})
	}

	#[inline(always)]
	pub fn get_instance_proc_addr<Fn: 'static>(&self, instance: core::VkInstance) -> core::PFN_vkVoidFunction {
		unsafe { (self.get_instance_proc_addr)(instance, utils::fn_typename::<Fn>()) }
	}

	#[inline(always)]
	pub fn create_instance(&self, create_info: &core::VkInstanceCreateInfo, allocator: &AllocationCallbacks, instance: &mut core::VkInstance) -> core::VkResult {
		unsafe { (self.create_instance)(create_info, allocator.as_ptr(), instance) }
	}

	pub fn enumerate_instance_extension_properties(&self, list: &mut [core::VkExtensionProperties]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.enumerate_instance_extension_properties)(mem::null(), &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as _, total);

			let result = (self.enumerate_instance_extension_properties)(mem::null(), &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	pub fn enumerate_instance_layer_properties(&self, list: &mut [core::VkLayerProperties]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.enumerate_instance_layer_properties)(&mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.enumerate_instance_layer_properties)(&mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	#[inline(always)]
	pub fn enumerate_instance_version(&self, api_version: &mut u32) -> core::VkResult {
		unsafe { (self.enumerate_instance_version)(api_version) }
	}

	pub fn load_instance_table(&self, instance: core::VkInstance) -> InstanceFnTable {
		let get_instance_proc_addr = self.get_instance_proc_addr;

		InstanceFnTable {
			destroy_instance: get_proc_addr!(get_instance_proc_addr, instance, "vkDestroyInstance\0"),

			create_device: get_proc_addr!(get_instance_proc_addr, instance, "vkCreateDevice\0"),

			get_device_proc_addr: get_proc_addr!(get_instance_proc_addr, instance, "vkGetDeviceProcAddr\0"),

			enumerate_physical_devices: get_proc_addr!(get_instance_proc_addr, instance, "vkEnumeratePhysicalDevices\0"),

			enumerate_physical_device_groups: get_proc_addr!(get_instance_proc_addr, instance, "vkEnumeratePhysicalDeviceGroups\0"),

			enumerate_device_extension_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkEnumerateDeviceExtensionProperties\0"),

			enumerate_device_layer_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkEnumerateDeviceLayerProperties\0"),

			get_physical_device_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceProperties\0"),

			get_physical_device_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceProperties2\0"),

			get_physical_device_features: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceFeatures\0"),

			get_physical_device_features2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceFeatures2\0"),

			get_physical_device_memory_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceMemoryProperties\0"),

			get_physical_device_memory_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceMemoryProperties2\0"),

			get_physical_device_queue_family_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceQueueFamilyProperties\0"),

			get_physical_device_queue_family_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceQueueFamilyProperties2\0"),

			get_physical_device_format_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceFormatProperties\0"),

			get_physical_device_format_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceFormatProperties2\0"),

			get_physical_device_image_format_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceImageFormatProperties\0"),

			get_physical_device_image_format_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceImageFormatProperties2\0"),

			get_physical_device_sparse_image_format_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceSparseImageFormatProperties\0"),

			get_physical_device_sparse_image_format_properties2: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceSparseImageFormatProperties2\0"),

			get_physical_device_external_buffer_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceExternalBufferProperties\0"),

			get_physical_device_external_fence_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceExternalFenceProperties\0"),

			get_physical_device_external_semaphore_properties: get_proc_addr!(get_instance_proc_addr, instance, "vkGetPhysicalDeviceExternalSemaphoreProperties\0"),
		}
	}
}

//
// InstanceFnTable:
//

pub struct InstanceFnTable {
	pub destroy_instance: core::PFN_vkDestroyInstance,

	pub create_device: core::PFN_vkCreateDevice,

	// NOTE: This is the bootstrap function used to construct DeviceFnTable and
	// therefore belongs here despite its first argument being of type VkDevice.
	pub get_device_proc_addr: core::PFN_vkGetDeviceProcAddr,

	pub enumerate_physical_devices: core::PFN_vkEnumeratePhysicalDevices,

	pub enumerate_physical_device_groups: core::PFN_vkEnumeratePhysicalDeviceGroups,

	pub enumerate_device_extension_properties: core::PFN_vkEnumerateDeviceExtensionProperties,

	pub enumerate_device_layer_properties: core::PFN_vkEnumerateDeviceLayerProperties,

	pub get_physical_device_properties: core::PFN_vkGetPhysicalDeviceProperties,

	pub get_physical_device_properties2: core::PFN_vkGetPhysicalDeviceProperties2,

	pub get_physical_device_features: core::PFN_vkGetPhysicalDeviceFeatures,

	pub get_physical_device_features2: core::PFN_vkGetPhysicalDeviceFeatures2,

	pub get_physical_device_memory_properties: core::PFN_vkGetPhysicalDeviceMemoryProperties,

	pub get_physical_device_memory_properties2: core::PFN_vkGetPhysicalDeviceMemoryProperties2,

	pub get_physical_device_queue_family_properties: core::PFN_vkGetPhysicalDeviceQueueFamilyProperties,

	pub get_physical_device_queue_family_properties2: core::PFN_vkGetPhysicalDeviceQueueFamilyProperties2,

	pub get_physical_device_format_properties: core::PFN_vkGetPhysicalDeviceFormatProperties,

	pub get_physical_device_format_properties2: core::PFN_vkGetPhysicalDeviceFormatProperties2,

	pub get_physical_device_image_format_properties: core::PFN_vkGetPhysicalDeviceImageFormatProperties,

	pub get_physical_device_image_format_properties2: core::PFN_vkGetPhysicalDeviceImageFormatProperties2,

	pub get_physical_device_sparse_image_format_properties: core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties,

	pub get_physical_device_sparse_image_format_properties2: core::PFN_vkGetPhysicalDeviceSparseImageFormatProperties2,

	pub get_physical_device_external_buffer_properties: core::PFN_vkGetPhysicalDeviceExternalBufferProperties,

	pub get_physical_device_external_fence_properties: core::PFN_vkGetPhysicalDeviceExternalFenceProperties,

	pub get_physical_device_external_semaphore_properties: core::PFN_vkGetPhysicalDeviceExternalSemaphoreProperties,
}

impl InstanceFnTable {
	#[inline(always)]
	pub fn destroy_instance(&self, instance: core::VkInstance, allocator: &AllocationCallbacks) {
		unsafe { (self.destroy_instance)(instance, allocator.as_ptr()) }
	}

	#[inline(always)]
	pub fn create_device(&self, physical_device: core::VkPhysicalDevice, create_info: &core::VkDeviceCreateInfo, allocator: &AllocationCallbacks, device: &mut core::VkDevice) -> core::VkResult {
		unsafe { (self.create_device)(physical_device, create_info, allocator.as_ptr(), device) }
	}

	#[inline(always)]
	pub fn get_device_proc_addr<Fn: 'static>(&self, device: core::VkDevice) -> core::PFN_vkVoidFunction {
		unsafe { (self.get_device_proc_addr)(device, utils::fn_typename::<Fn>()) }
	}

	pub fn enumerate_physical_devices(&self, instance: core::VkInstance, list: &mut [core::VkPhysicalDevice]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.enumerate_physical_devices)(instance, &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.enumerate_physical_devices)(instance, &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	pub fn enumerate_device_extension_properties(&self, physical_device: core::VkPhysicalDevice, list: &mut [core::VkExtensionProperties]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.enumerate_device_extension_properties)(physical_device, mem::null(), &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.enumerate_device_extension_properties)(physical_device, mem::null(), &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	#[inline(always)]
	pub fn get_physical_device_properties(&self, physical_device: core::VkPhysicalDevice, properties: &mut core::VkPhysicalDeviceProperties) {
		unsafe { (self.get_physical_device_properties)(physical_device, properties) }
	}

	#[inline(always)]
	pub fn get_physical_device_features(&self, physical_device: core::VkPhysicalDevice, features: &mut core::VkPhysicalDeviceFeatures) {
		unsafe { (self.get_physical_device_features)(physical_device, features) }
	}

	#[inline(always)]
	pub fn get_physical_device_memory_properties(&self, physical_device: core::VkPhysicalDevice, properties: &mut core::VkPhysicalDeviceMemoryProperties) {
		unsafe { (self.get_physical_device_memory_properties)(physical_device, properties) }
	}

	pub fn get_physical_device_queue_family_properties(&self, physical_device: core::VkPhysicalDevice, list: &mut [core::VkQueueFamilyProperties]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			(self.get_physical_device_queue_family_properties)(physical_device, &mut total, mem::null());

			if total > 0 {
				let mut count = cmp::min(list.len() as u32, total);

				(self.get_physical_device_queue_family_properties)(physical_device, &mut count, list.as_mut_ptr());

				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	#[inline(always)]
	pub fn get_physical_device_format_properties(&self, physical_device: core::VkPhysicalDevice, format: core::VkFormat, properties: &mut core::VkFormatProperties) {
		unsafe { (self.get_physical_device_format_properties)(physical_device, format, properties) }
	}

	pub fn select_queue_family_indices(&self, physical_device: core::VkPhysicalDevice) -> [u32; 3] {
		let mut family_index = [u32::MAX; 3];

		let mut family_list: [core::VkQueueFamilyProperties; 16] = unsafe { mem::zeroed() };

		let (family_count, _) = self.get_physical_device_queue_family_properties(physical_device, &mut family_list[..]);

		//
		// 1st pass to find a dedicated queue family for graphics:
		//

		for n in 0..family_count {
			let family = &family_list[n as usize];

			if utils::has_graphics_queue(family) && !utils::has_compute_queue(family) {
				family_index[0] = n;
				break;
			}
		}

		//
		// 2nd pass to find a general queue family for graphics as fallback:
		//

		if family_index[0] == u32::MAX {
			for n in 0..family_count {
				let family = &family_list[n as usize];

				if utils::has_graphics_queue(family) {
					family_index[0] = n;
					break;
				}
			}
		}

		if family_index[0] == u32::MAX {
			crate::panic!("Unable to find a queue family for graphics in the device");
		}

		//
		// 3rd pass to find a dedicated queue family for compute:
		//

		for n in 0..family_count {
			let family = &family_list[n as usize];

			if !utils::has_graphics_queue(family) && utils::has_compute_queue(family) {
				family_index[1] = n;
				break;
			}
		}

		//
		// 4th pass to find a general and unused queue family for compute as fallback:
		//

		if family_index[1] == u32::MAX {
			for n in 0..family_count {
				let family = &family_list[n as usize];

				if utils::has_compute_queue(family) && (n != family_index[0]) {
					family_index[1] = n;
					break;
				}
			}
		}

		if family_index[1] == u32::MAX {
			// NOTE: The last compute family fallback is the graphics one.
			family_index[1] = family_index[0];
		}

		//
		// 5th pass to find a dedicated queue family for transfer (DMA-like):
		//

		for n in 0..family_count {
			let family = &family_list[n as usize];

			if !utils::has_graphics_queue(family) && !utils::has_compute_queue(family) && utils::has_transfer_queue(family) {
				family_index[2] = n;
				break;
			}
		}

		//
		// 6th pass to find a general and unused queue family for transfer as fallback:
		//

		if family_index[2] == u32::MAX {
			for n in 0..family_count {
				let family = &family_list[n as usize];

				if utils::has_transfer_queue(family) && (n != family_index[0]) && (n != family_index[1]) {
					family_index[2] = n;
					break;
				}
			}
		}

		if family_index[2] == u32::MAX {
			// NOTE: The last transfer family fallback is the compute one.
			family_index[2] = family_index[1];
		}

		family_index
	}

	pub fn load_swapchain_table_unchecked(&self, device: core::VkDevice) -> SwapchainFnTable {
		let get_device_proc_addr = self.get_device_proc_addr;

		SwapchainFnTable {
			extension_name: DeviceExtensionName::SWAPCHAIN,

			create_swapchain_khr: get_proc_addr!(get_device_proc_addr, device, "vkCreateSwapchainKHR\0"),

			destroy_swapchain_khr: get_proc_addr!(get_device_proc_addr, device, "vkDestroySwapchainKHR\0"),

			get_swapchain_images_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetSwapchainImagesKHR\0"),

			acquire_next_image_khr: get_proc_addr!(get_device_proc_addr, device, "vkAcquireNextImageKHR\0"),

			acquire_next_image2_khr: get_proc_addr!(get_device_proc_addr, device, "vkAcquireNextImage2KHR\0"),

			queue_present_khr: get_proc_addr!(get_device_proc_addr, device, "vkQueuePresentKHR\0"),

			get_swapchain_status_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetSwapchainStatusKHR\0"),
		}
	}

	pub fn load_push_descriptor_table_unchecked(&self, device: core::VkDevice) -> PushDescriptorFnTable {
		let get_device_proc_addr = self.get_device_proc_addr;

		PushDescriptorFnTable {
			extension_name: DeviceExtensionName::PUSH_DESCRIPTOR,

			cmd_push_descriptor_set_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdPushDescriptorSetKHR\0"),

			cmd_push_descriptor_set_with_template_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdPushDescriptorSetWithTemplateKHR\0"),
		}
	}

	pub fn load_ray_tracing_pipeline_table_unchecked(&self, device: core::VkDevice) -> RayTracingPipelineFnTable {
		let get_device_proc_addr = self.get_device_proc_addr;

		RayTracingPipelineFnTable {
			extension_name: DeviceExtensionName::RAY_TRACING_PIPELINE,

			create_ray_tracing_pipelines_khr: get_proc_addr!(get_device_proc_addr, device, "vkCreateRayTracingPipelinesKHR\0"),

			get_ray_tracing_shader_group_handles_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetRayTracingShaderGroupHandlesKHR\0"),

			get_ray_tracing_capture_replay_shader_group_handles_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetRayTracingCaptureReplayShaderGroupHandlesKHR\0"),

			get_ray_tracing_shader_group_stack_size_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetRayTracingShaderGroupStackSizeKHR\0"),

			cmd_trace_rays_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdTraceRaysKHR\0"),

			cmd_trace_rays_indirect_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdTraceRaysIndirectKHR\0"),

			cmd_set_ray_tracing_pipeline_stack_size_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetRayTracingPipelineStackSizeKHR\0"),
		}
	}

	pub fn load_acceleration_structure_table_unchecked(&self, device: core::VkDevice) -> AccelerationStructureFnTable {
		let get_device_proc_addr = self.get_device_proc_addr;

		AccelerationStructureFnTable {
			extension_name: DeviceExtensionName::ACCELERATION_STRUCTURE,

			create_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkCreateAccelerationStructureKHR\0"),

			destroy_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkDestroyAccelerationStructureKHR\0"),

			build_acceleration_structures_khr: get_proc_addr!(get_device_proc_addr, device, "vkBuildAccelerationStructuresKHR\0"),

			cmd_build_acceleration_structures_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdBuildAccelerationStructuresKHR\0"),

			cmd_build_acceleration_structures_indirect_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdBuildAccelerationStructuresIndirectKHR\0"),

			copy_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkCopyAccelerationStructureKHR\0"),

			copy_acceleration_structure_to_memory_khr: get_proc_addr!(get_device_proc_addr, device, "vkCopyAccelerationStructureToMemoryKHR\0"),

			copy_memory_to_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkCopyMemoryToAccelerationStructureKHR\0"),

			cmd_copy_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyAccelerationStructureKHR\0"),

			cmd_copy_acceleration_structure_to_memory_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyAccelerationStructureToMemoryKHR\0"),

			cmd_copy_memory_to_acceleration_structure_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyMemoryToAccelerationStructureKHR\0"),

			write_acceleration_structures_properties_khr: get_proc_addr!(get_device_proc_addr, device, "vkWriteAccelerationStructuresPropertiesKHR\0"),

			cmd_write_acceleration_structures_properties_khr: get_proc_addr!(get_device_proc_addr, device, "vkCmdWriteAccelerationStructuresPropertiesKHR\0"),

			get_acceleration_structure_build_sizes_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetAccelerationStructureBuildSizesKHR\0"),

			get_acceleration_structure_device_address_khr: get_proc_addr!(get_device_proc_addr, device, "vkGetAccelerationStructureDeviceAddressKHR\0"),
		}
	}

	pub fn load_extensions_unchecked(&self, device: core::VkDevice, name_list: &[&str], extension_list: &mut [super::DeviceExtension]) {
		crate::panic_if!(name_list.len() != extension_list.len());

		for (n, name) in name_list.iter().enumerate() {
			let name = *name;

			if name == DeviceExtensionName::SWAPCHAIN {
				let table = self.load_swapchain_table_unchecked(device);

				extension_list[n] = super::DeviceExtension::Swapchain(table);
			}
			//
			else if name == DeviceExtensionName::PUSH_DESCRIPTOR {
				let table = self.load_push_descriptor_table_unchecked(device);

				extension_list[n] = super::DeviceExtension::PushDescriptor(table);
			}
			//
			else if name == DeviceExtensionName::RAY_TRACING_PIPELINE {
				let table = self.load_ray_tracing_pipeline_table_unchecked(device);

				extension_list[n] = super::DeviceExtension::RayTracingPipeline(table);
			}
			//
			else if name == DeviceExtensionName::ACCELERATION_STRUCTURE {
				let table = self.load_acceleration_structure_table_unchecked(device);

				extension_list[n] = super::DeviceExtension::AccelerationStructure(table);
			}
			//
			else {
				extension_list[n] = super::DeviceExtension::None;
			}
		}
	}

	pub fn load_device_table(&self, device: core::VkDevice) -> DeviceFnTable {
		let get_device_proc_addr = self.get_device_proc_addr;

		DeviceFnTable {
			destroy_device: get_proc_addr!(get_device_proc_addr, device, "vkDestroyDevice\0"),

			get_device_queue: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceQueue\0"),

			get_device_queue2: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceQueue2\0"),

			device_wait_idle: get_proc_addr!(get_device_proc_addr, device, "vkDeviceWaitIdle\0"),

			allocate_memory: get_proc_addr!(get_device_proc_addr, device, "vkAllocateMemory\0"),

			free_memory: get_proc_addr!(get_device_proc_addr, device, "vkFreeMemory\0"),

			map_memory: get_proc_addr!(get_device_proc_addr, device, "vkMapMemory\0"),

			unmap_memory: get_proc_addr!(get_device_proc_addr, device, "vkUnmapMemory\0"),

			flush_mapped_memory_ranges: get_proc_addr!(get_device_proc_addr, device, "vkFlushMappedMemoryRanges\0"),

			invalidate_mapped_memory_ranges: get_proc_addr!(get_device_proc_addr, device, "vkInvalidateMappedMemoryRanges\0"),

			get_device_memory_commitment: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceMemoryCommitment\0"),

			get_device_memory_opaque_capture_address: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceMemoryOpaqueCaptureAddress\0"),

			create_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCreateBuffer\0"),

			destroy_buffer: get_proc_addr!(get_device_proc_addr, device, "vkDestroyBuffer\0"),

			get_buffer_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetBufferMemoryRequirements\0"),

			get_buffer_memory_requirements2: get_proc_addr!(get_device_proc_addr, device, "vkGetBufferMemoryRequirements2\0"),

			bind_buffer_memory: get_proc_addr!(get_device_proc_addr, device, "vkBindBufferMemory\0"),

			bind_buffer_memory2: get_proc_addr!(get_device_proc_addr, device, "vkBindBufferMemory2\0"),

			get_buffer_device_address: get_proc_addr!(get_device_proc_addr, device, "vkGetBufferDeviceAddress\0"),

			get_buffer_opaque_capture_address: get_proc_addr!(get_device_proc_addr, device, "vkGetBufferOpaqueCaptureAddress\0"),

			create_buffer_view: get_proc_addr!(get_device_proc_addr, device, "vkCreateBufferView\0"),

			destroy_buffer_view: get_proc_addr!(get_device_proc_addr, device, "vkDestroyBufferView\0"),

			create_image: get_proc_addr!(get_device_proc_addr, device, "vkCreateImage\0"),

			destroy_image: get_proc_addr!(get_device_proc_addr, device, "vkDestroyImage\0"),

			get_image_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetImageMemoryRequirements\0"),

			get_image_memory_requirements2: get_proc_addr!(get_device_proc_addr, device, "vkGetImageMemoryRequirements2\0"),

			bind_image_memory: get_proc_addr!(get_device_proc_addr, device, "vkBindImageMemory\0"),

			bind_image_memory2: get_proc_addr!(get_device_proc_addr, device, "vkBindImageMemory2\0"),

			get_image_subresource_layout: get_proc_addr!(get_device_proc_addr, device, "vkGetImageSubresourceLayout\0"),

			get_image_sparse_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetImageSparseMemoryRequirements\0"),

			get_image_sparse_memory_requirements2: get_proc_addr!(get_device_proc_addr, device, "vkGetImageSparseMemoryRequirements2\0"),

			create_image_view: get_proc_addr!(get_device_proc_addr, device, "vkCreateImageView\0"),

			destroy_image_view: get_proc_addr!(get_device_proc_addr, device, "vkDestroyImageView\0"),

			create_shader_module: get_proc_addr!(get_device_proc_addr, device, "vkCreateShaderModule\0"),

			destroy_shader_module: get_proc_addr!(get_device_proc_addr, device, "vkDestroyShaderModule\0"),

			create_graphics_pipelines: get_proc_addr!(get_device_proc_addr, device, "vkCreateGraphicsPipelines\0"),

			create_compute_pipelines: get_proc_addr!(get_device_proc_addr, device, "vkCreateComputePipelines\0"),

			destroy_pipeline: get_proc_addr!(get_device_proc_addr, device, "vkDestroyPipeline\0"),

			create_pipeline_cache: get_proc_addr!(get_device_proc_addr, device, "vkCreatePipelineCache\0"),

			destroy_pipeline_cache: get_proc_addr!(get_device_proc_addr, device, "vkDestroyPipelineCache\0"),

			get_pipeline_cache_data: get_proc_addr!(get_device_proc_addr, device, "vkGetPipelineCacheData\0"),

			merge_pipeline_caches: get_proc_addr!(get_device_proc_addr, device, "vkMergePipelineCaches\0"),

			create_pipeline_layout: get_proc_addr!(get_device_proc_addr, device, "vkCreatePipelineLayout\0"),

			destroy_pipeline_layout: get_proc_addr!(get_device_proc_addr, device, "vkDestroyPipelineLayout\0"),

			create_render_pass: get_proc_addr!(get_device_proc_addr, device, "vkCreateRenderPass\0"),

			create_render_pass2: get_proc_addr!(get_device_proc_addr, device, "vkCreateRenderPass2\0"),

			destroy_render_pass: get_proc_addr!(get_device_proc_addr, device, "vkDestroyRenderPass\0"),

			get_render_area_granularity: get_proc_addr!(get_device_proc_addr, device, "vkGetRenderAreaGranularity\0"),

			create_framebuffer: get_proc_addr!(get_device_proc_addr, device, "vkCreateFramebuffer\0"),

			destroy_framebuffer: get_proc_addr!(get_device_proc_addr, device, "vkDestroyFramebuffer\0"),

			create_descriptor_set_layout: get_proc_addr!(get_device_proc_addr, device, "vkCreateDescriptorSetLayout\0"),

			destroy_descriptor_set_layout: get_proc_addr!(get_device_proc_addr, device, "vkDestroyDescriptorSetLayout\0"),

			get_descriptor_set_layout_support: get_proc_addr!(get_device_proc_addr, device, "vkGetDescriptorSetLayoutSupport\0"),

			create_descriptor_pool: get_proc_addr!(get_device_proc_addr, device, "vkCreateDescriptorPool\0"),

			destroy_descriptor_pool: get_proc_addr!(get_device_proc_addr, device, "vkDestroyDescriptorPool\0"),

			reset_descriptor_pool: get_proc_addr!(get_device_proc_addr, device, "vkResetDescriptorPool\0"),

			allocate_descriptor_sets: get_proc_addr!(get_device_proc_addr, device, "vkAllocateDescriptorSets\0"),

			free_descriptor_sets: get_proc_addr!(get_device_proc_addr, device, "vkFreeDescriptorSets\0"),

			update_descriptor_sets: get_proc_addr!(get_device_proc_addr, device, "vkUpdateDescriptorSets\0"),

			create_descriptor_update_template: get_proc_addr!(get_device_proc_addr, device, "vkCreateDescriptorUpdateTemplate\0"),

			destroy_descriptor_update_template: get_proc_addr!(get_device_proc_addr, device, "vkDestroyDescriptorUpdateTemplate\0"),

			update_descriptor_set_with_template: get_proc_addr!(get_device_proc_addr, device, "vkUpdateDescriptorSetWithTemplate\0"),

			create_command_pool: get_proc_addr!(get_device_proc_addr, device, "vkCreateCommandPool\0"),

			destroy_command_pool: get_proc_addr!(get_device_proc_addr, device, "vkDestroyCommandPool\0"),

			reset_command_pool: get_proc_addr!(get_device_proc_addr, device, "vkResetCommandPool\0"),

			trim_command_pool: get_proc_addr!(get_device_proc_addr, device, "vkTrimCommandPool\0"),

			allocate_command_buffers: get_proc_addr!(get_device_proc_addr, device, "vkAllocateCommandBuffers\0"),

			free_command_buffers: get_proc_addr!(get_device_proc_addr, device, "vkFreeCommandBuffers\0"),

			begin_command_buffer: get_proc_addr!(get_device_proc_addr, device, "vkBeginCommandBuffer\0"),

			end_command_buffer: get_proc_addr!(get_device_proc_addr, device, "vkEndCommandBuffer\0"),

			reset_command_buffer: get_proc_addr!(get_device_proc_addr, device, "vkResetCommandBuffer\0"),

			queue_submit: get_proc_addr!(get_device_proc_addr, device, "vkQueueSubmit\0"),

			queue_submit2: get_proc_addr!(get_device_proc_addr, device, "vkQueueSubmit2\0"),

			queue_wait_idle: get_proc_addr!(get_device_proc_addr, device, "vkQueueWaitIdle\0"),

			queue_bind_sparse: get_proc_addr!(get_device_proc_addr, device, "vkQueueBindSparse\0"),

			create_fence: get_proc_addr!(get_device_proc_addr, device, "vkCreateFence\0"),

			destroy_fence: get_proc_addr!(get_device_proc_addr, device, "vkDestroyFence\0"),

			reset_fences: get_proc_addr!(get_device_proc_addr, device, "vkResetFences\0"),

			get_fence_status: get_proc_addr!(get_device_proc_addr, device, "vkGetFenceStatus\0"),

			wait_for_fences: get_proc_addr!(get_device_proc_addr, device, "vkWaitForFences\0"),

			create_semaphore: get_proc_addr!(get_device_proc_addr, device, "vkCreateSemaphore\0"),

			destroy_semaphore: get_proc_addr!(get_device_proc_addr, device, "vkDestroySemaphore\0"),

			get_semaphore_counter_value: get_proc_addr!(get_device_proc_addr, device, "vkGetSemaphoreCounterValue\0"),

			wait_semaphores: get_proc_addr!(get_device_proc_addr, device, "vkWaitSemaphores\0"),

			signal_semaphore: get_proc_addr!(get_device_proc_addr, device, "vkSignalSemaphore\0"),

			create_event: get_proc_addr!(get_device_proc_addr, device, "vkCreateEvent\0"),

			destroy_event: get_proc_addr!(get_device_proc_addr, device, "vkDestroyEvent\0"),

			get_event_status: get_proc_addr!(get_device_proc_addr, device, "vkGetEventStatus\0"),

			set_event: get_proc_addr!(get_device_proc_addr, device, "vkSetEvent\0"),

			reset_event: get_proc_addr!(get_device_proc_addr, device, "vkResetEvent\0"),

			create_query_pool: get_proc_addr!(get_device_proc_addr, device, "vkCreateQueryPool\0"),

			destroy_query_pool: get_proc_addr!(get_device_proc_addr, device, "vkDestroyQueryPool\0"),

			get_query_pool_results: get_proc_addr!(get_device_proc_addr, device, "vkGetQueryPoolResults\0"),

			reset_query_pool: get_proc_addr!(get_device_proc_addr, device, "vkResetQueryPool\0"),

			create_sampler: get_proc_addr!(get_device_proc_addr, device, "vkCreateSampler\0"),

			destroy_sampler: get_proc_addr!(get_device_proc_addr, device, "vkDestroySampler\0"),

			create_sampler_ycbcr_conversion: get_proc_addr!(get_device_proc_addr, device, "vkCreateSamplerYcbcrConversion\0"),

			destroy_sampler_ycbcr_conversion: get_proc_addr!(get_device_proc_addr, device, "vkDestroySamplerYcbcrConversion\0"),

			cmd_draw: get_proc_addr!(get_device_proc_addr, device, "vkCmdDraw\0"),

			cmd_draw_indexed: get_proc_addr!(get_device_proc_addr, device, "vkCmdDrawIndexed\0"),

			cmd_draw_indirect: get_proc_addr!(get_device_proc_addr, device, "vkCmdDrawIndirect\0"),

			cmd_draw_indexed_indirect: get_proc_addr!(get_device_proc_addr, device, "vkCmdDrawIndexedIndirect\0"),

			cmd_draw_indirect_count: get_proc_addr!(get_device_proc_addr, device, "vkCmdDrawIndirectCount\0"),

			cmd_draw_indexed_indirect_count: get_proc_addr!(get_device_proc_addr, device, "vkCmdDrawIndexedIndirectCount\0"),

			cmd_dispatch: get_proc_addr!(get_device_proc_addr, device, "vkCmdDispatch\0"),

			cmd_dispatch_indirect: get_proc_addr!(get_device_proc_addr, device, "vkCmdDispatchIndirect\0"),

			cmd_dispatch_base: get_proc_addr!(get_device_proc_addr, device, "vkCmdDispatchBase\0"),

			cmd_copy_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyBuffer\0"),

			cmd_copy_buffer2: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyBuffer2\0"),

			cmd_copy_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyImage\0"),

			cmd_copy_image2: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyImage2\0"),

			cmd_copy_buffer_to_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyBufferToImage\0"),

			cmd_copy_buffer_to_image2: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyBufferToImage2\0"),

			cmd_copy_image_to_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyImageToBuffer\0"),

			cmd_copy_image_to_buffer2: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyImageToBuffer2\0"),

			cmd_blit_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdBlitImage\0"),

			cmd_blit_image2: get_proc_addr!(get_device_proc_addr, device, "vkCmdBlitImage2\0"),

			cmd_resolve_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdResolveImage\0"),

			cmd_resolve_image2: get_proc_addr!(get_device_proc_addr, device, "vkCmdResolveImage2\0"),

			cmd_update_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCmdUpdateBuffer\0"),

			cmd_fill_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCmdFillBuffer\0"),

			cmd_clear_color_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdClearColorImage\0"),

			cmd_clear_depth_stencil_image: get_proc_addr!(get_device_proc_addr, device, "vkCmdClearDepthStencilImage\0"),

			cmd_clear_attachments: get_proc_addr!(get_device_proc_addr, device, "vkCmdClearAttachments\0"),

			cmd_begin_render_pass: get_proc_addr!(get_device_proc_addr, device, "vkCmdBeginRenderPass\0"),

			cmd_begin_render_pass2: get_proc_addr!(get_device_proc_addr, device, "vkCmdBeginRenderPass2\0"),

			cmd_next_subpass: get_proc_addr!(get_device_proc_addr, device, "vkCmdNextSubpass\0"),

			cmd_next_subpass2: get_proc_addr!(get_device_proc_addr, device, "vkCmdNextSubpass2\0"),

			cmd_end_render_pass: get_proc_addr!(get_device_proc_addr, device, "vkCmdEndRenderPass\0"),

			cmd_end_render_pass2: get_proc_addr!(get_device_proc_addr, device, "vkCmdEndRenderPass2\0"),

			cmd_begin_rendering: get_proc_addr!(get_device_proc_addr, device, "vkCmdBeginRendering\0"),

			cmd_end_rendering: get_proc_addr!(get_device_proc_addr, device, "vkCmdEndRendering\0"),

			cmd_bind_pipeline: get_proc_addr!(get_device_proc_addr, device, "vkCmdBindPipeline\0"),

			cmd_bind_descriptor_sets: get_proc_addr!(get_device_proc_addr, device, "vkCmdBindDescriptorSets\0"),

			cmd_bind_index_buffer: get_proc_addr!(get_device_proc_addr, device, "vkCmdBindIndexBuffer\0"),

			cmd_bind_vertex_buffers: get_proc_addr!(get_device_proc_addr, device, "vkCmdBindVertexBuffers\0"),

			cmd_bind_vertex_buffers2: get_proc_addr!(get_device_proc_addr, device, "vkCmdBindVertexBuffers2\0"),

			cmd_push_constants: get_proc_addr!(get_device_proc_addr, device, "vkCmdPushConstants\0"),

			cmd_set_viewport: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetViewport\0"),

			cmd_set_viewport_with_count: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetViewportWithCount\0"),

			cmd_set_scissor: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetScissor\0"),

			cmd_set_scissor_with_count: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetScissorWithCount\0"),

			cmd_set_line_width: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetLineWidth\0"),

			cmd_set_depth_bias: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthBias\0"),

			cmd_set_depth_bias_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthBiasEnable\0"),

			cmd_set_depth_bounds: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthBounds\0"),

			cmd_set_depth_bounds_test_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthBoundsTestEnable\0"),

			cmd_set_depth_compare_op: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthCompareOp\0"),

			cmd_set_depth_test_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthTestEnable\0"),

			cmd_set_depth_write_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetDepthWriteEnable\0"),

			cmd_set_stencil_compare_mask: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetStencilCompareMask\0"),

			cmd_set_stencil_write_mask: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetStencilWriteMask\0"),

			cmd_set_stencil_reference: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetStencilReference\0"),

			cmd_set_stencil_test_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetStencilTestEnable\0"),

			cmd_set_stencil_op: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetStencilOp\0"),

			cmd_set_blend_constants: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetBlendConstants\0"),

			cmd_set_cull_mode: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetCullMode\0"),

			cmd_set_front_face: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetFrontFace\0"),

			cmd_set_primitive_topology: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetPrimitiveTopology\0"),

			cmd_set_primitive_restart_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetPrimitiveRestartEnable\0"),

			cmd_set_rasterizer_discard_enable: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetRasterizerDiscardEnable\0"),

			cmd_pipeline_barrier: get_proc_addr!(get_device_proc_addr, device, "vkCmdPipelineBarrier\0"),

			cmd_pipeline_barrier2: get_proc_addr!(get_device_proc_addr, device, "vkCmdPipelineBarrier2\0"),

			cmd_set_event: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetEvent\0"),

			cmd_set_event2: get_proc_addr!(get_device_proc_addr, device, "vkCmdSetEvent2\0"),

			cmd_reset_event: get_proc_addr!(get_device_proc_addr, device, "vkCmdResetEvent\0"),

			cmd_reset_event2: get_proc_addr!(get_device_proc_addr, device, "vkCmdResetEvent2\0"),

			cmd_wait_events: get_proc_addr!(get_device_proc_addr, device, "vkCmdWaitEvents\0"),

			cmd_wait_events2: get_proc_addr!(get_device_proc_addr, device, "vkCmdWaitEvents2\0"),

			cmd_begin_query: get_proc_addr!(get_device_proc_addr, device, "vkCmdBeginQuery\0"),

			cmd_end_query: get_proc_addr!(get_device_proc_addr, device, "vkCmdEndQuery\0"),

			cmd_reset_query_pool: get_proc_addr!(get_device_proc_addr, device, "vkCmdResetQueryPool\0"),

			cmd_write_timestamp: get_proc_addr!(get_device_proc_addr, device, "vkCmdWriteTimestamp\0"),

			cmd_write_timestamp2: get_proc_addr!(get_device_proc_addr, device, "vkCmdWriteTimestamp2\0"),

			cmd_copy_query_pool_results: get_proc_addr!(get_device_proc_addr, device, "vkCmdCopyQueryPoolResults\0"),

			cmd_execute_commands: get_proc_addr!(get_device_proc_addr, device, "vkCmdExecuteCommands\0"),

			create_private_data_slot: get_proc_addr!(get_device_proc_addr, device, "vkCreatePrivateDataSlot\0"),

			destroy_private_data_slot: get_proc_addr!(get_device_proc_addr, device, "vkDestroyPrivateDataSlot\0"),

			set_private_data: get_proc_addr!(get_device_proc_addr, device, "vkSetPrivateData\0"),

			get_private_data: get_proc_addr!(get_device_proc_addr, device, "vkGetPrivateData\0"),

			get_device_image_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceImageMemoryRequirements\0"),

			get_device_buffer_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceBufferMemoryRequirements\0"),

			get_device_image_sparse_memory_requirements: get_proc_addr!(get_device_proc_addr, device, "vkGetDeviceImageSparseMemoryRequirements\0"),
		}
	}
}

//
// Tables for instance-level extensions:
//
// SurfaceFnTable:
//

#[derive(Clone)]
pub struct SurfaceFnTable {
	pub extension_name: &'static str,

	pub destroy_surface_khr: core::PFN_vkDestroySurfaceKHR,

	pub get_physical_device_surface_capabilities_khr: core::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,

	pub get_physical_device_surface_formats_khr: core::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,

	pub get_physical_device_surface_present_modes_khr: core::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,

	pub get_physical_device_surface_support_khr: core::PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
}

impl SurfaceFnTable {
	#[inline(always)]
	pub fn destroy_surface(&self, instance: core::VkInstance, surface: core::VkSurfaceKHR, allocator: &AllocationCallbacks) {
		unsafe { (self.destroy_surface_khr)(instance, surface, allocator.as_ptr()) }
	}

	#[inline(always)]
	pub fn get_physical_device_surface_capabilities(&self, physical_device: core::VkPhysicalDevice, surface: core::VkSurfaceKHR, capabilities: &mut core::VkSurfaceCapabilitiesKHR) -> core::VkResult {
		unsafe { (self.get_physical_device_surface_capabilities_khr)(physical_device, surface, capabilities) }
	}

	pub fn get_physical_device_surface_formats(&self, physical_device: core::VkPhysicalDevice, surface: core::VkSurfaceKHR, list: &mut [core::VkSurfaceFormatKHR]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.get_physical_device_surface_formats_khr)(physical_device, surface, &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.get_physical_device_surface_formats_khr)(physical_device, surface, &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	pub fn get_physical_device_surface_present_modes(&self, physical_device: core::VkPhysicalDevice, surface: core::VkSurfaceKHR, list: &mut [core::VkPresentModeKHR]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.get_physical_device_surface_present_modes_khr)(physical_device, surface, &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.get_physical_device_surface_present_modes_khr)(physical_device, surface, &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	#[inline(always)]
	pub fn get_physical_device_surface_support(&self, physical_device: core::VkPhysicalDevice, queue_family_index: u32, surface: core::VkSurfaceKHR, support: &mut bool) -> core::VkResult {
		unsafe { (self.get_physical_device_surface_support_khr)(physical_device, queue_family_index, surface, (support as *mut _) as *mut core::VkBool32) }
	}
}

//
// XcbSurfaceFnTable:
//

pub struct XcbSurfaceFnTable {
	pub extension_name: &'static str,

	pub create_xcb_surface_khr: xcb::PFN_vkCreateXcbSurfaceKHR,

	pub get_physical_device_xcb_presentation_support_khr: xcb::PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
}

impl XcbSurfaceFnTable {
	#[inline(always)]
	pub fn create_xcb_surface(&self, instance: core::VkInstance, create_info: &xcb::VkXcbSurfaceCreateInfoKHR, allocator: &AllocationCallbacks, surface: &mut core::VkSurfaceKHR) -> core::VkResult {
		unsafe { (self.create_xcb_surface_khr)(instance, create_info, allocator.as_ptr(), surface) }
	}

	#[inline(always)]
	pub fn get_physical_device_xcb_presentation_support(
		&self,
		physical_device: core::VkPhysicalDevice,
		queue_family_index: u32,
		connection: *mut x11::xcb_connection_t,
		visual_id: x11::xproto::xcb_visualid_t,
	) -> bool {
		unsafe { (self.get_physical_device_xcb_presentation_support_khr)(physical_device, queue_family_index, connection, visual_id) != 0 }
	}
}

//
// WaylandSurfaceFnTable:
//

pub struct WaylandSurfaceFnTable {
	pub extension_name: &'static str,

	pub create_wayland_surface_khr: wl::PFN_vkCreateWaylandSurfaceKHR,

	pub get_physical_device_wayland_presentation_support_khr: wl::PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}

impl WaylandSurfaceFnTable {
	#[inline(always)]
	pub fn create_wayland_surface(
		&self,
		instance: core::VkInstance,
		create_info: &wl::VkWaylandSurfaceCreateInfoKHR,
		allocator: &AllocationCallbacks,
		surface: &mut core::VkSurfaceKHR,
	) -> core::VkResult {
		unsafe { (self.create_wayland_surface_khr)(instance, create_info, allocator.as_ptr(), surface) }
	}

	#[inline(always)]
	pub fn get_physical_device_wayland_presentation_support(&self, physical_device: core::VkPhysicalDevice, queue_family_index: u32, display: *mut wl::wl_display) -> bool {
		unsafe { (self.get_physical_device_wayland_presentation_support_khr)(physical_device, queue_family_index, display) != 0 }
	}
}

//
// HeadlessSurfaceFnTable:
//

pub struct HeadlessSurfaceFnTable {
	pub extension_name: &'static str,

	pub create_headless_surface_ext: core::PFN_vkCreateHeadlessSurfaceEXT,
}

//
// DisplayFnTable:
//

pub struct DisplayFnTable {
	pub extension_name: &'static str,

	pub create_display_plane_surface_khr: core::PFN_vkCreateDisplayPlaneSurfaceKHR,
}

//
// DeviceFnTable:
//

pub struct DeviceFnTable {
	// Device management:
	pub destroy_device: core::PFN_vkDestroyDevice,

	pub get_device_queue: core::PFN_vkGetDeviceQueue,

	pub get_device_queue2: core::PFN_vkGetDeviceQueue2,

	pub device_wait_idle: core::PFN_vkDeviceWaitIdle,

	// Memory:
	pub allocate_memory: core::PFN_vkAllocateMemory,

	pub free_memory: core::PFN_vkFreeMemory,

	pub map_memory: core::PFN_vkMapMemory,

	pub unmap_memory: core::PFN_vkUnmapMemory,

	pub flush_mapped_memory_ranges: core::PFN_vkFlushMappedMemoryRanges,

	pub invalidate_mapped_memory_ranges: core::PFN_vkInvalidateMappedMemoryRanges,

	pub get_device_memory_commitment: core::PFN_vkGetDeviceMemoryCommitment,

	pub get_device_memory_opaque_capture_address: core::PFN_vkGetDeviceMemoryOpaqueCaptureAddress,

	// Buffers:
	pub create_buffer: core::PFN_vkCreateBuffer,

	pub destroy_buffer: core::PFN_vkDestroyBuffer,

	pub get_buffer_memory_requirements: core::PFN_vkGetBufferMemoryRequirements,

	pub get_buffer_memory_requirements2: core::PFN_vkGetBufferMemoryRequirements2,

	pub bind_buffer_memory: core::PFN_vkBindBufferMemory,

	pub bind_buffer_memory2: core::PFN_vkBindBufferMemory2,

	pub get_buffer_device_address: core::PFN_vkGetBufferDeviceAddress,

	pub get_buffer_opaque_capture_address: core::PFN_vkGetBufferOpaqueCaptureAddress,

	pub create_buffer_view: core::PFN_vkCreateBufferView,

	pub destroy_buffer_view: core::PFN_vkDestroyBufferView,

	// Images:
	pub create_image: core::PFN_vkCreateImage,

	pub destroy_image: core::PFN_vkDestroyImage,

	pub get_image_memory_requirements: core::PFN_vkGetImageMemoryRequirements,

	pub get_image_memory_requirements2: core::PFN_vkGetImageMemoryRequirements2,

	pub bind_image_memory: core::PFN_vkBindImageMemory,

	pub bind_image_memory2: core::PFN_vkBindImageMemory2,

	pub get_image_subresource_layout: core::PFN_vkGetImageSubresourceLayout,

	pub get_image_sparse_memory_requirements: core::PFN_vkGetImageSparseMemoryRequirements,

	pub get_image_sparse_memory_requirements2: core::PFN_vkGetImageSparseMemoryRequirements2,

	pub create_image_view: core::PFN_vkCreateImageView,

	pub destroy_image_view: core::PFN_vkDestroyImageView,

	// Shaders:
	pub create_shader_module: core::PFN_vkCreateShaderModule,

	pub destroy_shader_module: core::PFN_vkDestroyShaderModule,

	// Pipelines:
	pub create_graphics_pipelines: core::PFN_vkCreateGraphicsPipelines,

	pub create_compute_pipelines: core::PFN_vkCreateComputePipelines,

	pub destroy_pipeline: core::PFN_vkDestroyPipeline,

	pub create_pipeline_cache: core::PFN_vkCreatePipelineCache,

	pub destroy_pipeline_cache: core::PFN_vkDestroyPipelineCache,

	pub get_pipeline_cache_data: core::PFN_vkGetPipelineCacheData,

	pub merge_pipeline_caches: core::PFN_vkMergePipelineCaches,

	pub create_pipeline_layout: core::PFN_vkCreatePipelineLayout,

	pub destroy_pipeline_layout: core::PFN_vkDestroyPipelineLayout,

	// Render passes:
	pub create_render_pass: core::PFN_vkCreateRenderPass,

	pub create_render_pass2: core::PFN_vkCreateRenderPass2,

	pub destroy_render_pass: core::PFN_vkDestroyRenderPass,

	pub get_render_area_granularity: core::PFN_vkGetRenderAreaGranularity,

	// Framebuffers:
	pub create_framebuffer: core::PFN_vkCreateFramebuffer,

	pub destroy_framebuffer: core::PFN_vkDestroyFramebuffer,

	// Descriptor sets:
	pub create_descriptor_set_layout: core::PFN_vkCreateDescriptorSetLayout,

	pub destroy_descriptor_set_layout: core::PFN_vkDestroyDescriptorSetLayout,

	pub get_descriptor_set_layout_support: core::PFN_vkGetDescriptorSetLayoutSupport,

	pub create_descriptor_pool: core::PFN_vkCreateDescriptorPool,

	pub destroy_descriptor_pool: core::PFN_vkDestroyDescriptorPool,

	pub reset_descriptor_pool: core::PFN_vkResetDescriptorPool,

	pub allocate_descriptor_sets: core::PFN_vkAllocateDescriptorSets,

	pub free_descriptor_sets: core::PFN_vkFreeDescriptorSets,

	pub update_descriptor_sets: core::PFN_vkUpdateDescriptorSets,

	pub create_descriptor_update_template: core::PFN_vkCreateDescriptorUpdateTemplate,

	pub destroy_descriptor_update_template: core::PFN_vkDestroyDescriptorUpdateTemplate,

	pub update_descriptor_set_with_template: core::PFN_vkUpdateDescriptorSetWithTemplate,

	// Command pools:
	pub create_command_pool: core::PFN_vkCreateCommandPool,

	pub destroy_command_pool: core::PFN_vkDestroyCommandPool,

	pub reset_command_pool: core::PFN_vkResetCommandPool,

	pub trim_command_pool: core::PFN_vkTrimCommandPool,

	pub allocate_command_buffers: core::PFN_vkAllocateCommandBuffers,

	pub free_command_buffers: core::PFN_vkFreeCommandBuffers,

	// Command buffer recording:
	pub begin_command_buffer: core::PFN_vkBeginCommandBuffer,

	pub end_command_buffer: core::PFN_vkEndCommandBuffer,

	pub reset_command_buffer: core::PFN_vkResetCommandBuffer,

	// Queue operations:
	pub queue_submit: core::PFN_vkQueueSubmit,

	pub queue_submit2: core::PFN_vkQueueSubmit2,

	pub queue_wait_idle: core::PFN_vkQueueWaitIdle,

	pub queue_bind_sparse: core::PFN_vkQueueBindSparse,

	// Synchronization:
	pub create_fence: core::PFN_vkCreateFence,

	pub destroy_fence: core::PFN_vkDestroyFence,

	pub reset_fences: core::PFN_vkResetFences,

	pub get_fence_status: core::PFN_vkGetFenceStatus,

	pub wait_for_fences: core::PFN_vkWaitForFences,

	pub create_semaphore: core::PFN_vkCreateSemaphore,

	pub destroy_semaphore: core::PFN_vkDestroySemaphore,

	pub get_semaphore_counter_value: core::PFN_vkGetSemaphoreCounterValue,

	pub wait_semaphores: core::PFN_vkWaitSemaphores,

	pub signal_semaphore: core::PFN_vkSignalSemaphore,

	pub create_event: core::PFN_vkCreateEvent,

	pub destroy_event: core::PFN_vkDestroyEvent,

	pub get_event_status: core::PFN_vkGetEventStatus,

	pub set_event: core::PFN_vkSetEvent,

	pub reset_event: core::PFN_vkResetEvent,

	// Queries:
	pub create_query_pool: core::PFN_vkCreateQueryPool,

	pub destroy_query_pool: core::PFN_vkDestroyQueryPool,

	pub get_query_pool_results: core::PFN_vkGetQueryPoolResults,

	pub reset_query_pool: core::PFN_vkResetQueryPool,

	// Samplers:
	pub create_sampler: core::PFN_vkCreateSampler,

	pub destroy_sampler: core::PFN_vkDestroySampler,

	pub create_sampler_ycbcr_conversion: core::PFN_vkCreateSamplerYcbcrConversion,

	pub destroy_sampler_ycbcr_conversion: core::PFN_vkDestroySamplerYcbcrConversion,

	// Draw commands:
	pub cmd_draw: core::PFN_vkCmdDraw,

	pub cmd_draw_indexed: core::PFN_vkCmdDrawIndexed,

	pub cmd_draw_indirect: core::PFN_vkCmdDrawIndirect,

	pub cmd_draw_indexed_indirect: core::PFN_vkCmdDrawIndexedIndirect,

	pub cmd_draw_indirect_count: core::PFN_vkCmdDrawIndirectCount,

	pub cmd_draw_indexed_indirect_count: core::PFN_vkCmdDrawIndexedIndirectCount,

	// Dispatch commands:
	pub cmd_dispatch: core::PFN_vkCmdDispatch,

	pub cmd_dispatch_indirect: core::PFN_vkCmdDispatchIndirect,

	pub cmd_dispatch_base: core::PFN_vkCmdDispatchBase,

	// Transfer commands:
	pub cmd_copy_buffer: core::PFN_vkCmdCopyBuffer,

	pub cmd_copy_buffer2: core::PFN_vkCmdCopyBuffer2,

	pub cmd_copy_image: core::PFN_vkCmdCopyImage,

	pub cmd_copy_image2: core::PFN_vkCmdCopyImage2,

	pub cmd_copy_buffer_to_image: core::PFN_vkCmdCopyBufferToImage,

	pub cmd_copy_buffer_to_image2: core::PFN_vkCmdCopyBufferToImage2,

	pub cmd_copy_image_to_buffer: core::PFN_vkCmdCopyImageToBuffer,

	pub cmd_copy_image_to_buffer2: core::PFN_vkCmdCopyImageToBuffer2,

	pub cmd_blit_image: core::PFN_vkCmdBlitImage,

	pub cmd_blit_image2: core::PFN_vkCmdBlitImage2,

	pub cmd_resolve_image: core::PFN_vkCmdResolveImage,

	pub cmd_resolve_image2: core::PFN_vkCmdResolveImage2,

	pub cmd_update_buffer: core::PFN_vkCmdUpdateBuffer,

	pub cmd_fill_buffer: core::PFN_vkCmdFillBuffer,

	// Clear commands:
	pub cmd_clear_color_image: core::PFN_vkCmdClearColorImage,

	pub cmd_clear_depth_stencil_image: core::PFN_vkCmdClearDepthStencilImage,

	pub cmd_clear_attachments: core::PFN_vkCmdClearAttachments,

	// Render pass commands:
	pub cmd_begin_render_pass: core::PFN_vkCmdBeginRenderPass,

	pub cmd_begin_render_pass2: core::PFN_vkCmdBeginRenderPass2,

	pub cmd_next_subpass: core::PFN_vkCmdNextSubpass,

	pub cmd_next_subpass2: core::PFN_vkCmdNextSubpass2,

	pub cmd_end_render_pass: core::PFN_vkCmdEndRenderPass,

	pub cmd_end_render_pass2: core::PFN_vkCmdEndRenderPass2,

	// Dynamic rendering commands (Vulkan 1.3):
	pub cmd_begin_rendering: core::PFN_vkCmdBeginRendering,

	pub cmd_end_rendering: core::PFN_vkCmdEndRendering,

	// Pipeline binding commands:
	pub cmd_bind_pipeline: core::PFN_vkCmdBindPipeline,

	pub cmd_bind_descriptor_sets: core::PFN_vkCmdBindDescriptorSets,

	pub cmd_bind_index_buffer: core::PFN_vkCmdBindIndexBuffer,

	pub cmd_bind_vertex_buffers: core::PFN_vkCmdBindVertexBuffers,

	pub cmd_bind_vertex_buffers2: core::PFN_vkCmdBindVertexBuffers2,

	pub cmd_push_constants: core::PFN_vkCmdPushConstants,

	// Dynamic state commands:
	pub cmd_set_viewport: core::PFN_vkCmdSetViewport,

	pub cmd_set_viewport_with_count: core::PFN_vkCmdSetViewportWithCount,

	pub cmd_set_scissor: core::PFN_vkCmdSetScissor,

	pub cmd_set_scissor_with_count: core::PFN_vkCmdSetScissorWithCount,

	pub cmd_set_line_width: core::PFN_vkCmdSetLineWidth,

	pub cmd_set_depth_bias: core::PFN_vkCmdSetDepthBias,

	pub cmd_set_depth_bias_enable: core::PFN_vkCmdSetDepthBiasEnable,

	pub cmd_set_depth_bounds: core::PFN_vkCmdSetDepthBounds,

	pub cmd_set_depth_bounds_test_enable: core::PFN_vkCmdSetDepthBoundsTestEnable,

	pub cmd_set_depth_compare_op: core::PFN_vkCmdSetDepthCompareOp,

	pub cmd_set_depth_test_enable: core::PFN_vkCmdSetDepthTestEnable,

	pub cmd_set_depth_write_enable: core::PFN_vkCmdSetDepthWriteEnable,

	pub cmd_set_stencil_compare_mask: core::PFN_vkCmdSetStencilCompareMask,

	pub cmd_set_stencil_write_mask: core::PFN_vkCmdSetStencilWriteMask,

	pub cmd_set_stencil_reference: core::PFN_vkCmdSetStencilReference,

	pub cmd_set_stencil_test_enable: core::PFN_vkCmdSetStencilTestEnable,

	pub cmd_set_stencil_op: core::PFN_vkCmdSetStencilOp,

	pub cmd_set_blend_constants: core::PFN_vkCmdSetBlendConstants,

	pub cmd_set_cull_mode: core::PFN_vkCmdSetCullMode,

	pub cmd_set_front_face: core::PFN_vkCmdSetFrontFace,

	pub cmd_set_primitive_topology: core::PFN_vkCmdSetPrimitiveTopology,

	pub cmd_set_primitive_restart_enable: core::PFN_vkCmdSetPrimitiveRestartEnable,

	pub cmd_set_rasterizer_discard_enable: core::PFN_vkCmdSetRasterizerDiscardEnable,

	// Barriers and synchronization commands:
	pub cmd_pipeline_barrier: core::PFN_vkCmdPipelineBarrier,

	pub cmd_pipeline_barrier2: core::PFN_vkCmdPipelineBarrier2,

	pub cmd_set_event: core::PFN_vkCmdSetEvent,

	pub cmd_set_event2: core::PFN_vkCmdSetEvent2,

	pub cmd_reset_event: core::PFN_vkCmdResetEvent,

	pub cmd_reset_event2: core::PFN_vkCmdResetEvent2,

	pub cmd_wait_events: core::PFN_vkCmdWaitEvents,

	pub cmd_wait_events2: core::PFN_vkCmdWaitEvents2,

	// Query commands:
	pub cmd_begin_query: core::PFN_vkCmdBeginQuery,

	pub cmd_end_query: core::PFN_vkCmdEndQuery,

	pub cmd_reset_query_pool: core::PFN_vkCmdResetQueryPool,

	pub cmd_write_timestamp: core::PFN_vkCmdWriteTimestamp,

	pub cmd_write_timestamp2: core::PFN_vkCmdWriteTimestamp2,

	pub cmd_copy_query_pool_results: core::PFN_vkCmdCopyQueryPoolResults,

	// Secondary command buffers:
	pub cmd_execute_commands: core::PFN_vkCmdExecuteCommands,

	// Private data (Vulkan 1.3):
	pub create_private_data_slot: core::PFN_vkCreatePrivateDataSlot,

	pub destroy_private_data_slot: core::PFN_vkDestroyPrivateDataSlot,

	pub set_private_data: core::PFN_vkSetPrivateData,

	pub get_private_data: core::PFN_vkGetPrivateData,

	// Maintenance:
	pub get_device_image_memory_requirements: core::PFN_vkGetDeviceImageMemoryRequirements,

	pub get_device_buffer_memory_requirements: core::PFN_vkGetDeviceBufferMemoryRequirements,

	pub get_device_image_sparse_memory_requirements: core::PFN_vkGetDeviceImageSparseMemoryRequirements,
}

//
// Tables for device-level extensions:
//
// SwapchainFnTable:
//

#[derive(Clone)]
pub struct SwapchainFnTable {
	pub extension_name: &'static str,

	pub create_swapchain_khr: core::PFN_vkCreateSwapchainKHR,

	pub destroy_swapchain_khr: core::PFN_vkDestroySwapchainKHR,

	pub get_swapchain_images_khr: core::PFN_vkGetSwapchainImagesKHR,

	pub acquire_next_image_khr: core::PFN_vkAcquireNextImageKHR,

	pub acquire_next_image2_khr: core::PFN_vkAcquireNextImage2KHR,

	pub queue_present_khr: core::PFN_vkQueuePresentKHR,

	pub get_swapchain_status_khr: core::PFN_vkGetSwapchainStatusKHR,
}

impl SwapchainFnTable {
	#[inline(always)]
	pub fn create_swapchain(&self, device: core::VkDevice, create_info: &core::VkSwapchainCreateInfoKHR, allocator: &AllocationCallbacks, swapchain: &mut core::VkSwapchainKHR) -> core::VkResult {
		unsafe { (self.create_swapchain_khr)(device, create_info, allocator.as_ptr(), swapchain) }
	}

	#[inline(always)]
	pub fn destroy_swapchain(&self, device: core::VkDevice, swapchain: core::VkSwapchainKHR, allocator: &AllocationCallbacks) {
		unsafe { (self.destroy_swapchain_khr)(device, swapchain, allocator.as_ptr()) }
	}

	pub fn get_swapchain_images(&self, device: core::VkDevice, swapchain: core::VkSwapchainKHR, list: &mut [core::VkImage]) -> (u32, u32) {
		let mut total = 0u32;

		unsafe {
			let result = (self.get_swapchain_images_khr)(device, swapchain, &mut total, mem::null());

			if (result != core::VK_SUCCESS) || (total == 0) {
				return (0, 0);
			}

			let mut count = cmp::min(list.len() as u32, total);

			let result = (self.get_swapchain_images_khr)(device, swapchain, &mut count, list.as_mut_ptr());

			if (result == core::VK_SUCCESS) || (result == core::VK_INCOMPLETE) {
				(count, total)
			} else {
				(0, 0)
			}
		}
	}

	#[inline(always)]
	pub fn acquire_next_image(
		&self,
		device: core::VkDevice,
		swapchain: core::VkSwapchainKHR,
		timeout: u64,
		semaphore: core::VkSemaphore,
		fence: core::VkFence,
		image_index: &mut u32,
	) -> core::VkResult {
		unsafe { (self.acquire_next_image_khr)(device, swapchain, timeout, semaphore, fence, image_index) }
	}
}

pub struct PushDescriptorFnTable {
	pub extension_name: &'static str,

	pub cmd_push_descriptor_set_khr: core::PFN_vkCmdPushDescriptorSetKHR,

	pub cmd_push_descriptor_set_with_template_khr: core::PFN_vkCmdPushDescriptorSetWithTemplateKHR,
}
