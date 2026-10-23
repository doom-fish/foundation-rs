use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

use crate::ffi;
use crate::util::{c_string, take_string};

#[derive(Clone, Copy, Debug)]
pub struct Bundle;

impl Bundle {
    #[must_use]
    pub const fn main() -> Self {
        Self
    }

    #[must_use]
    pub fn bundle_identifier(&self) -> Option<String> {
        unsafe { take_string(ffi::foundation_bundle_identifier()) }
    }

    #[must_use]
    pub fn bundle_path(&self) -> PathBuf {
        take_path(unsafe { ffi::foundation_bundle_path() })
            .expect("the main bundle has a file-system path")
    }

    #[must_use]
    pub fn url_for_auxiliary_executable(&self, name: &str) -> Option<PathBuf> {
        let name = c_string(name).ok()?;
        take_path(unsafe { ffi::foundation_bundle_url_for_auxiliary_executable(name.as_ptr()) })
    }
}

fn take_path(pointer: *mut core::ffi::c_char) -> Option<PathBuf> {
    if pointer.is_null() {
        return None;
    }
    let path = PathBuf::from(OsStr::from_bytes(
        unsafe { CStr::from_ptr(pointer) }.to_bytes(),
    ));
    unsafe { ffi::foundation_string_free(pointer) };
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::Bundle;

    #[test]
    fn main_bundle_contains_the_test_binary() {
        let bundle = Bundle::main();
        let path = bundle.bundle_path();
        assert!(path.is_absolute());
        assert!(path.is_dir());
        assert!(std::env::current_exe()
            .expect("test executable")
            .starts_with(path));
        assert!(bundle.bundle_identifier().is_none());
    }

    #[test]
    fn missing_auxiliary_executable_has_no_path() {
        let bundle = Bundle::main();
        let missing = format!("missing-auxiliary-executable-{}", std::process::id());
        assert!(bundle.url_for_auxiliary_executable(&missing).is_none());
        assert!(bundle
            .url_for_auxiliary_executable("invalid\0name")
            .is_none());
    }
}
