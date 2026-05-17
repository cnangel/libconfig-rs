use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr::NonNull;

#[allow(unused_imports)]
use crate::constants::{ConfigOptions, ErrorType, SettingFormat};
use crate::error::ConfigError;
use crate::ffi;
use crate::setting::Setting;

/// Owns a `libconfig::config_t`. All `Setting` references borrow from this.
///
/// `Config` is not `Send` or `Sync` — libconfig is not thread-safe.
pub struct Config {
    raw: NonNull<ffi::config_t>,
}

impl Config {
    /// Create a new, empty Config.
    pub fn new() -> Self {
        let raw = unsafe { ffi::wrapper_config_new() };
        Config {
            raw: NonNull::new(raw).expect("wrapper_config_new returned null"),
        }
    }

    pub(crate) fn raw_ptr(&self) -> *const ffi::config_t {
        self.raw.as_ptr() as *const _
    }

    pub(crate) fn raw_mut_ptr(&mut self) -> *mut ffi::config_t {
        self.raw.as_ptr()
    }

    // ── I/O ──

    /// Read configuration from a file.
    pub fn read_file(&mut self, filename: &str) -> Result<(), ConfigError> {
        let c_filename = CString::new(filename).map_err(|_| ConfigError {
            message: "invalid filename".into(),
            file: None,
            line: 0,
            error_type: ErrorType::None,
        })?;
        let ret = unsafe { ffi::config_read_file(self.raw_mut_ptr(), c_filename.as_ptr()) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.error_state())
        }
    }

    /// Write configuration to a file.
    pub fn write_file(&self, filename: &str) -> Result<(), ConfigError> {
        let c_filename = CString::new(filename).map_err(|_| ConfigError {
            message: "invalid filename".into(),
            file: None,
            line: 0,
            error_type: ErrorType::None,
        })?;
        let ret = unsafe { ffi::config_write_file(self.raw_ptr(), c_filename.as_ptr()) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.error_state())
        }
    }

    /// Read configuration from a string (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn read_string(&mut self, input: &str) -> Result<(), ConfigError> {
        let c_str = CString::new(input).map_err(|_| ConfigError {
            message: "invalid input string".into(),
            file: None,
            line: 0,
            error_type: ErrorType::None,
        })?;
        let ret = unsafe { ffi::config_read_string(self.raw_mut_ptr(), c_str.as_ptr()) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.error_state())
        }
    }

    // ── Lookup ──

    /// Look up a setting by path. Returns None if the path doesn't exist.
    pub fn lookup(&self, path: &str) -> Option<Setting<'_>> {
        let c_path = CString::new(path).ok()?;
        unsafe {
            let ptr = ffi::config_lookup(self.raw_ptr(), c_path.as_ptr());
            Setting::from_ptr(ptr)
        }
    }

    /// Get the root setting.
    pub fn get_root(&self) -> Setting<'_> {
        unsafe { Setting::from_ptr_unchecked(ffi::wrapper_config_root_setting(self.raw_ptr())) }
    }

    /// Check if a path exists.
    pub fn exists(&self, path: &str) -> bool {
        self.lookup(path).is_some()
    }

    /// Look up an integer value by path. Returns None if not found.
    pub fn lookup_int(&self, path: &str) -> Option<i32> {
        let c_path = CString::new(path).ok()?;
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_lookup_int(self.raw_ptr(), c_path.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE {
            Some(value)
        } else {
            None
        }
    }

    /// Look up an int64 value by path. Returns None if not found.
    pub fn lookup_int64(&self, path: &str) -> Option<i64> {
        let c_path = CString::new(path).ok()?;
        let mut value: i64 = 0;
        let ret = unsafe { ffi::config_lookup_int64(self.raw_ptr(), c_path.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE {
            Some(value)
        } else {
            None
        }
    }

    /// Look up a float value by path. Returns None if not found.
    pub fn lookup_float(&self, path: &str) -> Option<f64> {
        let c_path = CString::new(path).ok()?;
        let mut value: f64 = 0.0;
        let ret = unsafe { ffi::config_lookup_float(self.raw_ptr(), c_path.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE {
            Some(value)
        } else {
            None
        }
    }

    /// Look up a boolean value by path. Returns None if not found.
    pub fn lookup_bool(&self, path: &str) -> Option<bool> {
        let c_path = CString::new(path).ok()?;
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_lookup_bool(self.raw_ptr(), c_path.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE {
            Some(value != 0)
        } else {
            None
        }
    }

    /// Look up a string value by path. Returns None if not found.
    pub fn lookup_string(&self, path: &str) -> Option<&str> {
        let c_path = CString::new(path).ok()?;
        let mut value: *const std::os::raw::c_char = std::ptr::null();
        let ret =
            unsafe { ffi::config_lookup_string(self.raw_ptr(), c_path.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE && !value.is_null() {
            unsafe { Some(CStr::from_ptr(value).to_str().unwrap_or("")) }
        } else {
            None
        }
    }

    // ── Options (libconfig 1.8+) ──

    /// Set multiple options at once (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn set_options(&mut self, options: ConfigOptions) {
        unsafe { ffi::config_set_options(self.raw_mut_ptr(), options.bits()) }
    }

    /// Get the current options (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn get_options(&self) -> ConfigOptions {
        let bits = unsafe { ffi::config_get_options(self.raw_ptr()) };
        ConfigOptions::from_bits_truncate(bits)
    }

    /// Set a single option flag (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn set_option(&mut self, option: ConfigOptions, flag: bool) {
        unsafe {
            ffi::config_set_option(self.raw_mut_ptr(), option.bits(), flag as i32);
        }
    }

    /// Get a single option flag (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn get_option(&self, option: ConfigOptions) -> bool {
        unsafe { ffi::config_get_option(self.raw_ptr(), option.bits()) != 0 }
    }

    /// Enable or disable auto type conversion (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn set_auto_convert(&mut self, flag: bool) {
        self.set_option(ConfigOptions::AUTOCONVERT, flag)
    }

    /// Check if auto type conversion is enabled.
    pub fn get_auto_convert(&self) -> bool {
        unsafe { ffi::wrapper_config_get_auto_convert(self.raw_ptr()) != 0 }
    }

    // ── Format / Precision / Tab width ──

    /// Set the default numeric format.
    pub fn set_default_format(&mut self, format: SettingFormat) {
        unsafe {
            ffi::wrapper_config_set_default_format(self.raw_mut_ptr(), format as u8);
        }
    }

    /// Get the default numeric format.
    pub fn get_default_format(&self) -> SettingFormat {
        let raw = unsafe { ffi::wrapper_config_get_default_format(self.raw_ptr()) };
        SettingFormat::from_raw(raw.into()).unwrap_or(SettingFormat::Default)
    }

    /// Set float precision (number of decimal digits) (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn set_float_precision(&mut self, digits: u16) {
        unsafe { ffi::config_set_float_precision(self.raw_mut_ptr(), digits as u8) }
    }

    /// Get float precision (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn get_float_precision(&self) -> u16 {
        unsafe { ffi::config_get_float_precision(self.raw_ptr()) as u16 }
    }

    /// Set tab width for indentation (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn set_tab_width(&mut self, width: u16) {
        unsafe { ffi::config_set_tab_width(self.raw_mut_ptr(), width as u8) }
    }

    /// Get tab width.
    pub fn get_tab_width(&self) -> u16 {
        unsafe { ffi::wrapper_config_get_tab_width(self.raw_ptr()) as u16 }
    }

    // ── Include directory (libconfig 1.4+) ──

    /// Set the include directory for @include directives (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn set_include_dir(&mut self, dir: &str) {
        if let Ok(c_dir) = CString::new(dir) {
            unsafe { ffi::config_set_include_dir(self.raw_mut_ptr(), c_dir.as_ptr()) }
        }
    }

    /// Get the include directory for @include directives.
    pub fn get_include_dir(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::wrapper_config_get_include_dir(self.raw_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    // ── Clear (libconfig 1.8+) ──

    /// Clear all settings from this config without destroying it (libconfig 1.8+).
    #[cfg(libconfig_1_8)]
    pub fn clear(&mut self) {
        unsafe { ffi::config_clear(self.raw_mut_ptr()) }
    }

    // ── Error state ──

    /// Get the current error text.
    pub fn error_text(&self) -> &str {
        unsafe {
            let ptr = ffi::wrapper_config_error_text(self.raw_ptr());
            if ptr.is_null() {
                ""
            } else {
                CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    /// Get the file where the last error occurred.
    pub fn error_file(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::wrapper_config_error_file(self.raw_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    /// Get the line number of the last error.
    pub fn error_line(&self) -> i32 {
        unsafe { ffi::wrapper_config_error_line(self.raw_ptr()) }
    }

    /// Get the type of the last error.
    pub fn error_type(&self) -> ErrorType {
        let raw = unsafe { ffi::wrapper_config_error_type(self.raw_ptr()) };
        ErrorType::from_raw(raw).unwrap_or(ErrorType::None)
    }

    // ── Internal ──

    fn error_state(&self) -> ConfigError {
        ConfigError::from_config(self.raw_ptr())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe {
            ffi::wrapper_config_free(self.raw.as_ptr());
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .finish()
    }
}