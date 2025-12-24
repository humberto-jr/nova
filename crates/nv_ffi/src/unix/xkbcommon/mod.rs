// NOTE: These bindings are generated from xkbcommon version 1.13.1.

use crate::libc;

pub mod compose;
pub mod keysyms;
pub mod names;
pub mod registry;
pub mod x11;

pub const XKB_LAYOUT_INVALID: u32 = 4294967295;
pub const XKB_LEVEL_INVALID: u32 = 4294967295;
pub const XKB_MOD_INVALID: u32 = 4294967295;
pub const XKB_LED_INVALID: u32 = 4294967295;
pub const XKB_GROUP_INVALID: u32 = 4294967295;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_context {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_keymap {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_state_machine {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_state {
	_unused: [u8; 0],
}

pub type xkb_keycode_t = u32;
pub type xkb_keysym_t = u32;
pub type xkb_layout_index_t = u32;
pub type xkb_layout_mask_t = u32;
pub type xkb_level_index_t = u32;
pub type xkb_mod_index_t = u32;
pub type xkb_mod_mask_t = u32;
pub type xkb_led_index_t = u32;
pub type xkb_led_mask_t = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_rmlvo_builder {
	_unused: [u8; 0],
}

pub type xkb_rmlvo_builder_flags = u32;
pub const XKB_RMLVO_BUILDER_NO_FLAGS: xkb_rmlvo_builder_flags = 0;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_rule_names {
	pub rules: *const i8,
	pub model: *const i8,
	pub layout: *const i8,
	pub variant: *const i8,
	pub options: *const i8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_component_names {
	pub keycodes: *mut i8,
	pub compatibility: *mut i8,
	pub geometry: *mut i8,
	pub symbols: *mut i8,
	pub types: *mut i8,
}

pub type xkb_keysym_flags = u32;
pub const XKB_KEYSYM_NO_FLAGS: xkb_keysym_flags = 0;
pub const XKB_KEYSYM_CASE_INSENSITIVE: xkb_keysym_flags = 1;

pub type xkb_context_flags = u32;
pub const XKB_CONTEXT_NO_FLAGS: xkb_context_flags = 0;
pub const XKB_CONTEXT_NO_DEFAULT_INCLUDES: xkb_context_flags = 1;
pub const XKB_CONTEXT_NO_ENVIRONMENT_NAMES: xkb_context_flags = 2;
pub const XKB_CONTEXT_NO_SECURE_GETENV: xkb_context_flags = 4;

pub type xkb_log_level = u32;
pub const XKB_LOG_LEVEL_CRITICAL: xkb_log_level = 10;
pub const XKB_LOG_LEVEL_ERROR: xkb_log_level = 20;
pub const XKB_LOG_LEVEL_WARNING: xkb_log_level = 30;
pub const XKB_LOG_LEVEL_INFO: xkb_log_level = 40;
pub const XKB_LOG_LEVEL_DEBUG: xkb_log_level = 50;

pub type xkb_keymap_compile_flags = u32;
pub const XKB_KEYMAP_COMPILE_NO_FLAGS: xkb_keymap_compile_flags = 0;

pub type xkb_keymap_format = u32;
pub const XKB_KEYMAP_FORMAT_TEXT_V1: xkb_keymap_format = 1;
pub const XKB_KEYMAP_FORMAT_TEXT_V2: xkb_keymap_format = 2;

pub type xkb_keymap_serialize_flags = u32;
pub const XKB_KEYMAP_SERIALIZE_NO_FLAGS: xkb_keymap_serialize_flags = 0;
pub const XKB_KEYMAP_SERIALIZE_PRETTY: xkb_keymap_serialize_flags = 1;
pub const XKB_KEYMAP_SERIALIZE_KEEP_UNUSED: xkb_keymap_serialize_flags = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_keymap_key_iterator {
	_unused: [u8; 0],
}

pub type xkb_keymap_key_iterator_flags = u32;
pub const XKB_KEYMAP_KEY_ITERATOR_NO_FLAGS: xkb_keymap_key_iterator_flags = 0;
pub const XKB_KEYMAP_KEY_ITERATOR_DESCENDING_ORDER: xkb_keymap_key_iterator_flags = 1;
pub const XKB_KEYMAP_KEY_ITERATOR_SKIP_UNBOUND: xkb_keymap_key_iterator_flags = 2;

pub type xkb_keymap_key_iter_t = unsafe extern "C" fn(keymap: *mut xkb_keymap, key: xkb_keycode_t, data: *mut ());

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_state_machine_options {
	_unused: [u8; 0],
}

pub type xkb_state_accessibility_flags = u32;
pub const XKB_STATE_A11Y_NO_FLAGS: xkb_state_accessibility_flags = 0;
pub const XKB_STATE_A11Y_LATCH_TO_LOCK: xkb_state_accessibility_flags = 1;
pub const XKB_STATE_A11Y_LATCH_SIMULTANEOUS_KEYS: xkb_state_accessibility_flags = 2;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_event {
	_unused: [u8; 0],
}

pub type xkb_event_type = u32;
pub const XKB_EVENT_TYPE_KEY_DOWN: xkb_event_type = 1;
pub const XKB_EVENT_TYPE_KEY_UP: xkb_event_type = 2;
pub const XKB_EVENT_TYPE_COMPONENTS_CHANGE: xkb_event_type = 3;

pub type xkb_state_component = u32;
pub const XKB_STATE_MODS_DEPRESSED: xkb_state_component = 1;
pub const XKB_STATE_MODS_LATCHED: xkb_state_component = 2;
pub const XKB_STATE_MODS_LOCKED: xkb_state_component = 4;
pub const XKB_STATE_MODS_EFFECTIVE: xkb_state_component = 8;
pub const XKB_STATE_LAYOUT_DEPRESSED: xkb_state_component = 16;
pub const XKB_STATE_LAYOUT_LATCHED: xkb_state_component = 32;
pub const XKB_STATE_LAYOUT_LOCKED: xkb_state_component = 64;
pub const XKB_STATE_LAYOUT_EFFECTIVE: xkb_state_component = 128;
pub const XKB_STATE_LEDS: xkb_state_component = 256;
pub const XKB_STATE_CONTROLS: xkb_state_component = 512;

pub type xkb_keyboard_controls = u32;
pub const XKB_KEYBOARD_CONTROL_NONE: xkb_keyboard_controls = 0;
pub const XKB_KEYBOARD_CONTROL_A11Y_STICKY_KEYS: xkb_keyboard_controls = 8;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct xkb_event_iterator {
	_unused: [u8; 0],
}

pub type xkb_key_direction = u32;
pub const XKB_KEY_UP: xkb_key_direction = 0;
pub const XKB_KEY_DOWN: xkb_key_direction = 1;

pub type xkb_state_match = u32;
pub const XKB_STATE_MATCH_ANY: xkb_state_match = 1;
pub const XKB_STATE_MATCH_ALL: xkb_state_match = 2;
pub const XKB_STATE_MATCH_NON_EXCLUSIVE: xkb_state_match = 65536;

pub type xkb_consumed_mode = u32;
pub const XKB_CONSUMED_MODE_XKB: xkb_consumed_mode = 0;
pub const XKB_CONSUMED_MODE_GTK: xkb_consumed_mode = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct va_list {
	_unused: [u8; 0],
}

// NOTE: LogFn and va_list are not part of the original library.
pub type LogFn = unsafe extern "C" fn(context: *mut xkb_context, level: xkb_log_level, format: *const i8, args: *mut va_list);

#[link(name = "xkbcommon")]
unsafe extern "C" {
	pub fn xkb_rmlvo_builder_new(context: *mut xkb_context, rules: *const i8, model: *const i8, flags: xkb_rmlvo_builder_flags) -> *mut xkb_rmlvo_builder;

	pub fn xkb_rmlvo_builder_append_layout(rmlvo: *mut xkb_rmlvo_builder, layout: *const i8, variant: *const i8, options: *const *const i8, options_len: usize) -> bool;

	pub fn xkb_rmlvo_builder_append_option(rmlvo: *mut xkb_rmlvo_builder, option: *const i8) -> bool;

	pub fn xkb_rmlvo_builder_ref(rmlvo: *mut xkb_rmlvo_builder) -> *mut xkb_rmlvo_builder;

	pub fn xkb_rmlvo_builder_unref(rmlvo: *mut xkb_rmlvo_builder);

	pub fn xkb_components_names_from_rules(context: *mut xkb_context, rmlvo_in: *const xkb_rule_names, rmlvo_out: *mut xkb_rule_names, components_out: *mut xkb_component_names) -> bool;

	pub fn xkb_keysym_get_name(keysym: xkb_keysym_t, buffer: *mut i8, size: usize) -> i32;

	pub fn xkb_keysym_from_name(name: *const i8, flags: xkb_keysym_flags) -> xkb_keysym_t;

	pub fn xkb_keysym_to_utf8(keysym: xkb_keysym_t, buffer: *mut i8, size: usize) -> i32;

	pub fn xkb_keysym_to_utf32(keysym: xkb_keysym_t) -> u32;

	pub fn xkb_utf32_to_keysym(ucs: u32) -> xkb_keysym_t;

	pub fn xkb_keysym_to_upper(ks: xkb_keysym_t) -> xkb_keysym_t;

	pub fn xkb_keysym_to_lower(ks: xkb_keysym_t) -> xkb_keysym_t;

	pub fn xkb_context_new(flags: xkb_context_flags) -> *mut xkb_context;

	pub fn xkb_context_ref(context: *mut xkb_context) -> *mut xkb_context;

	pub fn xkb_context_unref(context: *mut xkb_context);

	pub fn xkb_context_set_user_data(context: *mut xkb_context, user_data: *mut ());

	pub fn xkb_context_get_user_data(context: *mut xkb_context) -> *mut ();

	pub fn xkb_context_include_path_append(context: *mut xkb_context, path: *const i8) -> i32;

	pub fn xkb_context_include_path_append_default(context: *mut xkb_context) -> i32;

	pub fn xkb_context_include_path_reset_defaults(context: *mut xkb_context) -> i32;

	pub fn xkb_context_include_path_clear(context: *mut xkb_context);

	pub fn xkb_context_num_include_paths(context: *mut xkb_context) -> u32;

	pub fn xkb_context_include_path_get(context: *mut xkb_context, index: u32) -> *const i8;

	pub fn xkb_context_set_log_level(context: *mut xkb_context, level: xkb_log_level);

	pub fn xkb_context_get_log_level(context: *mut xkb_context) -> xkb_log_level;

	pub fn xkb_context_set_log_verbosity(context: *mut xkb_context, verbosity: i32);

	pub fn xkb_context_get_log_verbosity(context: *mut xkb_context) -> i32;

	pub fn xkb_context_set_log_fn(context: *mut xkb_context, log_fn: LogFn);

	pub fn xkb_keymap_new_from_rmlvo(rmlvo: *const xkb_rmlvo_builder, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_new_from_names(context: *mut xkb_context, names: *const xkb_rule_names, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_new_from_names2(context: *mut xkb_context, names: *const xkb_rule_names, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_new_from_file(context: *mut xkb_context, file: *mut libc::FILE, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_new_from_string(context: *mut xkb_context, string: *const i8, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_new_from_buffer(context: *mut xkb_context, buffer: *const i8, length: usize, format: xkb_keymap_format, flags: xkb_keymap_compile_flags) -> *mut xkb_keymap;

	pub fn xkb_keymap_ref(keymap: *mut xkb_keymap) -> *mut xkb_keymap;

	pub fn xkb_keymap_unref(keymap: *mut xkb_keymap);

	pub fn xkb_keymap_get_as_string(keymap: *mut xkb_keymap, format: xkb_keymap_format) -> *mut i8;

	pub fn xkb_keymap_get_as_string2(keymap: *mut xkb_keymap, format: xkb_keymap_format, flags: xkb_keymap_serialize_flags) -> *mut i8;

	pub fn xkb_keymap_min_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;

	pub fn xkb_keymap_max_keycode(keymap: *mut xkb_keymap) -> xkb_keycode_t;

	pub fn xkb_keymap_key_iterator_new(keymap: *mut xkb_keymap, flags: xkb_keymap_key_iterator_flags) -> *mut xkb_keymap_key_iterator;

	pub fn xkb_keymap_key_iterator_destroy(iter: *mut xkb_keymap_key_iterator);

	pub fn xkb_keymap_key_iterator_next(iter: *mut xkb_keymap_key_iterator) -> xkb_keycode_t;

	pub fn xkb_keymap_key_for_each(keymap: *mut xkb_keymap, iter: xkb_keymap_key_iter_t, data: *mut ());

	pub fn xkb_keymap_key_get_name(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> *const i8;

	pub fn xkb_keymap_key_by_name(keymap: *mut xkb_keymap, name: *const i8) -> xkb_keycode_t;

	pub fn xkb_keymap_num_mods(keymap: *mut xkb_keymap) -> xkb_mod_index_t;

	pub fn xkb_keymap_mod_get_name(keymap: *mut xkb_keymap, idx: xkb_mod_index_t) -> *const i8;

	pub fn xkb_keymap_mod_get_index(keymap: *mut xkb_keymap, name: *const i8) -> xkb_mod_index_t;

	pub fn xkb_keymap_mod_get_mask(keymap: *mut xkb_keymap, name: *const i8) -> xkb_mod_mask_t;

	pub fn xkb_keymap_mod_get_mask2(keymap: *mut xkb_keymap, idx: xkb_mod_index_t) -> xkb_mod_mask_t;

	pub fn xkb_keymap_num_layouts(keymap: *mut xkb_keymap) -> xkb_layout_index_t;

	pub fn xkb_keymap_layout_get_name(keymap: *mut xkb_keymap, idx: xkb_layout_index_t) -> *const i8;

	pub fn xkb_keymap_layout_get_index(keymap: *mut xkb_keymap, name: *const i8) -> xkb_layout_index_t;

	pub fn xkb_keymap_num_leds(keymap: *mut xkb_keymap) -> xkb_led_index_t;

	pub fn xkb_keymap_led_get_name(keymap: *mut xkb_keymap, idx: xkb_led_index_t) -> *const i8;

	pub fn xkb_keymap_led_get_index(keymap: *mut xkb_keymap, name: *const i8) -> xkb_led_index_t;

	pub fn xkb_keymap_num_layouts_for_key(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> xkb_layout_index_t;

	pub fn xkb_keymap_num_levels_for_key(keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t) -> xkb_level_index_t;

	pub fn xkb_keymap_key_get_mods_for_level(
		keymap: *mut xkb_keymap,
		key: xkb_keycode_t,
		layout: xkb_layout_index_t,
		level: xkb_level_index_t,
		masks_out: *mut xkb_mod_mask_t,
		masks_size: usize,
	) -> usize;

	pub fn xkb_keymap_key_get_syms_by_level(keymap: *mut xkb_keymap, key: xkb_keycode_t, layout: xkb_layout_index_t, level: xkb_level_index_t, syms_out: *mut *const xkb_keysym_t) -> i32;

	pub fn xkb_keymap_key_repeats(keymap: *mut xkb_keymap, key: xkb_keycode_t) -> i32;

	pub fn xkb_state_machine_options_new(context: *mut xkb_context) -> *mut xkb_state_machine_options;

	pub fn xkb_state_machine_options_destroy(options: *mut xkb_state_machine_options);

	pub fn xkb_state_machine_options_update_a11y_flags(options: *mut xkb_state_machine_options, affect: xkb_state_accessibility_flags, flags: xkb_state_accessibility_flags) -> i32;

	pub fn xkb_event_get_type(event: *const xkb_event) -> xkb_event_type;

	pub fn xkb_event_get_keycode(event: *const xkb_event) -> xkb_keycode_t;

	pub fn xkb_event_get_changed_components(event: *const xkb_event) -> xkb_state_component;

	pub fn xkb_event_serialize_controls(event: *const xkb_event, components: xkb_state_component) -> xkb_keyboard_controls;

	pub fn xkb_event_serialize_mods(event: *const xkb_event, components: xkb_state_component) -> xkb_mod_mask_t;

	pub fn xkb_event_serialize_layout(event: *const xkb_event, components: xkb_state_component) -> xkb_layout_index_t;

	pub fn xkb_event_iterator_new(sm: *mut xkb_state_machine) -> *mut xkb_event_iterator;

	pub fn xkb_event_iterator_destroy(events: *mut xkb_event_iterator);

	pub fn xkb_event_iterator_next(events: *mut xkb_event_iterator) -> *const xkb_event;

	pub fn xkb_state_machine_new(keymap: *mut xkb_keymap, options: *const xkb_state_machine_options) -> *mut xkb_state_machine;

	pub fn xkb_state_machine_ref(sm: *mut xkb_state_machine) -> *mut xkb_state_machine;

	pub fn xkb_state_machine_unref(sm: *mut xkb_state_machine);

	pub fn xkb_state_machine_get_keymap(sm: *const xkb_state_machine) -> *mut xkb_keymap;

	pub fn xkb_state_machine_update_controls(sm: *mut xkb_state_machine, events: *mut xkb_event_iterator, affect: xkb_keyboard_controls, controls: xkb_keyboard_controls) -> i32;

	pub fn xkb_state_machine_update_key(sm: *mut xkb_state_machine, events: *mut xkb_event_iterator, key: xkb_keycode_t, direction: xkb_key_direction) -> i32;

	pub fn xkb_state_machine_update_latched_locked(
		sm: *mut xkb_state_machine,
		events: *mut xkb_event_iterator,
		affect_latched_mods: xkb_mod_mask_t,
		latched_mods: xkb_mod_mask_t,
		affect_latched_layout: bool,
		latched_layout: i32,
		affect_locked_mods: xkb_mod_mask_t,
		locked_mods: xkb_mod_mask_t,
		affect_locked_layout: bool,
		locked_layout: i32,
	) -> i32;

	pub fn xkb_state_new(keymap: *mut xkb_keymap) -> *mut xkb_state;

	pub fn xkb_state_ref(state: *mut xkb_state) -> *mut xkb_state;

	pub fn xkb_state_unref(state: *mut xkb_state);

	pub fn xkb_state_get_keymap(state: *mut xkb_state) -> *mut xkb_keymap;

	pub fn xkb_state_update_controls(state: *mut xkb_state, affect: xkb_keyboard_controls, controls: xkb_keyboard_controls) -> xkb_state_component;

	pub fn xkb_state_update_key(state: *mut xkb_state, key: xkb_keycode_t, direction: xkb_key_direction) -> xkb_state_component;

	pub fn xkb_state_update_from_event(state: *mut xkb_state, event: *const xkb_event) -> xkb_state_component;

	pub fn xkb_state_update_latched_locked(
		state: *mut xkb_state,
		affect_latched_mods: xkb_mod_mask_t,
		latched_mods: xkb_mod_mask_t,
		affect_latched_layout: bool,
		latched_layout: i32,
		affect_locked_mods: xkb_mod_mask_t,
		locked_mods: xkb_mod_mask_t,
		affect_locked_layout: bool,
		locked_layout: i32,
	) -> xkb_state_component;

	pub fn xkb_state_update_mask(
		state: *mut xkb_state,
		depressed_mods: xkb_mod_mask_t,
		latched_mods: xkb_mod_mask_t,
		locked_mods: xkb_mod_mask_t,
		depressed_layout: xkb_layout_index_t,
		latched_layout: xkb_layout_index_t,
		locked_layout: xkb_layout_index_t,
	) -> xkb_state_component;

	pub fn xkb_state_key_get_syms(state: *mut xkb_state, key: xkb_keycode_t, syms_out: *mut *const xkb_keysym_t) -> i32;

	pub fn xkb_state_key_get_utf8(state: *mut xkb_state, key: xkb_keycode_t, buffer: *mut i8, size: usize) -> i32;

	pub fn xkb_state_key_get_utf32(state: *mut xkb_state, key: xkb_keycode_t) -> u32;

	pub fn xkb_state_key_get_one_sym(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_keysym_t;

	pub fn xkb_state_key_get_layout(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_layout_index_t;

	pub fn xkb_state_key_get_level(state: *mut xkb_state, key: xkb_keycode_t, layout: xkb_layout_index_t) -> xkb_level_index_t;

	pub fn xkb_state_serialize_controls(state: *const xkb_state, components: xkb_state_component) -> xkb_keyboard_controls;

	pub fn xkb_state_serialize_mods(state: *mut xkb_state, components: xkb_state_component) -> xkb_mod_mask_t;

	pub fn xkb_state_serialize_layout(state: *mut xkb_state, components: xkb_state_component) -> xkb_layout_index_t;

	pub fn xkb_state_mod_name_is_active(state: *mut xkb_state, name: *const i8, type_: xkb_state_component) -> i32;

	pub fn xkb_state_mod_names_are_active(state: *mut xkb_state, type_: xkb_state_component, match_: xkb_state_match, ...) -> i32;

	pub fn xkb_state_mod_index_is_active(state: *mut xkb_state, idx: xkb_mod_index_t, type_: xkb_state_component) -> i32;

	pub fn xkb_state_mod_indices_are_active(state: *mut xkb_state, type_: xkb_state_component, match_: xkb_state_match, ...) -> i32;

	pub fn xkb_state_key_get_consumed_mods2(state: *mut xkb_state, key: xkb_keycode_t, mode: xkb_consumed_mode) -> xkb_mod_mask_t;

	pub fn xkb_state_key_get_consumed_mods(state: *mut xkb_state, key: xkb_keycode_t) -> xkb_mod_mask_t;

	pub fn xkb_state_mod_index_is_consumed2(state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t, mode: xkb_consumed_mode) -> i32;

	pub fn xkb_state_mod_index_is_consumed(state: *mut xkb_state, key: xkb_keycode_t, idx: xkb_mod_index_t) -> i32;

	pub fn xkb_state_mod_mask_remove_consumed(state: *mut xkb_state, key: xkb_keycode_t, mask: xkb_mod_mask_t) -> xkb_mod_mask_t;

	pub fn xkb_state_layout_name_is_active(state: *mut xkb_state, name: *const i8, type_: xkb_state_component) -> i32;

	pub fn xkb_state_layout_index_is_active(state: *mut xkb_state, idx: xkb_layout_index_t, type_: xkb_state_component) -> i32;

	pub fn xkb_state_led_name_is_active(state: *mut xkb_state, name: *const i8) -> i32;

	pub fn xkb_state_led_index_is_active(state: *mut xkb_state, idx: xkb_led_index_t) -> i32;
}
