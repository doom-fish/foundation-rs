use core::ffi::c_void;
use core::fmt;
use core::ops::{BitOr, BitOrAssign};
use core::ptr::NonNull;
use std::time::Duration;

use crate::error::FoundationError;
use crate::ffi;
use crate::util::{c_string, take_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperatingSystemVersion {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

impl OperatingSystemVersion {
    #[must_use]
    pub const fn new(major: i64, minor: i64, patch: i64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct ActivityOptions(u64);

impl ActivityOptions {
    pub const IDLE_DISPLAY_SLEEP_DISABLED: Self = Self(1 << 40);
    pub const IDLE_SYSTEM_SLEEP_DISABLED: Self = Self(1 << 20);
    pub const SUDDEN_TERMINATION_DISABLED: Self = Self(1 << 14);
    pub const AUTOMATIC_TERMINATION_DISABLED: Self = Self(1 << 15);
    pub const ANIMATION_TRACKING_ENABLED: Self = Self(1 << 45);
    pub const TRACKING_ENABLED: Self = Self(1 << 46);
    pub const USER_INITIATED: Self = Self(0x00ff_ffff | Self::IDLE_SYSTEM_SLEEP_DISABLED.0);
    pub const USER_INITIATED_ALLOWING_IDLE_SYSTEM_SLEEP: Self =
        Self(Self::USER_INITIATED.0 & !Self::IDLE_SYSTEM_SLEEP_DISABLED.0);
    pub const BACKGROUND: Self = Self(0x0000_00ff);
    pub const LATENCY_CRITICAL: Self = Self(0xff_0000_0000);
    pub const USER_INTERACTIVE: Self = Self(Self::USER_INITIATED.0 | Self::LATENCY_CRITICAL.0);

    #[must_use]
    pub const fn bits(self) -> u64 {
        self.0
    }

    #[must_use]
    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }
}

impl BitOr for ActivityOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ActivityOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Debug)]
pub struct Activity {
    handle: NonNull<c_void>,
}

unsafe impl Send for Activity {}
unsafe impl Sync for Activity {}

impl Drop for Activity {
    fn drop(&mut self) {
        unsafe { ffi::foundation_process_info_end_activity(self.handle.as_ptr()) };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ProcessInfo;

impl ProcessInfo {
    pub const THERMAL_STATE_DID_CHANGE_NOTIFICATION: &'static str =
        "NSProcessInfoThermalStateDidChangeNotification";
    pub const POWER_STATE_DID_CHANGE_NOTIFICATION: &'static str =
        "NSProcessInfoPowerStateDidChangeNotification";

    #[must_use]
    pub fn operating_system_version_string() -> String {
        unsafe { take_string(ffi::foundation_process_info_operating_system_version_string()) }
            .unwrap_or_default()
    }

    #[must_use]
    pub fn operating_system_version() -> OperatingSystemVersion {
        let (mut major, mut minor, mut patch) = (0, 0, 0);
        unsafe {
            ffi::foundation_process_info_operating_system_version(
                &raw mut major,
                &raw mut minor,
                &raw mut patch,
            );
        }
        OperatingSystemVersion {
            major,
            minor,
            patch,
        }
    }

    #[must_use]
    pub fn is_operating_system_at_least(version: OperatingSystemVersion) -> bool {
        unsafe {
            ffi::foundation_process_info_is_operating_system_at_least(
                version.major,
                version.minor,
                version.patch,
            )
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
        usize::try_from(unsafe { ffi::foundation_process_info_active_processor_count() })
            .unwrap_or(0)
    }

    #[must_use]
    pub fn physical_memory() -> u64 {
        unsafe { ffi::foundation_process_info_physical_memory() }
    }

    #[must_use]
    pub fn system_uptime() -> Duration {
        Duration::try_from_secs_f64(unsafe { ffi::foundation_process_info_system_uptime() })
            .unwrap_or_default()
    }

    #[must_use]
    pub fn is_low_power_mode_enabled() -> bool {
        unsafe { ffi::foundation_process_info_is_low_power_mode_enabled() }
    }

    #[must_use]
    pub fn thermal_state() -> ThermalState {
        ThermalState::from_raw(unsafe { ffi::foundation_process_info_thermal_state() })
    }

    pub fn begin_activity(
        options: ActivityOptions,
        reason: &str,
    ) -> Result<Activity, FoundationError> {
        if reason.is_empty() {
            return Err(FoundationError::InvalidArgument(
                "activity reason must not be empty".to_owned(),
            ));
        }
        let reason = c_string(reason)?;
        let handle =
            unsafe { ffi::foundation_process_info_begin_activity(options.bits(), reason.as_ptr()) };
        Ok(Activity {
            handle: NonNull::new(handle).expect("process activities return an activity"),
        })
    }
}
