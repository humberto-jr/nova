#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_context {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_model {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_layout {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_option_group {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_option {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_iso639_code {
	_unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct rxkb_iso3166_code {
	_unused: [u8; 0],
}

pub type rxkb_popularity = u32;
pub const RXKB_POPULARITY_STANDARD: rxkb_popularity = 1;
pub const RXKB_POPULARITY_EXOTIC: rxkb_popularity = 2;

pub type rxkb_context_flags = u32;
pub const RXKB_CONTEXT_NO_FLAGS: rxkb_context_flags = 0;
pub const RXKB_CONTEXT_NO_DEFAULT_INCLUDES: rxkb_context_flags = 1;
pub const RXKB_CONTEXT_LOAD_EXOTIC_RULES: rxkb_context_flags = 2;
pub const RXKB_CONTEXT_NO_SECURE_GETENV: rxkb_context_flags = 4;

pub type rxkb_log_level = u32;
pub const RXKB_LOG_LEVEL_CRITICAL: rxkb_log_level = 10;
pub const RXKB_LOG_LEVEL_ERROR: rxkb_log_level = 20;
pub const RXKB_LOG_LEVEL_WARNING: rxkb_log_level = 30;
pub const RXKB_LOG_LEVEL_INFO: rxkb_log_level = 40;
pub const RXKB_LOG_LEVEL_DEBUG: rxkb_log_level = 50;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct va_list {
	_unused: [u8; 0],
}

// NOTE: LogFn and va_list are not part of the original library.
pub type LogFn = unsafe extern "C" fn(ctx: *mut rxkb_context, level: rxkb_log_level, format: *const i8, args: *mut va_list);

unsafe extern "C" {
	pub fn rxkb_context_new(flags: rxkb_context_flags) -> *mut rxkb_context;

	pub fn rxkb_context_set_log_level(ctx: *mut rxkb_context, level: rxkb_log_level);

	pub fn rxkb_context_get_log_level(ctx: *mut rxkb_context) -> rxkb_log_level;

	pub fn rxkb_context_set_log_fn(ctx: *mut rxkb_context, log_fn: LogFn);

	pub fn rxkb_context_parse(ctx: *mut rxkb_context, ruleset: *const i8) -> bool;

	pub fn rxkb_context_parse_default_ruleset(ctx: *mut rxkb_context) -> bool;

	pub fn rxkb_context_ref(ctx: *mut rxkb_context) -> *mut rxkb_context;

	pub fn rxkb_context_unref(ctx: *mut rxkb_context) -> *mut rxkb_context;

	pub fn rxkb_context_set_user_data(ctx: *mut rxkb_context, user_data: *mut ());

	pub fn rxkb_context_get_user_data(ctx: *mut rxkb_context) -> *mut ();

	pub fn rxkb_context_include_path_append(ctx: *mut rxkb_context, path: *const i8) -> bool;

	pub fn rxkb_context_include_path_append_default(ctx: *mut rxkb_context) -> bool;

	pub fn rxkb_model_first(ctx: *mut rxkb_context) -> *mut rxkb_model;

	pub fn rxkb_model_next(m: *mut rxkb_model) -> *mut rxkb_model;

	pub fn rxkb_model_ref(m: *mut rxkb_model) -> *mut rxkb_model;

	pub fn rxkb_model_unref(m: *mut rxkb_model) -> *mut rxkb_model;

	pub fn rxkb_model_get_name(m: *mut rxkb_model) -> *const i8;

	pub fn rxkb_model_get_description(m: *mut rxkb_model) -> *const i8;

	pub fn rxkb_model_get_vendor(m: *mut rxkb_model) -> *const i8;

	pub fn rxkb_model_get_popularity(m: *mut rxkb_model) -> rxkb_popularity;

	pub fn rxkb_layout_first(ctx: *mut rxkb_context) -> *mut rxkb_layout;

	pub fn rxkb_layout_next(l: *mut rxkb_layout) -> *mut rxkb_layout;

	pub fn rxkb_layout_ref(l: *mut rxkb_layout) -> *mut rxkb_layout;

	pub fn rxkb_layout_unref(l: *mut rxkb_layout) -> *mut rxkb_layout;

	pub fn rxkb_layout_get_name(l: *mut rxkb_layout) -> *const i8;

	pub fn rxkb_layout_get_variant(l: *mut rxkb_layout) -> *const i8;

	pub fn rxkb_layout_get_brief(l: *mut rxkb_layout) -> *const i8;

	pub fn rxkb_layout_get_description(l: *mut rxkb_layout) -> *const i8;

	pub fn rxkb_layout_get_popularity(l: *mut rxkb_layout) -> rxkb_popularity;

	pub fn rxkb_option_group_first(ctx: *mut rxkb_context) -> *mut rxkb_option_group;

	pub fn rxkb_option_group_next(g: *mut rxkb_option_group) -> *mut rxkb_option_group;

	pub fn rxkb_option_group_ref(g: *mut rxkb_option_group) -> *mut rxkb_option_group;

	pub fn rxkb_option_group_unref(g: *mut rxkb_option_group) -> *mut rxkb_option_group;

	pub fn rxkb_option_group_get_name(m: *mut rxkb_option_group) -> *const i8;

	pub fn rxkb_option_group_get_description(m: *mut rxkb_option_group) -> *const i8;

	pub fn rxkb_option_group_allows_multiple(g: *mut rxkb_option_group) -> bool;

	pub fn rxkb_option_group_get_popularity(g: *mut rxkb_option_group) -> rxkb_popularity;

	pub fn rxkb_option_first(group: *mut rxkb_option_group) -> *mut rxkb_option;

	pub fn rxkb_option_next(o: *mut rxkb_option) -> *mut rxkb_option;

	pub fn rxkb_option_ref(o: *mut rxkb_option) -> *mut rxkb_option;

	pub fn rxkb_option_unref(o: *mut rxkb_option) -> *mut rxkb_option;

	pub fn rxkb_option_get_name(o: *mut rxkb_option) -> *const i8;

	pub fn rxkb_option_get_brief(o: *mut rxkb_option) -> *const i8;

	pub fn rxkb_option_get_description(o: *mut rxkb_option) -> *const i8;

	pub fn rxkb_option_get_popularity(o: *mut rxkb_option) -> rxkb_popularity;

	pub fn rxkb_option_is_layout_specific(o: *mut rxkb_option) -> bool;

	pub fn rxkb_iso639_code_ref(iso639: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code;

	pub fn rxkb_iso639_code_unref(iso639: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code;

	pub fn rxkb_iso639_code_get_code(iso639: *mut rxkb_iso639_code) -> *const i8;

	pub fn rxkb_layout_get_iso639_first(layout: *mut rxkb_layout) -> *mut rxkb_iso639_code;

	pub fn rxkb_iso639_code_next(iso639: *mut rxkb_iso639_code) -> *mut rxkb_iso639_code;

	pub fn rxkb_iso3166_code_ref(iso3166: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code;

	pub fn rxkb_iso3166_code_unref(iso3166: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code;

	pub fn rxkb_iso3166_code_get_code(iso3166: *mut rxkb_iso3166_code) -> *const i8;

	pub fn rxkb_layout_get_iso3166_first(layout: *mut rxkb_layout) -> *mut rxkb_iso3166_code;

	pub fn rxkb_iso3166_code_next(iso3166: *mut rxkb_iso3166_code) -> *mut rxkb_iso3166_code;
}
