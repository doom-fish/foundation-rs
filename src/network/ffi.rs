use core::ffi::{c_char, c_void};

pub type ContextFunction = unsafe extern "C" fn(*mut c_void);
pub type EventFunction = unsafe extern "C" fn(*mut c_void, *mut c_void);

extern "C" {
    pub fn foundation_network_queue_new(name: *const c_char) -> *mut c_void;
    pub fn foundation_network_queue_release(queue: *mut c_void);
    pub fn foundation_network_queue_is_current(queue: *mut c_void) -> bool;
    pub fn foundation_network_session_new(
        queue: *mut c_void,
        idle_timeout: f64,
        resource_timeout: f64,
        connections: i32,
        proxy_policy: i32,
    ) -> *mut c_void;
    pub fn foundation_network_session_release(session: *mut c_void);
    pub fn foundation_network_session_cancel(session: *mut c_void);
    pub fn foundation_network_request_new(
        url: *const c_char,
        method: *const c_char,
        timeout: f64,
    ) -> *mut c_void;
    pub fn foundation_network_request_release(request: *mut c_void);
    pub fn foundation_network_request_add_header(
        request: *mut c_void,
        name: *const c_char,
        value: *const c_char,
    );
    pub fn foundation_network_request_set_body(
        request: *mut c_void,
        bytes: *const u8,
        count: usize,
    );
    pub fn foundation_network_request_url(request: *mut c_void) -> *mut c_char;
    pub fn foundation_network_request_method(request: *mut c_void) -> *mut c_char;
    pub fn foundation_network_request_timeout(request: *mut c_void) -> f64;
    pub fn foundation_network_request_header_count(request: *mut c_void) -> usize;
    pub fn foundation_network_request_header_name(
        request: *mut c_void,
        index: usize,
    ) -> *mut c_char;
    pub fn foundation_network_request_header_value(
        request: *mut c_void,
        index: usize,
    ) -> *mut c_char;
    pub fn foundation_network_task_new(
        session: *mut c_void,
        request: *mut c_void,
        kind: i32,
        deadline: f64,
        message_limit: usize,
        context: *mut c_void,
        callback: EventFunction,
        retain: ContextFunction,
        release: ContextFunction,
    ) -> *mut c_void;
    pub fn foundation_network_task_release(task: *mut c_void);
    pub fn foundation_network_task_resume(task: *mut c_void);
    pub fn foundation_network_task_suspend(task: *mut c_void);
    pub fn foundation_network_task_cancel(task: *mut c_void);
    pub fn foundation_network_task_upload(task: *mut c_void, length: i64) -> *mut c_void;
    pub fn foundation_network_upload_write(
        upload: *mut c_void,
        bytes: *const u8,
        count: usize,
    ) -> isize;
    pub fn foundation_network_upload_finish(upload: *mut c_void) -> bool;
    pub fn foundation_network_upload_release(upload: *mut c_void);
    pub fn foundation_network_websocket_receive(task: *mut c_void) -> bool;
    pub fn foundation_network_websocket_send(
        task: *mut c_void,
        operation: i64,
        text: bool,
        bytes: *const u8,
        count: usize,
    ) -> bool;
    pub fn foundation_network_websocket_ping(task: *mut c_void, operation: i64) -> bool;
    pub fn foundation_network_websocket_close(
        task: *mut c_void,
        code: i64,
        reason: *const u8,
        count: usize,
    ) -> bool;
    pub fn foundation_network_event_kind(event: *mut c_void) -> i32;
    pub fn foundation_network_event_response(event: *mut c_void) -> *mut c_void;
    pub fn foundation_network_event_request(event: *mut c_void) -> *mut c_void;
    pub fn foundation_network_event_redirect(event: *mut c_void) -> *mut c_void;
    pub fn foundation_network_event_error(event: *mut c_void) -> *mut c_void;
    pub fn foundation_network_event_text(event: *mut c_void) -> *mut c_char;
    pub fn foundation_network_event_bytes(event: *mut c_void, count: *mut usize) -> *const u8;
    pub fn foundation_network_event_number(event: *mut c_void, index: i32) -> i64;
    pub fn foundation_network_event_flags(event: *mut c_void) -> i32;
    pub fn foundation_network_response_url(response: *mut c_void) -> *mut c_char;
    pub fn foundation_network_response_status(response: *mut c_void) -> i64;
    pub fn foundation_network_response_length(response: *mut c_void) -> i64;
    pub fn foundation_network_response_header_count(response: *mut c_void) -> usize;
    pub fn foundation_network_response_header_name(
        response: *mut c_void,
        index: usize,
    ) -> *mut c_char;
    pub fn foundation_network_response_header_value(
        response: *mut c_void,
        index: usize,
    ) -> *mut c_char;
    pub fn foundation_network_redirect_resolve(redirect: *mut c_void, request: *mut c_void)
        -> bool;
    pub fn foundation_network_redirect_release(redirect: *mut c_void);
    pub fn foundation_network_error_domain(error: *mut c_void) -> *mut c_char;
    pub fn foundation_network_error_code(error: *mut c_void) -> i64;
    pub fn foundation_network_error_description(error: *mut c_void) -> *mut c_char;
    pub fn foundation_network_error_details(error: *mut c_void) -> *mut c_char;
    pub fn foundation_network_error_trust(error: *mut c_void) -> *mut c_char;
    pub fn foundation_network_error_underlying(error: *mut c_void) -> *mut c_void;
    pub fn foundation_network_error_socket_code(error: *mut c_void, code: *mut i32) -> bool;
}
