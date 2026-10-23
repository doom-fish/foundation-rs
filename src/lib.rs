#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod process_info;
pub mod user_defaults;
mod util;

pub use error::FoundationError;
pub use process_info::{OperatingSystemVersion, ProcessInfo, ThermalState};
pub use user_defaults::{DefaultValue, UserDefaults};

pub mod prelude {
    pub use crate::error::FoundationError;
    pub use crate::process_info::{OperatingSystemVersion, ProcessInfo, ThermalState};
    pub use crate::user_defaults::{DefaultValue, UserDefaults};
}
