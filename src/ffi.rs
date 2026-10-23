use core::ffi::{c_char, c_void};

extern "C" {
    pub fn foundation_string_free(string: *mut c_char);

    pub fn foundation_user_defaults_standard() -> *mut c_void;
    pub fn foundation_user_defaults_with_suite_name(name: *const c_char) -> *mut c_void;
    pub fn foundation_user_defaults_release(defaults: *mut c_void);
    pub fn foundation_user_defaults_register_bool(
        defaults: *mut c_void,
        key: *const c_char,
        value: bool,
    );
    pub fn foundation_user_defaults_register_integer(
        defaults: *mut c_void,
        key: *const c_char,
        value: i64,
    );
    pub fn foundation_user_defaults_register_double(
        defaults: *mut c_void,
        key: *const c_char,
        value: f64,
    );
    pub fn foundation_user_defaults_register_string(
        defaults: *mut c_void,
        key: *const c_char,
        value: *const c_char,
    );
    pub fn foundation_user_defaults_set_bool(
        defaults: *mut c_void,
        key: *const c_char,
        value: bool,
    );
    pub fn foundation_user_defaults_set_integer(
        defaults: *mut c_void,
        key: *const c_char,
        value: i64,
    );
    pub fn foundation_user_defaults_set_double(
        defaults: *mut c_void,
        key: *const c_char,
        value: f64,
    );
    pub fn foundation_user_defaults_set_string(
        defaults: *mut c_void,
        key: *const c_char,
        value: *const c_char,
    );
    pub fn foundation_user_defaults_bool(defaults: *mut c_void, key: *const c_char) -> bool;
    pub fn foundation_user_defaults_integer(defaults: *mut c_void, key: *const c_char) -> i64;
    pub fn foundation_user_defaults_double(defaults: *mut c_void, key: *const c_char) -> f64;
    pub fn foundation_user_defaults_string(
        defaults: *mut c_void,
        key: *const c_char,
    ) -> *mut c_char;
    pub fn foundation_user_defaults_contains(defaults: *mut c_void, key: *const c_char) -> bool;
    pub fn foundation_user_defaults_remove(defaults: *mut c_void, key: *const c_char);

    pub fn foundation_process_info_operating_system_version_string() -> *mut c_char;
    pub fn foundation_process_info_operating_system_version(
        major: *mut i64,
        minor: *mut i64,
        patch: *mut i64,
    );
    pub fn foundation_process_info_is_operating_system_at_least(
        major: i64,
        minor: i64,
        patch: i64,
    ) -> bool;
    pub fn foundation_process_info_process_name() -> *mut c_char;
    pub fn foundation_process_info_process_identifier() -> i32;
    pub fn foundation_process_info_processor_count() -> i64;
    pub fn foundation_process_info_active_processor_count() -> i64;
    pub fn foundation_process_info_physical_memory() -> u64;
    pub fn foundation_process_info_system_uptime() -> f64;
    pub fn foundation_process_info_is_low_power_mode_enabled() -> bool;
    pub fn foundation_process_info_thermal_state() -> i64;
    pub fn foundation_process_info_begin_activity(
        options: u64,
        reason: *const c_char,
    ) -> *mut c_void;
    pub fn foundation_process_info_end_activity(activity: *mut c_void);

    pub fn foundation_notification_center_default() -> *mut c_void;
    pub fn foundation_notification_center_new() -> *mut c_void;
    pub fn foundation_notification_center_retain(center: *mut c_void);
    pub fn foundation_notification_center_release(center: *mut c_void);
    pub fn foundation_notification_center_add_observer(
        center: *mut c_void,
        name: *const c_char,
        callback: unsafe extern "C" fn(*mut c_void),
        context: *mut c_void,
        retain: unsafe extern "C" fn(*mut c_void),
        release: unsafe extern "C" fn(*mut c_void),
    ) -> *mut c_void;
    pub fn foundation_notification_observer_release(observer: *mut c_void);
    pub fn foundation_notification_center_post(center: *mut c_void, name: *const c_char);
}
