mod ffi;
mod types;

use core::ffi::{c_char, c_void};
use core::ptr::NonNull;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::sync::Arc;
use std::time::Duration;

use doom_fish_utils::callback_context::CallbackContext;

pub use types::{
    ErrorCategory, Event, Message, Metrics, NetworkError, ProxyPolicy, Request, Response,
    SessionConfig, WebSocketConfig,
};

type EventCallback = Box<dyn Fn(Event) + Send + Sync + 'static>;

#[derive(Debug)]
struct QueueInner(NonNull<c_void>);

unsafe impl Send for QueueInner {}
unsafe impl Sync for QueueInner {}

#[derive(Clone, Debug)]
pub struct DelegateQueue(Arc<QueueInner>);

#[derive(Debug)]
struct SessionInner {
    handle: NonNull<c_void>,
    _queue: DelegateQueue,
}

unsafe impl Send for SessionInner {}
unsafe impl Sync for SessionInner {}

#[derive(Clone, Debug)]
pub struct Session(Arc<SessionInner>);

#[derive(Debug)]
struct TaskInner {
    handle: NonNull<c_void>,
    context: CallbackContext<EventCallback>,
    _session: Arc<SessionInner>,
}

unsafe impl Send for TaskInner {}
unsafe impl Sync for TaskInner {}

#[derive(Clone, Debug)]
pub struct DataTask(Arc<TaskInner>);

#[derive(Clone, Debug)]
pub struct WebSocketTask(Arc<TaskInner>);

#[derive(Debug)]
pub struct UploadWriter {
    handle: NonNull<c_void>,
    length: u64,
    written: u64,
    finished: bool,
}

unsafe impl Send for UploadWriter {}

#[derive(Debug)]
pub struct RedirectDecision {
    handle: NonNull<c_void>,
}

unsafe impl Send for RedirectDecision {}
unsafe impl Sync for RedirectDecision {}

struct NativeRequest(NonNull<c_void>);

fn c_string(value: &str) -> Result<CString, NetworkError> {
    CString::new(value).map_err(|_| NetworkError::argument("A string contains a NUL byte."))
}

fn timeout(duration: Duration) -> Result<f64, NetworkError> {
    let seconds = duration.as_secs_f64();
    if duration.is_zero() || seconds > f64::from(u32::MAX) {
        return Err(NetworkError::argument(
            "A timeout must be positive and must not exceed 4294967295 seconds.",
        ));
    }
    Ok(seconds)
}

fn optional_timeout(duration: Option<Duration>) -> Result<f64, NetworkError> {
    duration.map_or(Ok(0.0), timeout)
}

fn is_http_name(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&byte))
}

unsafe fn owned_string(pointer: *mut c_char) -> String {
    if pointer.is_null() {
        return String::new();
    }
    let value = unsafe { CStr::from_ptr(pointer) }
        .to_string_lossy()
        .into_owned();
    unsafe { crate::ffi::foundation_string_free(pointer) };
    value
}

impl DelegateQueue {
    pub fn new(name: &str) -> Result<Self, NetworkError> {
        let name = c_string(name)?;
        let handle = unsafe { ffi::foundation_network_queue_new(name.as_ptr()) };
        Ok(Self(Arc::new(QueueInner(
            NonNull::new(handle).ok_or_else(|| {
                NetworkError::argument("The delegate queue could not be created.")
            })?,
        ))))
    }

    #[must_use]
    pub fn is_current(&self) -> bool {
        unsafe { ffi::foundation_network_queue_is_current(self.0 .0.as_ptr()) }
    }
}

impl Drop for QueueInner {
    fn drop(&mut self) {
        unsafe { ffi::foundation_network_queue_release(self.0.as_ptr()) };
    }
}

