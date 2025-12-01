use crate::unix::{
	x11, //
	x11::xproto,
};

use super::core;

pub const VK_KHR_XCB_SURFACE_SPEC_VERSION: u32 = 6;
pub const VK_KHR_XCB_SURFACE_EXTENSION_NAME: &[u8; 19] = b"VK_KHR_xcb_surface\0";

pub type VkXcbSurfaceCreateFlagsKHR = core::VkFlags;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VkXcbSurfaceCreateInfoKHR {
	pub sType: core::VkStructureType,
	pub pNext: *const (),
	pub flags: VkXcbSurfaceCreateFlagsKHR,
	pub connection: *mut x11::xcb_connection_t,
	pub window: xproto::xcb_window_t,
}

pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "C" fn(
	instance: core::VkInstance,
	pCreateInfo: *const VkXcbSurfaceCreateInfoKHR,
	pAllocator: *const core::VkAllocationCallbacks,
	pSurface: *mut core::VkSurfaceKHR,
) -> core::VkResult;

pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR =
	unsafe extern "C" fn(physicalDevice: core::VkPhysicalDevice, queueFamilyIndex: u32, connection: *mut x11::xcb_connection_t, visual_id: xproto::xcb_visualid_t) -> core::VkBool32;
