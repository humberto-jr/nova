use ::core::{
	cmp, //
	iter::Iterator,
	marker,
	ops,
};

use crate::{
	mem, //
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