impl Session {
    pub fn new(queue: &DelegateQueue, config: &SessionConfig) -> Result<Self, NetworkError> {
        if config.maximum_connections_per_host == 0 {
            return Err(NetworkError::argument(
                "The connection limit must be greater than zero.",
            ));
        }
        let idle = timeout(config.idle_timeout)?;
        let resource = timeout(config.resource_timeout)?;
        let handle = unsafe {
            ffi::foundation_network_session_new(
                queue.0 .0.as_ptr(),
                idle,
                resource,
                i32::from(config.maximum_connections_per_host),
                match config.proxy_policy {
                    ProxyPolicy::SystemWithLocalBypass => 0,
                    ProxyPolicy::Direct => 1,
                },
            )
        };
        Ok(Self(Arc::new(SessionInner {
            handle: NonNull::new(handle)
                .ok_or_else(|| NetworkError::argument("The session could not be created."))?,
            _queue: queue.clone(),
        })))
    }

    pub fn data_task(
        &self,
        request: &Request,
        callback: impl Fn(Event) + Send + Sync + 'static,
    ) -> Result<DataTask, NetworkError> {
        if !request.url.split_once(':').is_some_and(|(scheme, _)| {
            scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https")
        }) {
            return Err(NetworkError::argument(
                "An HTTP task needs an HTTP or HTTPS URL.",
            ));
        }
        Ok(DataTask(TaskInner::new(self, request, 0, 0, callback)?))
    }

    pub fn web_socket_task(
        &self,
        request: &Request,
        config: &WebSocketConfig,
        callback: impl Fn(Event) + Send + Sync + 'static,
    ) -> Result<WebSocketTask, NetworkError> {
        if !request.url.split_once(':').is_some_and(|(scheme, _)| {
            scheme.eq_ignore_ascii_case("ws") || scheme.eq_ignore_ascii_case("wss")
        }) {
            return Err(NetworkError::argument(
                "A WebSocket task needs a WS or WSS URL.",
            ));
        }
        if config.maximum_message_size == 0 || config.maximum_message_size > isize::MAX as usize {
            return Err(NetworkError::argument("The message limit is out of range."));
        }
        let mut request = request.clone();
        if request.body.is_some() {
            return Err(NetworkError::argument(
                "A WebSocket request cannot have a body.",
            ));
        }
        if request
            .headers
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("Sec-WebSocket-Protocol"))
        {
            return Err(NetworkError::argument(
                "Use WebSocketConfig to set the subprotocols.",
            ));
        }
        for (index, protocol) in config.protocols.iter().enumerate() {
            if !is_http_name(protocol) || config.protocols[..index].contains(protocol) {
                return Err(NetworkError::argument(
                    "A WebSocket subprotocol is invalid.",
                ));
            }
        }
        if !config.protocols.is_empty() {
            request
                .headers
                .push(("Sec-WebSocket-Protocol".into(), config.protocols.join(", ")));
        }
        Ok(WebSocketTask(TaskInner::new(
            self,
            &request,
            1,
            config.maximum_message_size,
            callback,
        )?))
    }

    pub fn invalidate_and_cancel(&self) {
        unsafe { ffi::foundation_network_session_cancel(self.0.handle.as_ptr()) };
    }
}

impl Drop for SessionInner {
    fn drop(&mut self) {
        unsafe { ffi::foundation_network_session_release(self.handle.as_ptr()) };
    }
}

impl NativeRequest {
    fn new(request: &Request) -> Result<Self, NetworkError> {
        if !is_http_name(&request.method)
            || request.url.chars().any(char::is_control)
            || request.headers.len() > 100
        {
            return Err(NetworkError::argument("The request is invalid."));
        }
        let url = c_string(&request.url)?;
        let method = c_string(&request.method)?;
        let idle = optional_timeout(request.idle_timeout)?;
        let pointer =
            unsafe { ffi::foundation_network_request_new(url.as_ptr(), method.as_ptr(), idle) };
        let native = Self(
            NonNull::new(pointer).ok_or_else(|| NetworkError::argument("The URL is invalid."))?,
        );
        for (name, value) in &request.headers {
            if !is_http_name(name)
                || !value
                    .bytes()
                    .all(|byte| byte == b'\t' || (32..=126).contains(&byte))
                || name.len().saturating_add(value.len()) > 100 * 1024
            {
                return Err(NetworkError::argument("A request header is invalid."));
            }
            let name = c_string(name)?;
            let value = c_string(value)?;
            unsafe {
                ffi::foundation_network_request_add_header(
                    native.0.as_ptr(),
                    name.as_ptr(),
                    value.as_ptr(),
                );
            }
        }
        if let Some(body) = &request.body {
            unsafe {
                ffi::foundation_network_request_set_body(
                    native.0.as_ptr(),
                    body.as_ptr(),
                    body.len(),
                );
            }
        }
        Ok(native)
    }
}

