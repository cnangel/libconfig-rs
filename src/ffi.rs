#![allow(dead_code)]

use std::os::raw::{c_char, c_int, c_uchar, c_uint};

// Opaque types — we never access fields directly
#[repr(C)]
pub(crate) struct config_t {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct config_setting_t {
    _private: [u8; 0],
}

// Boolean constants
#[allow(dead_code)]
pub(crate) const CONFIG_TRUE: c_int = 1;
#[allow(dead_code)]
pub(crate) const CONFIG_FALSE: c_int = 0;

// Config type constants
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_NONE: c_int = 0;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_GROUP: c_int = 1;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_INT: c_int = 2;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_INT64: c_int = 3;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_FLOAT: c_int = 4;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_STRING: c_int = 5;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_BOOL: c_int = 6;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_ARRAY: c_int = 7;
#[allow(dead_code)]
pub(crate) const CONFIG_TYPE_LIST: c_int = 8;

// Format constants
#[allow(dead_code)]
pub(crate) const CONFIG_FORMAT_DEFAULT: c_int = 0;
#[allow(dead_code)]
pub(crate) const CONFIG_FORMAT_HEX: c_int = 1;
#[allow(dead_code)]
pub(crate) const CONFIG_FORMAT_BIN: c_int = 2;
#[allow(dead_code)]
pub(crate) const CONFIG_FORMAT_OCT: c_int = 3;

// Error type constants
#[allow(dead_code)]
pub(crate) const CONFIG_ERR_NONE: c_int = 0;
#[allow(dead_code)]
pub(crate) const CONFIG_ERR_FILE_IO: c_int = 1;
#[allow(dead_code)]
pub(crate) const CONFIG_ERR_PARSE: c_int = 2;

extern "C" {
    // ── Config lifecycle ──
    pub(crate) fn config_init(config: *mut config_t);
    pub(crate) fn config_destroy(config: *mut config_t);

    // ── Config I/O ──
    pub(crate) fn config_read_file(config: *mut config_t, filename: *const c_char) -> c_int;
    pub(crate) fn config_write_file(config: *const config_t, filename: *const c_char) -> c_int;
}

#[cfg(libconfig_1_4)]
extern "C" {
    pub(crate) fn config_read_string(config: *mut config_t, str: *const c_char) -> c_int;
}

#[cfg(libconfig_1_8)]
extern "C" {
    pub(crate) fn config_clear(config: *mut config_t);
    // Options
    pub(crate) fn config_set_options(config: *mut config_t, options: c_int);
    pub(crate) fn config_get_options(config: *const config_t) -> c_int;
    pub(crate) fn config_set_option(config: *mut config_t, option: c_int, flag: c_int);
    pub(crate) fn config_get_option(config: *const config_t, option: c_int) -> c_int;
    // Float precision
    pub(crate) fn config_set_float_precision(config: *mut config_t, digits: c_uchar);
    pub(crate) fn config_get_float_precision(config: *const config_t) -> c_uchar;
    // Tab width
    pub(crate) fn config_set_tab_width(config: *mut config_t, width: c_uchar);
    pub(crate) fn config_get_tab_width(config: *const config_t) -> c_uchar;
}

#[cfg(libconfig_1_4)]
extern "C" {
    pub(crate) fn config_set_default_format(config: *mut config_t, format: c_uchar);
}

extern "C" {
    // ── Config lookup ──
    pub(crate) fn config_lookup(config: *const config_t, path: *const c_char) -> *mut config_setting_t;
    pub(crate) fn config_lookup_int(config: *const config_t, path: *const c_char, value: *mut c_int) -> c_int;
    pub(crate) fn config_lookup_int64(config: *const config_t, path: *const c_char, value: *mut i64) -> c_int;
    pub(crate) fn config_lookup_float(config: *const config_t, path: *const c_char, value: *mut f64) -> c_int;
    pub(crate) fn config_lookup_bool(config: *const config_t, path: *const c_char, value: *mut c_int) -> c_int;
    pub(crate) fn config_lookup_string(config: *const config_t, path: *const c_char, value: *mut *const c_char)
        -> c_int;

    // ── Setting value getters ──
    pub(crate) fn config_setting_get_int(setting: *const config_setting_t) -> c_int;
    pub(crate) fn config_setting_get_int64(setting: *const config_setting_t) -> i64;
    pub(crate) fn config_setting_get_float(setting: *const config_setting_t) -> f64;
    pub(crate) fn config_setting_get_bool(setting: *const config_setting_t) -> c_int;
    pub(crate) fn config_setting_get_string(setting: *const config_setting_t) -> *const c_char;

    // ── Setting value setters ──
    pub(crate) fn config_setting_set_int(setting: *mut config_setting_t, value: c_int) -> c_int;
    pub(crate) fn config_setting_set_int64(setting: *mut config_setting_t, value: i64) -> c_int;
    pub(crate) fn config_setting_set_float(setting: *mut config_setting_t, value: f64) -> c_int;
    pub(crate) fn config_setting_set_bool(setting: *mut config_setting_t, value: c_int) -> c_int;
    pub(crate) fn config_setting_set_string(setting: *mut config_setting_t, value: *const c_char) -> c_int;

    // ── Setting structure ──
    pub(crate) fn config_setting_length(setting: *const config_setting_t) -> c_int;
    pub(crate) fn config_setting_get_elem(
        setting: *const config_setting_t,
        idx: c_int,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_get_member(
        setting: *const config_setting_t,
        name: *const c_char,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_add(
        parent: *mut config_setting_t,
        name: *const c_char,
        type_: c_int,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_remove(parent: *mut config_setting_t, name: *const c_char) -> c_int;
    pub(crate) fn config_setting_remove_elem(parent: *mut config_setting_t, idx: c_int) -> c_int;
    pub(crate) fn config_setting_index(setting: *const config_setting_t) -> c_int;

    // ── Setting element getters ──
    pub(crate) fn config_setting_get_int_elem(setting: *const config_setting_t, idx: c_int) -> c_int;
    pub(crate) fn config_setting_get_int64_elem(setting: *const config_setting_t, idx: c_int) -> i64;
    pub(crate) fn config_setting_get_float_elem(setting: *const config_setting_t, idx: c_int) -> f64;
    pub(crate) fn config_setting_get_bool_elem(setting: *const config_setting_t, idx: c_int) -> c_int;
    pub(crate) fn config_setting_get_string_elem(
        setting: *const config_setting_t,
        idx: c_int,
    ) -> *const c_char;

    // ── Setting element setters ──
    pub(crate) fn config_setting_set_int_elem(
        setting: *mut config_setting_t,
        idx: c_int,
        value: c_int,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_set_int64_elem(
        setting: *mut config_setting_t,
        idx: c_int,
        value: i64,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_set_float_elem(
        setting: *mut config_setting_t,
        idx: c_int,
        value: f64,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_set_bool_elem(
        setting: *mut config_setting_t,
        idx: c_int,
        value: c_int,
    ) -> *mut config_setting_t;
    pub(crate) fn config_setting_set_string_elem(
        setting: *mut config_setting_t,
        idx: c_int,
        value: *const c_char,
    ) -> *mut config_setting_t;

    // ── Setting hook ──
    pub(crate) fn config_setting_set_hook(setting: *mut config_setting_t, hook: *mut std::ffi::c_void);
}

// libconfig 1.8+ safe getters
#[cfg(libconfig_1_8)]
extern "C" {
    pub(crate) fn config_setting_get_int_safe(setting: *const config_setting_t, value: *mut c_int)
        -> c_int;
    pub(crate) fn config_setting_get_int64_safe(
        setting: *const config_setting_t,
        value: *mut i64,
    ) -> c_int;
    pub(crate) fn config_setting_get_float_safe(setting: *const config_setting_t, value: *mut f64)
        -> c_int;
    pub(crate) fn config_setting_get_bool_safe(setting: *const config_setting_t, value: *mut c_int)
        -> c_int;
    pub(crate) fn config_setting_get_string_safe(
        setting: *const config_setting_t,
        value: *mut *const c_char,
    ) -> c_int;
}

// libconfig 1.5+ setting lookup by path
#[cfg(libconfig_1_5)]
extern "C" {
    pub(crate) fn config_setting_lookup(
        setting: *const config_setting_t,
        path: *const c_char,
    ) -> *mut config_setting_t;
}

// libconfig 1.4+ setting-level typed lookups
#[cfg(libconfig_1_4)]
extern "C" {
    pub(crate) fn config_setting_lookup_int(
        setting: *const config_setting_t,
        name: *const c_char,
        value: *mut c_int,
    ) -> c_int;
    pub(crate) fn config_setting_lookup_int64(
        setting: *const config_setting_t,
        name: *const c_char,
        value: *mut i64,
    ) -> c_int;
    pub(crate) fn config_setting_lookup_float(
        setting: *const config_setting_t,
        name: *const c_char,
        value: *mut f64,
    ) -> c_int;
    pub(crate) fn config_setting_lookup_bool(
        setting: *const config_setting_t,
        name: *const c_char,
        value: *mut c_int,
    ) -> c_int;
    pub(crate) fn config_setting_lookup_string(
        setting: *const config_setting_t,
        name: *const c_char,
        value: *mut *const c_char,
    ) -> c_int;
}

// libconfig 1.5+ setting format
#[cfg(libconfig_1_5)]
extern "C" {
    pub(crate) fn config_setting_set_format(setting: *mut config_setting_t, format: c_uchar) -> c_int;
    pub(crate) fn config_setting_get_format(setting: *const config_setting_t) -> c_uchar;
}

// ── C wrapper functions (expose C macros as actual functions) ──
extern "C" {
    // Config lifecycle (C-side allocation)
    pub(crate) fn wrapper_config_new() -> *mut config_t;
    pub(crate) fn wrapper_config_free(config: *mut config_t);
    // Config error
    pub(crate) fn wrapper_config_error_text(config: *const config_t) -> *const c_char;
    pub(crate) fn wrapper_config_error_file(config: *const config_t) -> *const c_char;
    pub(crate) fn wrapper_config_error_line(config: *const config_t) -> c_int;
    pub(crate) fn wrapper_config_error_type(config: *const config_t) -> c_int;
    // Config macro wrappers
    pub(crate) fn wrapper_config_root_setting(config: *const config_t) -> *mut config_setting_t;
    pub(crate) fn wrapper_config_get_default_format(config: *const config_t) -> c_uchar;
    pub(crate) fn wrapper_config_set_default_format(config: *mut config_t, format: c_uchar);
    pub(crate) fn wrapper_config_set_auto_convert(config: *mut config_t, flag: c_int);
    pub(crate) fn wrapper_config_get_tab_width(config: *const config_t) -> c_uchar;
    pub(crate) fn wrapper_config_get_float_precision(config: *const config_t) -> c_uchar;
    pub(crate) fn wrapper_config_get_auto_convert(config: *const config_t) -> c_int;
    // Setting macro wrappers
    pub(crate) fn wrapper_config_setting_type(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_name(setting: *const config_setting_t) -> *const c_char;
    pub(crate) fn wrapper_config_setting_parent(
        setting: *const config_setting_t,
    ) -> *mut config_setting_t;
    pub(crate) fn wrapper_config_setting_is_root(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_source_line(setting: *const config_setting_t) -> c_uint;
    pub(crate) fn wrapper_config_setting_source_file(setting: *const config_setting_t) -> *const c_char;
    // Setting type check wrappers
    pub(crate) fn wrapper_config_setting_is_group(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_is_array(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_is_list(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_is_number(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_is_scalar(setting: *const config_setting_t) -> c_int;
    pub(crate) fn wrapper_config_setting_is_aggregate(setting: *const config_setting_t) -> c_int;
    // Hook wrappers
    pub(crate) fn wrapper_config_setting_get_hook(setting: *const config_setting_t)
        -> *mut std::ffi::c_void;
    pub(crate) fn wrapper_config_get_hook(config: *const config_t) -> *mut std::ffi::c_void;
    // Include dir wrapper
    pub(crate) fn wrapper_config_get_include_dir(config: *const config_t) -> *const c_char;
}

// libconfig 1.4+ include dir
#[cfg(libconfig_1_4)]
extern "C" {
    pub(crate) fn config_set_include_dir(config: *mut config_t, include_dir: *const c_char);
}