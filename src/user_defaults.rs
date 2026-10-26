use core::ffi::{c_char, c_void};
use core::ptr::NonNull;
use std::ffi::CString;

use crate::error::FoundationError;
use crate::ffi;
use crate::util::{c_string, take_string};

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultValue {
    Bool(bool),
    Integer(i64),
    Double(f64),
    String(String),
}

impl From<bool> for DefaultValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<i64> for DefaultValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}

impl From<f64> for DefaultValue {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<&str> for DefaultValue {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}

impl From<String> for DefaultValue {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

enum NativeValue {
    Bool(bool),
    Integer(i64),
    Double(f64),
    String(CString),
}

impl NativeValue {
    fn new(value: &DefaultValue) -> Result<Self, FoundationError> {
        Ok(match value {
            DefaultValue::Bool(value) => Self::Bool(*value),
            DefaultValue::Integer(value) => Self::Integer(*value),
            DefaultValue::Double(value) => Self::Double(*value),
            DefaultValue::String(value) => Self::String(c_string(value)?),
        })
    }
}

#[derive(Debug)]
pub struct UserDefaults {
    handle: NonNull<c_void>,
}

unsafe impl Send for UserDefaults {}
unsafe impl Sync for UserDefaults {}

impl UserDefaults {
    #[must_use]
    pub fn standard() -> Self {
        let handle = unsafe { ffi::foundation_user_defaults_standard() };
        Self {
            handle: NonNull::new(handle).expect("every process has standard user defaults"),
        }
    }

    pub fn with_suite_name(name: &str) -> Result<Self, FoundationError> {
        let suite = c_string(name)?;
        let handle = unsafe { ffi::foundation_user_defaults_with_suite_name(suite.as_ptr()) };
        NonNull::new(handle)
            .map(|handle| Self { handle })
            .ok_or_else(|| {
                FoundationError::InvalidArgument(format!("{name} cannot be a suite name"))
            })
    }

    pub fn register(&self, defaults: &[(&str, DefaultValue)]) -> Result<(), FoundationError> {
        let entries = defaults
            .iter()
            .map(|(key, value)| Ok((c_string(key)?, NativeValue::new(value)?)))
            .collect::<Result<Vec<_>, FoundationError>>()?;
        let handle = self.handle.as_ptr();
        for (key, value) in &entries {
            let key = key.as_ptr();
            unsafe {
                match value {
                    NativeValue::Bool(value) => {
                        ffi::foundation_user_defaults_register_bool(handle, key, *value);
                    }
                    NativeValue::Integer(value) => {
                        ffi::foundation_user_defaults_register_integer(handle, key, *value);
                    }
                    NativeValue::Double(value) => {
                        ffi::foundation_user_defaults_register_double(handle, key, *value);
                    }
                    NativeValue::String(value) => {
                        ffi::foundation_user_defaults_register_string(handle, key, value.as_ptr());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn set(&self, key: &str, value: &DefaultValue) -> Result<(), FoundationError> {
        let value = NativeValue::new(value)?;
        self.with_key(key, |handle, key| unsafe {
            match &value {
                NativeValue::Bool(value) => {
                    ffi::foundation_user_defaults_set_bool(handle, key, *value);
                }
                NativeValue::Integer(value) => {
                    ffi::foundation_user_defaults_set_integer(handle, key, *value);
                }
                NativeValue::Double(value) => {
                    ffi::foundation_user_defaults_set_double(handle, key, *value);
                }
                NativeValue::String(value) => {
                    ffi::foundation_user_defaults_set_string(handle, key, value.as_ptr());
                }
            }
        })
    }

    pub fn bool(&self, key: &str) -> Result<bool, FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            ffi::foundation_user_defaults_bool(handle, key)
        })
    }

    pub fn integer(&self, key: &str) -> Result<i64, FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            ffi::foundation_user_defaults_integer(handle, key)
        })
    }

    pub fn double(&self, key: &str) -> Result<f64, FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            ffi::foundation_user_defaults_double(handle, key)
        })
    }

    pub fn string(&self, key: &str) -> Result<Option<String>, FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            take_string(ffi::foundation_user_defaults_string(handle, key))
        })
    }

    pub fn contains(&self, key: &str) -> Result<bool, FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            ffi::foundation_user_defaults_contains(handle, key)
        })
    }

    pub fn remove(&self, key: &str) -> Result<(), FoundationError> {
        self.with_key(key, |handle, key| unsafe {
            ffi::foundation_user_defaults_remove(handle, key);
        })
    }

    fn with_key<T>(
        &self,
        key: &str,
        call: impl FnOnce(*mut c_void, *const c_char) -> T,
    ) -> Result<T, FoundationError> {
        let key = c_string(key)?;
        Ok(call(self.handle.as_ptr(), key.as_ptr()))
    }
}

impl Drop for UserDefaults {
    fn drop(&mut self) {
        unsafe { ffi::foundation_user_defaults_release(self.handle.as_ptr()) };
    }
}
