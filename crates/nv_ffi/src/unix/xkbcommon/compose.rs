use crate::libc;

pub use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_compose_table {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_compose_state {
	_unused: [u8; 0],
}

pub type xkb_compose_compile_flags = u32;
pub const XKB_COMPOSE_COMPILE_NO_FLAGS: xkb_compose_compile_flags = 0;

pub type xkb_compose_format = u32;
pub const XKB_COMPOSE_FORMAT_TEXT_V1: xkb_compose_format = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_compose_table_entry {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_compose_table_iterator {
	_unused: [u8; 0],
}

pub type xkb_compose_state_flags = u32;
pub const XKB_COMPOSE_STATE_NO_FLAGS: xkb_compose_state_flags = 0;

pub type xkb_compose_status = u32;
pub const XKB_COMPOSE_NOTHING: xkb_compose_status = 0;
pub const XKB_COMPOSE_COMPOSING: xkb_compose_status = 1;
pub const XKB_COMPOSE_COMPOSED: xkb_compose_status = 2;
pub const XKB_COMPOSE_CANCELLED: xkb_compose_status = 3;

pub type xkb_compose_feed_result = u32;
pub const XKB_COMPOSE_FEED_IGNORED: xkb_compose_feed_result = 0;
pub const XKB_COMPOSE_FEED_ACCEPTED: xkb_compose_feed_result = 1;

#[link(name = "xkbcommon")]
unsafe extern "C" {
	pub fn xkb_compose_table_new_from_locale(context: *mut xkb_context, locale: *const i8, flags: xkb_compose_compile_flags) -> *mut xkb_compose_table;

	pub fn xkb_compose_table_new_from_file(context: *mut xkb_context, file: *mut libc::FILE, locale: *const i8, format: xkb_compose_format, flags: xkb_compose_compile_flags)
	-> *mut xkb_compose_table;

	pub fn xkb_compose_table_new_from_buffer(
		context: *mut xkb_context,
		buffer: *const i8,
		length: usize,
		locale: *const i8,
		format: xkb_compose_format,
		flags: xkb_compose_compile_flags,
	) -> *mut xkb_compose_table;

	pub fn xkb_compose_table_ref(table: *mut xkb_compose_table) -> *mut xkb_compose_table;

	pub fn xkb_compose_table_unref(table: *mut xkb_compose_table);

	pub fn xkb_compose_table_entry_sequence(entry: *mut xkb_compose_table_entry, sequence_length: *mut usize) -> *const xkb_keysym_t;

	pub fn xkb_compose_table_entry_keysym(entry: *mut xkb_compose_table_entry) -> xkb_keysym_t;

	pub fn xkb_compose_table_entry_utf8(entry: *mut xkb_compose_table_entry) -> *const i8;

	pub fn xkb_compose_table_iterator_new(table: *mut xkb_compose_table) -> *mut xkb_compose_table_iterator;

	pub fn xkb_compose_table_iterator_free(iter: *mut xkb_compose_table_iterator);

	pub fn xkb_compose_table_iterator_next(iter: *mut xkb_compose_table_iterator) -> *mut xkb_compose_table_entry;

	pub fn xkb_compose_state_new(table: *mut xkb_compose_table, flags: xkb_compose_state_flags) -> *mut xkb_compose_state;

	pub fn xkb_compose_state_ref(state: *mut xkb_compose_state) -> *mut xkb_compose_state;

	pub fn xkb_compose_state_unref(state: *mut xkb_compose_state);

	pub fn xkb_compose_state_get_compose_table(state: *mut xkb_compose_state) -> *mut xkb_compose_table;

	pub fn xkb_compose_state_feed(state: *mut xkb_compose_state, keysym: xkb_keysym_t) -> xkb_compose_feed_result;

	pub fn xkb_compose_state_reset(state: *mut xkb_compose_state);

	pub fn xkb_compose_state_get_status(state: *mut xkb_compose_state) -> xkb_compose_status;

	pub fn xkb_compose_state_get_utf8(state: *mut xkb_compose_state, buffer: *mut i8, size: usize) -> i32;

	pub fn xkb_compose_state_get_one_sym(state: *mut xkb_compose_state) -> xkb_keysym_t;
}
