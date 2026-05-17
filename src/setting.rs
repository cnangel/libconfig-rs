use std::ffi::{CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[allow(unused_imports)]
use crate::constants::{SettingFormat, SettingType};
use crate::ffi;

/// A reference to a `config_setting_t` owned by a parent `Config`.
///
/// The lifetime `'config` ensures the parent `Config` outlives this reference.
/// Destroying the `Config` invalidates all `Setting` references — the borrow
/// checker prevents use-after-free at compile time.
pub struct Setting<'config> {
    pub(crate) ptr: NonNull<ffi::config_setting_t>,
    _phantom: PhantomData<&'config ffi::config_t>,
}

impl<'config> Setting<'config> {
    /// Create a Setting from a raw pointer. Returns None if ptr is null.
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::config_setting_t) -> Option<Self> {
        NonNull::new(ptr).map(|p| Setting {
            ptr: p,
            _phantom: PhantomData,
        })
    }

    /// Create a Setting from a non-null pointer (caller guarantees non-null).
    pub(crate) unsafe fn from_ptr_unchecked(ptr: *mut ffi::config_setting_t) -> Self {
        Setting {
            ptr: NonNull::new_unchecked(ptr),
            _phantom: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *const ffi::config_setting_t {
        self.ptr.as_ptr() as *const _
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut ffi::config_setting_t {
        self.ptr.as_ptr()
    }

    // ── Type queries ──

    /// Returns the type of this setting.
    pub fn get_type(&self) -> SettingType {
        let raw = unsafe { ffi::wrapper_config_setting_type(self.as_ptr()) };
        SettingType::from_raw(raw).unwrap_or(SettingType::None)
    }

    /// Returns true if this setting is a group.
    pub fn is_group(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_group(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is an array.
    pub fn is_array(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_array(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is a list.
    pub fn is_list(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_list(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is a number (int, int64, or float).
    pub fn is_number(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_number(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is a scalar value.
    pub fn is_scalar(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_scalar(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is an aggregate (group, array, or list).
    pub fn is_aggregate(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_aggregate(self.as_ptr()) != 0 }
    }

    /// Returns true if this setting is a string.
    pub fn is_string(&self) -> bool {
        self.get_type() == SettingType::String
    }

    // ── Properties ──

    /// Returns the name of this setting, or None if it is the root.
    pub fn get_name(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::wrapper_config_setting_name(self.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    /// Returns the index of this setting within its parent.
    pub fn get_index(&self) -> i32 {
        unsafe { ffi::config_setting_index(self.as_ptr()) }
    }

    /// Returns the parent setting, or None if this is the root.
    pub fn parent(&self) -> Option<Setting<'config>> {
        unsafe {
            let ptr = ffi::wrapper_config_setting_parent(self.as_ptr());
            Setting::from_ptr(ptr)
        }
    }

    /// Returns true if this setting is the root.
    pub fn is_root(&self) -> bool {
        unsafe { ffi::wrapper_config_setting_is_root(self.as_ptr()) != 0 }
    }

    /// Returns the number of children (for groups, arrays, lists).
    pub fn len(&self) -> u32 {
        unsafe { ffi::config_setting_length(self.as_ptr()) as u32 }
    }

    /// Returns true if this setting has no children.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the line number in the source file where this setting is defined.
    pub fn source_line(&self) -> u32 {
        unsafe { ffi::wrapper_config_setting_source_line(self.as_ptr()) }
    }

    /// Returns the source file name, if available (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn source_file(&self) -> Option<&str> {
        unsafe {
            let ptr = ffi::wrapper_config_setting_source_file(self.as_ptr());
            if ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    // ── Value getters ──

    /// Returns the integer value of this setting.
    pub fn get_int(&self) -> i32 {
        unsafe { ffi::config_setting_get_int(self.as_ptr()) }
    }

    /// Returns the 64-bit integer value of this setting.
    pub fn get_int64(&self) -> i64 {
        unsafe { ffi::config_setting_get_int64(self.as_ptr()) }
    }

    /// Returns the float value of this setting.
    pub fn get_float(&self) -> f64 {
        unsafe { ffi::config_setting_get_float(self.as_ptr()) }
    }

    /// Returns the boolean value of this setting.
    pub fn get_bool(&self) -> bool {
        unsafe { ffi::config_setting_get_bool(self.as_ptr()) != 0 }
    }

    /// Returns the string value of this setting.
    pub fn get_string(&self) -> &str {
        unsafe {
            let ptr = ffi::config_setting_get_string(self.as_ptr());
            if ptr.is_null() {
                ""
            } else {
                CStr::from_ptr(ptr).to_str().unwrap_or("")
            }
        }
    }

    // ── Safe getters (libconfig 1.8+) ──

    /// Returns Some(value) if the setting is an integer type, None otherwise.
    #[cfg(libconfig_1_8)]
    pub fn get_int_safe(&self) -> Option<i32> {
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_setting_get_int_safe(self.as_ptr(), &mut value) };
        if ret != 0 { Some(value) } else { None }
    }

    /// Returns Some(value) if the setting is an int64 type, None otherwise.
    #[cfg(libconfig_1_8)]
    pub fn get_int64_safe(&self) -> Option<i64> {
        let mut value: i64 = 0;
        let ret = unsafe { ffi::config_setting_get_int64_safe(self.as_ptr(), &mut value) };
        if ret != 0 { Some(value) } else { None }
    }

    /// Returns Some(value) if the setting is a float type, None otherwise.
    #[cfg(libconfig_1_8)]
    pub fn get_float_safe(&self) -> Option<f64> {
        let mut value: f64 = 0.0;
        let ret = unsafe { ffi::config_setting_get_float_safe(self.as_ptr(), &mut value) };
        if ret != 0 { Some(value) } else { None }
    }

    /// Returns Some(value) if the setting is a boolean type, None otherwise.
    #[cfg(libconfig_1_8)]
    pub fn get_bool_safe(&self) -> Option<bool> {
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_setting_get_bool_safe(self.as_ptr(), &mut value) };
        if ret != 0 { Some(value != 0) } else { None }
    }

    /// Returns Some(value) if the setting is a string type, None otherwise.
    #[cfg(libconfig_1_8)]
    pub fn get_string_safe(&self) -> Option<&str> {
        let mut value: *const std::os::raw::c_char = std::ptr::null();
        let ret = unsafe { ffi::config_setting_get_string_safe(self.as_ptr(), &mut value) };
        if ret != 0 && !value.is_null() {
            unsafe { Some(CStr::from_ptr(value).to_str().unwrap_or("")) }
        } else {
            None
        }
    }

    // ── Value setters ──

    /// Set the integer value of this setting.
    pub fn set_int(&mut self, value: i32) -> Result<(), crate::ConfigError> {
        let ret = unsafe { ffi::config_setting_set_int(self.as_mut_ptr(), value) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    /// Set the 64-bit integer value of this setting.
    pub fn set_int64(&mut self, value: i64) -> Result<(), crate::ConfigError> {
        let ret = unsafe { ffi::config_setting_set_int64(self.as_mut_ptr(), value) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    /// Set the float value of this setting.
    pub fn set_float(&mut self, value: f64) -> Result<(), crate::ConfigError> {
        let ret = unsafe { ffi::config_setting_set_float(self.as_mut_ptr(), value) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    /// Set the boolean value of this setting.
    pub fn set_bool(&mut self, value: bool) -> Result<(), crate::ConfigError> {
        let v: i32 = if value { 1 } else { 0 };
        let ret = unsafe { ffi::config_setting_set_bool(self.as_mut_ptr(), v) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    /// Set the string value of this setting.
    pub fn set_string(&mut self, value: &str) -> Result<(), crate::ConfigError> {
        let c_str = CString::new(value).map_err(|_| self.make_error())?;
        let ret = unsafe { ffi::config_setting_set_string(self.as_mut_ptr(), c_str.as_ptr()) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    // ── Element access ──

    /// Get the child element at the given index.
    pub fn get_elem(&self, idx: u32) -> Option<Setting<'config>> {
        unsafe {
            let ptr = ffi::config_setting_get_elem(self.as_ptr(), idx as i32);
            Setting::from_ptr(ptr)
        }
    }

    /// Get a child member by name.
    pub fn get_member(&self, name: &str) -> Option<Setting<'config>> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::config_setting_get_member(self.as_ptr(), c_name.as_ptr());
            Setting::from_ptr(ptr)
        }
    }

    /// Check if a child member with the given name exists.
    pub fn exists(&self, name: &str) -> bool {
        self.get_member(name).is_some()
    }

    /// Get the integer value of the element at the given index.
    pub fn get_int_elem(&self, idx: u32) -> Option<i32> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { ffi::config_setting_get_int_elem(self.as_ptr(), idx as i32) })
    }

    /// Get the int64 value of the element at the given index.
    pub fn get_int64_elem(&self, idx: u32) -> Option<i64> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { ffi::config_setting_get_int64_elem(self.as_ptr(), idx as i32) })
    }

    /// Get the float value of the element at the given index.
    pub fn get_float_elem(&self, idx: u32) -> Option<f64> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { ffi::config_setting_get_float_elem(self.as_ptr(), idx as i32) })
    }

    /// Get the boolean value of the element at the given index.
    pub fn get_bool_elem(&self, idx: u32) -> Option<bool> {
        if idx >= self.len() {
            return None;
        }
        Some(unsafe { ffi::config_setting_get_bool_elem(self.as_ptr(), idx as i32) != 0 })
    }

    /// Get the string value of the element at the given index.
    pub fn get_string_elem(&self, idx: u32) -> Option<&str> {
        if idx >= self.len() {
            return None;
        }
        unsafe {
            let ptr = ffi::config_setting_get_string_elem(self.as_ptr(), idx as i32);
            if ptr.is_null() {
                Some("")
            } else {
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    // ── Setting-level lookup ──

    /// Look up a child setting by path from this setting (libconfig 1.5+).
    #[cfg(libconfig_1_5)]
    pub fn lookup(&self, path: &str) -> Option<Setting<'config>> {
        let c_path = CString::new(path).ok()?;
        unsafe {
            let ptr = ffi::config_setting_lookup(self.as_ptr(), c_path.as_ptr());
            Setting::from_ptr(ptr)
        }
    }

    /// Look up an integer value by name from this setting (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn lookup_int(&self, name: &str) -> Option<i32> {
        let c_name = CString::new(name).ok()?;
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_setting_lookup_int(self.as_ptr(), c_name.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE { Some(value) } else { None }
    }

    /// Look up an int64 value by name from this setting (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn lookup_int64(&self, name: &str) -> Option<i64> {
        let c_name = CString::new(name).ok()?;
        let mut value: i64 = 0;
        let ret = unsafe { ffi::config_setting_lookup_int64(self.as_ptr(), c_name.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE { Some(value) } else { None }
    }

    /// Look up a float value by name from this setting (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn lookup_float(&self, name: &str) -> Option<f64> {
        let c_name = CString::new(name).ok()?;
        let mut value: f64 = 0.0;
        let ret = unsafe { ffi::config_setting_lookup_float(self.as_ptr(), c_name.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE { Some(value) } else { None }
    }

    /// Look up a boolean value by name from this setting (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn lookup_bool(&self, name: &str) -> Option<bool> {
        let c_name = CString::new(name).ok()?;
        let mut value: i32 = 0;
        let ret = unsafe { ffi::config_setting_lookup_bool(self.as_ptr(), c_name.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE { Some(value != 0) } else { None }
    }

    /// Look up a string value by name from this setting (libconfig 1.4+).
    #[cfg(libconfig_1_4)]
    pub fn lookup_string(&self, name: &str) -> Option<&str> {
        let c_name = CString::new(name).ok()?;
        let mut value: *const std::os::raw::c_char = std::ptr::null();
        let ret = unsafe { ffi::config_setting_lookup_string(self.as_ptr(), c_name.as_ptr(), &mut value) };
        if ret == ffi::CONFIG_TRUE && !value.is_null() {
            unsafe { Some(CStr::from_ptr(value).to_str().unwrap_or("")) }
        } else {
            None
        }
    }

    // ── Mutation ──

    /// Add a child setting with the given name and type.
    pub fn add(&mut self, name: &str, ty: SettingType) -> Option<Setting<'config>> {
        let c_name = CString::new(name).ok()?;
        unsafe {
            let ptr = ffi::config_setting_add(self.as_mut_ptr(), c_name.as_ptr(), ty as i32);
            Setting::from_ptr(ptr)
        }
    }

    /// Remove a child setting by name.
    pub fn remove(&mut self, name: &str) -> Result<(), crate::ConfigError> {
        let c_name = CString::new(name).map_err(|_| self.make_error())?;
        let ret = unsafe { ffi::config_setting_remove(self.as_mut_ptr(), c_name.as_ptr()) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    /// Remove a child element at the given index.
    pub fn remove_elem(&mut self, idx: u32) -> Result<(), crate::ConfigError> {
        let ret = unsafe { ffi::config_setting_remove_elem(self.as_mut_ptr(), idx as i32) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    // ── Format ──

    /// Get the format of this setting (libconfig 1.5+).
    #[cfg(libconfig_1_5)]
    pub fn get_format(&self) -> SettingFormat {
        let raw = unsafe { ffi::config_setting_get_format(self.as_ptr()) };
        SettingFormat::from_raw(raw.into()).unwrap_or(SettingFormat::Default)
    }

    /// Set the format of this setting (libconfig 1.5+).
    #[cfg(libconfig_1_5)]
    pub fn set_format(&mut self, format: SettingFormat) -> Result<(), crate::ConfigError> {
        let ret = unsafe { ffi::config_setting_set_format(self.as_mut_ptr(), format as u8) };
        if ret == ffi::CONFIG_TRUE {
            Ok(())
        } else {
            Err(self.make_error())
        }
    }

    // ── Internal helpers ──

    fn make_error(&self) -> crate::ConfigError {
        crate::ConfigError {
            message: "setting operation failed".into(),
            file: None,
            line: 0,
            error_type: crate::constants::ErrorType::None,
        }
    }
}

// ── Iterator ──

impl<'config> IntoIterator for &'config Setting<'config> {
    type Item = Setting<'config>;
    type IntoIter = SettingIter<'config>;

    fn into_iter(self) -> Self::IntoIter {
        SettingIter {
            setting: self,
            index: 0,
            len: self.len(),
        }
    }
}

/// Iterator over child settings.
pub struct SettingIter<'config> {
    setting: &'config Setting<'config>,
    index: u32,
    len: u32,
}

impl<'config> Iterator for SettingIter<'config> {
    type Item = Setting<'config>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.len {
            return None;
        }
        let child = self.setting.get_elem(self.index);
        self.index += 1;
        child
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.len - self.index) as usize;
        (remaining, Some(remaining))
    }
}

impl<'config> ExactSizeIterator for SettingIter<'config> {}

// ── Debug / Display ──

impl<'config> fmt::Debug for Setting<'config> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.get_name().unwrap_or("(root)");
        let ty = self.get_type();
        f.debug_struct("Setting")
            .field("name", &name)
            .field("type", &ty)
            .field("len", &self.len())
            .finish()
    }
}