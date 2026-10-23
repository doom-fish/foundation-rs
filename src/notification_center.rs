use core::ffi::c_void;
use core::ptr::NonNull;

use doom_fish_utils::callback_context::CallbackContext;

use crate::error::FoundationError;
use crate::ffi;
use crate::util::c_string;

type NotificationCallback = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Debug)]
pub struct NotificationCenter {
    handle: NonNull<c_void>,
}

unsafe impl Send for NotificationCenter {}
unsafe impl Sync for NotificationCenter {}

#[derive(Debug)]
pub struct NotificationObserver {
    handle: NonNull<c_void>,
    context: CallbackContext<NotificationCallback>,
}

unsafe impl Send for NotificationObserver {}

unsafe extern "C" fn invoke_notification(context: *mut c_void) {
    let _ = unsafe {
        CallbackContext::<NotificationCallback>::with(
            context,
            "foundation::notification_center",
            |callback| {
                callback();
            },
        )
    };
}

impl NotificationCenter {
    #[must_use]
    pub fn default_center() -> Self {
        let handle = unsafe { ffi::foundation_notification_center_default() };
        Self {
            handle: NonNull::new(handle).expect("every process has a default notification center"),
        }
    }

    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let handle = unsafe { ffi::foundation_notification_center_new() };
        Self {
            handle: NonNull::new(handle).expect("notification centers return a center"),
        }
    }

    pub fn add_observer(
        &self,
        name: &str,
        callback: impl Fn() + Send + Sync + 'static,
    ) -> Result<NotificationObserver, FoundationError> {
        let name = c_string(name)?;
        let callback: NotificationCallback = Box::new(callback);
        let context = CallbackContext::new(callback);
        let handle = unsafe {
            ffi::foundation_notification_center_add_observer(
                self.handle.as_ptr(),
                name.as_ptr(),
                invoke_notification,
                context.as_ptr(),
                CallbackContext::<NotificationCallback>::RETAIN,
                CallbackContext::<NotificationCallback>::RELEASE,
            )
        };
        Ok(NotificationObserver {
            handle: NonNull::new(handle).expect("adding an observer returns a registration"),
            context,
        })
    }

    pub fn post(&self, name: &str) -> Result<(), FoundationError> {
        let name = c_string(name)?;
        unsafe { ffi::foundation_notification_center_post(self.handle.as_ptr(), name.as_ptr()) };
        Ok(())
    }
}

impl Clone for NotificationCenter {
    fn clone(&self) -> Self {
        unsafe { ffi::foundation_notification_center_retain(self.handle.as_ptr()) };
        Self {
            handle: self.handle,
        }
    }
}

impl Drop for NotificationCenter {
    fn drop(&mut self) {
        unsafe { ffi::foundation_notification_center_release(self.handle.as_ptr()) };
    }
}

impl Drop for NotificationObserver {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { ffi::foundation_notification_observer_release(self.handle.as_ptr()) };
    }
}
