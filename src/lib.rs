#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(target_os = "macos")]

mod autorelease;
pub mod bundle;
pub mod error;
pub mod ffi;
pub mod notification_center;
pub mod process_info;
pub mod user_defaults;
mod util;

#[cfg(feature = "network")]
pub mod network;

pub use autorelease::autoreleasepool;
pub use bundle::Bundle;
pub use error::FoundationError;
pub use notification_center::{NotificationCenter, NotificationObserver};
pub use process_info::{
    Activity, ActivityOptions, OperatingSystemVersion, ProcessInfo, ThermalState,
};
pub use user_defaults::{DefaultValue, UserDefaults};

pub mod prelude {
    pub use crate::autorelease::autoreleasepool;
    pub use crate::bundle::Bundle;
    pub use crate::error::FoundationError;
    pub use crate::notification_center::{NotificationCenter, NotificationObserver};
    pub use crate::process_info::{
        Activity, ActivityOptions, OperatingSystemVersion, ProcessInfo, ThermalState,
    };
    pub use crate::user_defaults::{DefaultValue, UserDefaults};
}
