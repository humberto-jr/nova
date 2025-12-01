use super::core;

pub const VK_KHR_WAYLAND_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_WAYLAND_SURFACE_EXTENSION_NAME: &[u8; 23] = b"VK_KHR_wayland_surface\0";

pub type VkWaylandSurfaceCreateFlagsKHR = core::VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_display {
	pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_surface {
	pub _address: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkWaylandSurfaceCreateInfoKHR {
	pub sType: core::VkStructureType,
	pub pNext: *const (),
	pub flags: VkWaylandSurfaceCreateFlagsKHR,
	pub display: *mut wl_display,
	pub surface: *mut wl_surface,
}

pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "C" fn(
	instance: core::VkInstance,
	pCreateInfo: *const VkWaylandSurfaceCreateInfoKHR,
	pAllocator: *const core::VkAllocationCallbacks,
	pSurface: *mut core::VkSurfaceKHR,
) -> core::VkResult;

pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "C" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, display: *mut wl_display) -> core::VkBool32;
