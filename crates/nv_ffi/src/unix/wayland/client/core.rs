pub use super::super::util::*;

pub const WL_MARSHAL_FLAG_DESTROY: u32 = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct timespec {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_proxy {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_display {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_event_queue {
	_unused: [u8; 0],
}

#[link(name = "wayland-client")]
unsafe extern "C" {
	pub fn wl_event_queue_destroy(queue: *mut wl_event_queue);

	pub fn wl_proxy_marshal_flags(proxy: *mut wl_proxy, opcode: u32, interface: *const wl_interface, version: u32, flags: u32, ...) -> *mut wl_proxy;

	pub fn wl_proxy_marshal_array_flags(proxy: *mut wl_proxy, opcode: u32, interface: *const wl_interface, version: u32, flags: u32, args: *mut wl_argument) -> *mut wl_proxy;

	pub fn wl_proxy_marshal(p: *mut wl_proxy, opcode: u32, ...);

	pub fn wl_proxy_marshal_array(p: *mut wl_proxy, opcode: u32, args: *mut wl_argument);

	pub fn wl_proxy_create(factory: *mut wl_proxy, interface: *const wl_interface) -> *mut wl_proxy;

	pub fn wl_proxy_create_wrapper(proxy: *mut ()) -> *mut ();

	pub fn wl_proxy_wrapper_destroy(proxy_wrapper: *mut ());

	pub fn wl_proxy_marshal_constructor(proxy: *mut wl_proxy, opcode: u32, interface: *const wl_interface, ...) -> *mut wl_proxy;

	pub fn wl_proxy_marshal_constructor_versioned(proxy: *mut wl_proxy, opcode: u32, interface: *const wl_interface, version: u32, ...) -> *mut wl_proxy;

	pub fn wl_proxy_marshal_array_constructor(proxy: *mut wl_proxy, opcode: u32, args: *mut wl_argument, interface: *const wl_interface) -> *mut wl_proxy;

	pub fn wl_proxy_marshal_array_constructor_versioned(proxy: *mut wl_proxy, opcode: u32, args: *mut wl_argument, interface: *const wl_interface, version: u32) -> *mut wl_proxy;

	pub fn wl_proxy_destroy(proxy: *mut wl_proxy);

	// NOTE: The C equivalent type of implementation is "void (**implementation)(void)",
	// which is a pointer to an array of function pointers.
	pub fn wl_proxy_add_listener(proxy: *mut wl_proxy, implementation: *const *const unsafe extern "C" fn(), data: *mut ()) -> i32;

	pub fn wl_proxy_get_listener(proxy: *mut wl_proxy) -> *const ();

	pub fn wl_proxy_add_dispatcher(proxy: *mut wl_proxy, dispatcher_func: wl_dispatcher_func_t, dispatcher_data: *const (), data: *mut ()) -> i32;

	pub fn wl_proxy_set_user_data(proxy: *mut wl_proxy, user_data: *mut ());

	pub fn wl_proxy_get_user_data(proxy: *mut wl_proxy) -> *mut ();

	pub fn wl_proxy_get_version(proxy: *mut wl_proxy) -> u32;

	pub fn wl_proxy_get_id(proxy: *mut wl_proxy) -> u32;

	pub fn wl_proxy_set_tag(proxy: *mut wl_proxy, tag: *const *const i8);

	pub fn wl_proxy_get_tag(proxy: *mut wl_proxy) -> *const *const i8;

	pub fn wl_proxy_get_class(proxy: *mut wl_proxy) -> *const i8;

	pub fn wl_proxy_get_interface(proxy: *mut wl_proxy) -> *const wl_interface;

	pub fn wl_proxy_get_display(proxy: *mut wl_proxy) -> *mut wl_display;

	pub fn wl_proxy_set_queue(proxy: *mut wl_proxy, queue: *mut wl_event_queue);

	pub fn wl_proxy_get_queue(proxy: *const wl_proxy) -> *mut wl_event_queue;

	pub fn wl_event_queue_get_name(queue: *const wl_event_queue) -> *const i8;

	pub fn wl_display_connect(name: *const i8) -> *mut wl_display;

	pub fn wl_display_connect_to_fd(fd: i32) -> *mut wl_display;

	pub fn wl_display_disconnect(display: *mut wl_display);

	pub fn wl_display_get_fd(display: *mut wl_display) -> i32;

	pub fn wl_display_dispatch(display: *mut wl_display) -> i32;

	pub fn wl_display_dispatch_queue(display: *mut wl_display, queue: *mut wl_event_queue) -> i32;

	pub fn wl_display_dispatch_timeout(display: *mut wl_display, timeout: *const timespec) -> i32;

	pub fn wl_display_dispatch_queue_timeout(display: *mut wl_display, queue: *mut wl_event_queue, timeout: *const timespec) -> i32;

	pub fn wl_display_dispatch_queue_pending(display: *mut wl_display, queue: *mut wl_event_queue) -> i32;

	pub fn wl_display_dispatch_queue_pending_single(display: *mut wl_display, queue: *mut wl_event_queue) -> i32;

	pub fn wl_display_dispatch_pending(display: *mut wl_display) -> i32;

	pub fn wl_display_dispatch_pending_single(display: *mut wl_display) -> i32;

	pub fn wl_display_get_error(display: *mut wl_display) -> i32;

	pub fn wl_display_get_protocol_error(display: *mut wl_display, interface: *mut *const wl_interface, id: *mut u32) -> u32;

	pub fn wl_display_flush(display: *mut wl_display) -> i32;

	pub fn wl_display_roundtrip_queue(display: *mut wl_display, queue: *mut wl_event_queue) -> i32;

	pub fn wl_display_roundtrip(display: *mut wl_display) -> i32;

	pub fn wl_display_create_queue(display: *mut wl_display) -> *mut wl_event_queue;

	pub fn wl_display_create_queue_with_name(display: *mut wl_display, name: *const i8) -> *mut wl_event_queue;

	pub fn wl_display_prepare_read_queue(display: *mut wl_display, queue: *mut wl_event_queue) -> i32;

	pub fn wl_display_prepare_read(display: *mut wl_display) -> i32;

	pub fn wl_display_cancel_read(display: *mut wl_display);

	pub fn wl_display_read_events(display: *mut wl_display) -> i32;

	pub fn wl_log_set_handler_client(handler: wl_log_func_t);

	pub fn wl_display_set_max_buffer_size(display: *mut wl_display, max_buffer_size: usize);
}