impl Drop for NativeRequest {
    fn drop(&mut self) {
        unsafe { ffi::foundation_network_request_release(self.0.as_ptr()) };
    }
}

impl TaskInner {
    fn new(
        session: &Session,
        request: &Request,
        kind: i32,
        message_limit: usize,
        callback: impl Fn(Event) + Send + Sync + 'static,
    ) -> Result<Arc<Self>, NetworkError> {
        let native = NativeRequest::new(request)?;
        let deadline = optional_timeout(request.deadline)?;
        let callback: EventCallback = Box::new(callback);
        let context = CallbackContext::new(callback);
        let handle = unsafe {
            ffi::foundation_network_task_new(
                session.0.handle.as_ptr(),
                native.0.as_ptr(),
                kind,
                deadline,
                message_limit,
                context.as_ptr(),
                invoke_event,
                CallbackContext::<EventCallback>::RETAIN,
                CallbackContext::<EventCallback>::RELEASE,
            )
        };
        Ok(Arc::new(Self {
            handle: NonNull::new(handle)
                .ok_or_else(|| NetworkError::argument("The session is invalid."))?,
            context,
            _session: Arc::clone(&session.0),
        }))
    }
}

impl Drop for TaskInner {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { ffi::foundation_network_task_release(self.handle.as_ptr()) };
    }
}

impl DataTask {
    pub fn resume(&self) {
        unsafe { ffi::foundation_network_task_resume(self.0.handle.as_ptr()) };
    }

    pub fn suspend(&self) {
        unsafe { ffi::foundation_network_task_suspend(self.0.handle.as_ptr()) };
    }

    pub fn cancel(&self) {
        unsafe { ffi::foundation_network_task_cancel(self.0.handle.as_ptr()) };
    }

    pub fn upload_body(&self, length: u64) -> Result<UploadWriter, NetworkError> {
        let native_length = i64::try_from(length)
            .map_err(|_| NetworkError::argument("The upload length is out of range."))?;
        let handle =
            unsafe { ffi::foundation_network_task_upload(self.0.handle.as_ptr(), native_length) };
        Ok(UploadWriter {
            handle: NonNull::new(handle).ok_or_else(|| {
                NetworkError::argument("Set the upload body before the task starts.")
            })?,
            length,
            written: 0,
            finished: false,
        })
    }
}

impl Write for UploadWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.finished || bytes.len() as u64 > self.length - self.written {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "The upload exceeds its declared length or is complete.",
            ));
        }
        if bytes.is_empty() {
            return Ok(0);
        }
        let count = bytes.len().min(64 * 1024);
        let result = unsafe {
            ffi::foundation_network_upload_write(self.handle.as_ptr(), bytes.as_ptr(), count)
        };
        let count = usize::try_from(result)
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "The upload was cancelled."))?;
        self.written += count as u64;
        Ok(count)
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.finished {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "The upload is complete.",
            ));
        }
        Ok(())
    }
}

impl UploadWriter {
    pub fn finish(mut self) -> io::Result<()> {
        if self.written != self.length {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "The upload is shorter than its declared length.",
            ));
        }
        self.finished = unsafe { ffi::foundation_network_upload_finish(self.handle.as_ptr()) };
        if !self.finished {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "The upload was cancelled.",
            ));
        }
        Ok(())
    }
}

