use core::ffi::c_char;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::error::FoundationError;
use crate::ffi;

pub fn c_string(value: &str) -> Result<CString, FoundationError> {
    CString::new(value).map_err(|error| FoundationError::InvalidArgument(error.to_string()))
}

pub unsafe fn take_string(pointer: *mut c_char) -> Option<String> {
    if pointer.is_null() {
        return None;
    }
    let value = unsafe { CStr::from_ptr(pointer) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::foundation_string_free(pointer) };
    Some(value)
}

pub unsafe fn take_path(pointer: *mut c_char) -> Option<PathBuf> {
    if pointer.is_null() {
        return None;
    }
    let path = PathBuf::from(OsStr::from_bytes(
        unsafe { CStr::from_ptr(pointer) }.to_bytes(),
    ));
    unsafe { ffi::foundation_string_free(pointer) };
    Some(path)
}
