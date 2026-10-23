use core::fmt;
use std::time::Duration;

use crate::ffi;
use crate::util::take_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperatingSystemVersion {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

impl OperatingSystemVersion {
    #[must_use]
    pub const fn new(major: i64, minor: i64, patch: i64) -> Self {
        Self { major, minor, patch }
    }
}

impl fmt::Display for OperatingSystemVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermalState {
    Nominal,
    Fair,
    Serious,
    Critical,
    Unknown(i64),
}

impl ThermalState {
    const fn from_raw(raw: i64) -> Self {
        match raw {
            0 => Self::Nominal,
            1 => Self::Fair,
            2 => Self::Serious,
            3 => Self::Critical,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessInfo;

impl ProcessInfo {
    #[must_use]
    pub fn operating_system_version_string() -> String {
        unsafe { take_string(ffi::foundation_process_info_operating_system_version_string()) }.unwrap_or_default()
    }

    #[must_use]
    pub fn operating_system_version() -> OperatingSystemVersion {
        let (mut major, mut minor, mut patch) = (0, 0, 0);
        unsafe {
            ffi::foundation_process_info_operating_system_version(&raw mut major, &raw mut minor, &raw mut patch);
        }
        OperatingSystemVersion { major, minor, patch }
    }

    #[must_use]
    pub fn is_operating_system_at_least(version: OperatingSystemVersion) -> bool {
        unsafe {
            ffi::foundation_process_info_is_operating_system_at_least(version.major, version.minor, version.patch)
        }
    }

    #[must_use]
    pub fn process_name() -> String {
        unsafe { take_string(ffi::foundation_process_info_process_name()) }.unwrap_or_default()
    }

    #[must_use]
    pub fn process_identifier() -> i32 {
        unsafe { ffi::foundation_process_info_process_identifier() }
    }

    #[must_use]
    pub fn processor_count() -> usize {
        usize::try_from(unsafe { ffi::foundation_process_info_processor_count() }).unwrap_or(0)
    }

    #[must_use]
    pub fn active_processor_count() -> usize {
        usize::try_from(unsafe { ffi::foundation_process_info_active_processor_count() }).unwrap_or(0)
    }

    #[must_use]
    pub fn physical_memory() -> u64 {
        unsafe { ffi::foundation_process_info_physical_memory() }
    }

    #[must_use]
    pub fn system_uptime() -> Duration {
        Duration::try_from_secs_f64(unsafe { ffi::foundation_process_info_system_uptime() }).unwrap_or_default()
    }

    #[must_use]
    pub fn is_low_power_mode_enabled() -> bool {
        unsafe { ffi::foundation_process_info_is_low_power_mode_enabled() }
    }

    #[must_use]
    pub fn thermal_state() -> ThermalState {
        ThermalState::from_raw(unsafe { ffi::foundation_process_info_thermal_state() })
    }
}