impl Drop for UploadWriter {
    fn drop(&mut self) {
        unsafe { ffi::foundation_network_upload_release(self.handle.as_ptr()) };
    }
}

impl WebSocketTask {
    pub fn resume(&self) {
        unsafe { ffi::foundation_network_task_resume(self.0.handle.as_ptr()) };
    }

    pub fn cancel(&self) {
        unsafe { ffi::foundation_network_task_cancel(self.0.handle.as_ptr()) };
    }

    pub fn receive(&self) -> Result<(), NetworkError> {
        if !unsafe { ffi::foundation_network_websocket_receive(self.0.handle.as_ptr()) } {
            return Err(NetworkError::argument(
                "A receive is pending or the WebSocket is not open.",
            ));
        }
        Ok(())
    }

    pub fn send(&self, operation: i64, message: &Message) -> Result<(), NetworkError> {
        let (text, bytes) = match message {
            Message::Text(value) => (true, value.as_bytes()),
            Message::Binary(value) => (false, value.as_slice()),
        };
        if !unsafe {
            ffi::foundation_network_websocket_send(
                self.0.handle.as_ptr(),
                operation,
                text,
                bytes.as_ptr(),
                bytes.len(),
            )
        } {
            return Err(NetworkError::argument(
                "A send is pending or the WebSocket is not open.",
            ));
        }
        Ok(())
    }

    pub fn ping(&self, operation: i64) -> Result<(), NetworkError> {
        if !unsafe { ffi::foundation_network_websocket_ping(self.0.handle.as_ptr(), operation) } {
            return Err(NetworkError::argument(
                "A ping is pending or the WebSocket is not open.",
            ));
        }
        Ok(())
    }

    pub fn close(&self, code: u16, reason: &str) -> Result<(), NetworkError> {
        if !matches!(code, 1000..=1003 | 1005 | 1007..=1014 | 3000..=4999) {
            return Err(NetworkError::argument(
                "The WebSocket close code is invalid.",
            ));
        }
        let mut count = if code == 1005 {
            0
        } else {
            reason.len().min(123)
        };
        while !reason.is_char_boundary(count) {
            count -= 1;
        }
        if !unsafe {
            ffi::foundation_network_websocket_close(
                self.0.handle.as_ptr(),
                i64::from(code),
                reason.as_ptr(),
                count,
            )
        } {
            return Err(NetworkError::argument("The WebSocket is not open."));
        }
        Ok(())
    }
}

impl RedirectDecision {
    pub fn follow(self, request: &Request) -> Result<(), NetworkError> {
        let request = NativeRequest::new(request)?;
        if !unsafe {
            ffi::foundation_network_redirect_resolve(self.handle.as_ptr(), request.0.as_ptr())
        } {
            return Err(NetworkError::argument("The redirect is no longer pending."));
        }
        Ok(())
    }

    pub fn refuse(self) {
        unsafe {
            ffi::foundation_network_redirect_resolve(self.handle.as_ptr(), core::ptr::null_mut());
        }
    }
}

impl Drop for RedirectDecision {
    fn drop(&mut self) {
        unsafe { ffi::foundation_network_redirect_release(self.handle.as_ptr()) };
    }
}

unsafe fn response_from_native(handle: *mut c_void) -> Response {
    let count = unsafe { ffi::foundation_network_response_header_count(handle) };
    let headers = (0..count)
        .map(|index| unsafe {
            (
                owned_string(ffi::foundation_network_response_header_name(handle, index)),
                owned_string(ffi::foundation_network_response_header_value(handle, index)),
            )
        })
        .collect();
    Response {
        url: unsafe { owned_string(ffi::foundation_network_response_url(handle)) },
        status: u16::try_from(unsafe { ffi::foundation_network_response_status(handle) })
            .unwrap_or(0),
        expected_content_length: u64::try_from(unsafe {
            ffi::foundation_network_response_length(handle)
        })
        .ok(),
        headers,
    }
}

