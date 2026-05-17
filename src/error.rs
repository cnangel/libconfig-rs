use std::ffi::CStr;
use std::fmt;

use crate::constants::ErrorType;
use crate::ffi;

/// Error returned by libconfig operations.
#[derive(Debug, Clone)]
pub struct ConfigError {
    pub message: String,
    pub file: Option<String>,
    pub line: i32,
    pub error_type: ErrorType,
}

impl ConfigError {
    /// Read error state from a config pointer using C wrapper functions.
    pub(crate) fn from_config(config: *const ffi::config_t) -> Self {
        let message = unsafe {
            let ptr = ffi::wrapper_config_error_text(config);
            if ptr.is_null() {
                "unknown error".to_string()
            } else {
                CStr::from_ptr(ptr).to_string_lossy().into_owned()
            }
        };

        let line = unsafe { ffi::wrapper_config_error_line(config) };

        let file = unsafe {
            let ptr = ffi::wrapper_config_error_file(config);
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        };

        let error_type = unsafe {
            let et = ffi::wrapper_config_error_type(config);
            ErrorType::from_raw(et).unwrap_or(ErrorType::None)
        };

        ConfigError {
            message,
            file,
            line,
            error_type,
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(ref file) = self.file {
            write!(f, " ({}:{})", file, self.line)?;
        }
        Ok(())
    }
}

impl std::error::Error for ConfigError {}