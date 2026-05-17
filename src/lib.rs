mod ffi;
mod config;
mod setting;
mod error;
pub mod constants;

pub use config::Config;
pub use setting::{Setting, SettingIter};
pub use error::ConfigError;
pub use constants::{ConfigOptions, ErrorType, SettingFormat, SettingType};

/// Return the libconfig version string this crate was compiled against.
pub fn version() -> &'static str {
    option_env!("libconfig_ver").unwrap_or("unknown")
}