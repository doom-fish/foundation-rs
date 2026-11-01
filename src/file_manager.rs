use core::ffi::c_char;
use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use crate::ffi;
use crate::util::{take_path, take_string};
use crate::FoundationError;

#[derive(Clone, Copy, Debug)]
pub struct FileManager;

impl FileManager {
    pub fn trash_item(path: &Path) -> Result<PathBuf, FoundationError> {
        with_file_path(path, |path, domain, code, message| unsafe {
            take_path(ffi::foundation_file_manager_trash_item(
                path, domain, code, message,
            ))
        })
    }

    pub fn set_excluded_from_backup(path: &Path, excluded: bool) -> Result<(), FoundationError> {
        with_file_path(path, |path, domain, code, message| {
            unsafe {
                ffi::foundation_file_manager_set_excluded_from_backup(
                    path, excluded, domain, code, message,
                )
            }
            .then_some(())
        })
    }

    pub fn is_excluded_from_backup(path: &Path) -> Result<bool, FoundationError> {
        with_file_path(path, |path, domain, code, message| {
            match unsafe {
                ffi::foundation_file_manager_is_excluded_from_backup(path, domain, code, message)
            } {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        })
    }
}

fn with_file_path<T>(
    path: &Path,
    operation: impl FnOnce(*const c_char, *mut *mut c_char, *mut i64, *mut *mut c_char) -> Option<T>,
) -> Result<T, FoundationError> {
    let bytes = path.as_os_str().as_bytes();
    if bytes.is_empty() {
        return Err(FoundationError::InvalidArgument("a file path is empty".into()));
    }
    let path = CString::new(bytes)
        .map_err(|error| FoundationError::InvalidArgument(error.to_string()))?;
    let mut domain = core::ptr::null_mut();
    let mut code = 0;
    let mut message = core::ptr::null_mut();
    let result = operation(
        path.as_ptr(),
        &raw mut domain,
        &raw mut code,
        &raw mut message,
    );
    let domain = unsafe { take_string(domain) };
    let message = unsafe { take_string(message) };
    result.ok_or_else(|| FoundationError::Native {
        domain: domain.unwrap_or_else(|| "NSCocoaErrorDomain".into()),
        code,
        message: message.unwrap_or_else(|| "The file operation failed.".into()),
    })
}
