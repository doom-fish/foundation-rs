use core::ffi::c_char;
use std::ffi::{CStr, CString};

use crate::error::FoundationError;
use crate::ffi;

pub fn c_string(value: &str) -> Result<CString, FoundationError> {
    CString::new(value).map_err(|error| FoundationError::InvalidArgument(error.to_string()))
}

pub unsafe fn take_string(pointer: *mut c_char) -> Option<String> {
    if pointer.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(pointer) }.to_string_lossy().into_owned();
    unsafe { ffi::foundation_string_free(pointer) };
    Some(value)
}