unsafe fn request_from_native(handle: *mut c_void) -> Request {
    let mut request =
        Request::new(unsafe { owned_string(ffi::foundation_network_request_url(handle)) });
    request.method = unsafe { owned_string(ffi::foundation_network_request_method(handle)) };
    request.idle_timeout =
        Duration::try_from_secs_f64(unsafe { ffi::foundation_network_request_timeout(handle) })
            .ok()
            .filter(|duration| !duration.is_zero());
    let count = unsafe { ffi::foundation_network_request_header_count(handle) };
    request.headers = (0..count)
        .map(|index| unsafe {
            (
                owned_string(ffi::foundation_network_request_header_name(handle, index)),
                owned_string(ffi::foundation_network_request_header_value(handle, index)),
            )
        })
        .collect();
    request
}

unsafe fn event_bytes(event: *mut c_void) -> Vec<u8> {
    let mut count = 0;
    let bytes = unsafe { ffi::foundation_network_event_bytes(event, &raw mut count) };
    if count == 0 || bytes.is_null() {
        return Vec::new();
    }
    unsafe { core::slice::from_raw_parts(bytes, count) }.to_vec()
}

unsafe fn event_error(event: *mut c_void) -> Result<(), NetworkError> {
    let error = unsafe { ffi::foundation_network_event_error(event) };
    if error.is_null() {
        Ok(())
    } else {
        Err(unsafe { NetworkError::from_native(error, 0) })
    }
}

unsafe extern "C" fn invoke_event(context: *mut c_void, native: *mut c_void) {
    let _ = unsafe {
        CallbackContext::<EventCallback>::with(context, "foundation::network", |callback| {
            let number = |index| ffi::foundation_network_event_number(native, index);
            let flags = ffi::foundation_network_event_flags(native);
            let event = match ffi::foundation_network_event_kind(native) {
                1 => Event::Response(response_from_native(
                    ffi::foundation_network_event_response(native),
                )),
                2 => Event::Data(event_bytes(native)),
                3 => Event::UploadProgress {
                    bytes_sent: u64::try_from(number(0)).unwrap_or(0),
                    total_bytes: u64::try_from(number(1)).ok(),
                },
                4 => Event::Redirect {
                    response: response_from_native(ffi::foundation_network_event_response(native)),
                    proposed_request: request_from_native(ffi::foundation_network_event_request(
                        native,
                    )),
                    decision: RedirectDecision {
                        handle: NonNull::new(ffi::foundation_network_event_redirect(native))
                            .expect("a redirect event has a decision"),
                    },
                },
                5 => Event::Complete(event_error(native)),
                6 => {
                    let protocol = owned_string(ffi::foundation_network_event_text(native));
                    Event::WebSocketOpen {
                        protocol: (!protocol.is_empty()).then_some(protocol),
                    }
                }
                7 => Event::Message(event_error(native).and_then(|()| {
                    let bytes = event_bytes(native);
                    if flags & 1 == 0 {
                        Ok(Message::Binary(bytes))
                    } else {
                        String::from_utf8(bytes)
                            .map(Message::Text)
                            .map_err(|_| NetworkError::argument("The text is not UTF-8."))
                    }
                })),
                8 => Event::WebSocketClose {
                    code: u16::try_from(number(0)).unwrap_or(1005),
                    reason: event_bytes(native),
                },
                9 => Event::Sent {
                    operation: number(0),
                    result: event_error(native),
                },
                10 => Event::Pong {
                    operation: number(0),
                    result: event_error(native),
                },
                11 => Event::Metrics(Metrics {
                    protocol: owned_string(ffi::foundation_network_event_text(native)),
                    tls_protocol: u16::try_from(number(0)).ok().filter(|value| *value != 0),
                    reused_connection: flags & 1 != 0,
                    proxy_connection: flags & 2 != 0,
                    local_bypass: flags & 4 != 0,
                }),
                _ => return,
            };
            callback(event);
        })
    };
}
