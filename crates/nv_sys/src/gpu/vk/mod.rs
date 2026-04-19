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
