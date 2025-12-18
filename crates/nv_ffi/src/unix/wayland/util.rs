use ::core::marker;

pub const WL_MAX_MESSAGE_SIZE: u32 = 4096;

// NOTE: Both wl_message and wl_interface types are used in the definition of static objects,
// which must implement the Sync trait. But they contain raw pointers that are not Sync by
// default. Since these static objects will be immutable with pointers pointing to const data,
// it is safe to implement the Sync trait on them. For an example, see the binding symbol
// client::xdg_shell_protocol::xdg_wm_base_interface.
unsafe impl marker::Sync for wl_message {}
unsafe impl marker::Sync for wl_interface {}

// NOTE: These and va_list are not part of the original library.
pub const NULL_INTERFACE: *const wl_interface = 0 as *const _;
pub const NULL_MESSAGE: *const wl_message = 0 as *const _;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct va_list {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_object {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_message {
	pub name: *const i8,
	pub signature: *const i8,
	pub types: *const *const wl_interface,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_interface {
	pub name: *const i8,
	pub version: i32,
	pub method_count: i32,
	pub methods: *const wl_message,
	pub event_count: i32,
	pub events: *const wl_message,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_list {
	pub prev: *mut wl_list,
	pub next: *mut wl_list,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct wl_array {
	pub size: usize,
	pub alloc: usize,
	pub data: *mut (),
}

pub type wl_fixed_t = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub union wl_argument {
	pub i: i32,
	pub u: u32,
	pub f: wl_fixed_t,
	pub s: *const i8,
	pub o: *mut wl_object,
	pub n: u32,
	pub a: *mut wl_array,
	pub h: i32,
}

pub type wl_dispatcher_func_t = unsafe extern "C" fn(user_data: *const (), target: *mut (), opcode: u32, msg: *const wl_message, args: *mut wl_argument) -> i32;

pub type wl_log_func_t = unsafe extern "C" fn(fmt: *const i8, args: *mut va_list);

pub type wl_iterator_result = u32;
pub const WL_ITERATOR_STOP: wl_iterator_result = 0;
pub const WL_ITERATOR_CONTINUE: wl_iterator_result = 1;

#[link(name = "wayland-client")]
unsafe extern "C" {
	pub fn wl_list_init(list: *mut wl_list);

	pub fn wl_list_insert(list: *mut wl_list, elm: *mut wl_list);

	pub fn wl_list_remove(elm: *mut wl_list);

	pub fn wl_list_length(list: *const wl_list) -> i32;

	pub fn wl_list_empty(list: *const wl_list) -> i32;

	pub fn wl_list_insert_list(list: *mut wl_list, other: *mut wl_list);

	pub fn wl_array_init(array: *mut wl_array);

	pub fn wl_array_release(array: *mut wl_array);

	pub fn wl_array_add(array: *mut wl_array, size: usize) -> *mut ();

	pub fn wl_array_copy(array: *mut wl_array, source: *mut wl_array) -> i32;
}
