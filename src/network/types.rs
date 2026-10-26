use core::ffi::c_void;
use std::fmt;
use std::time::Duration;

use super::{ffi, owned_string, RedirectDecision};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProxyPolicy {
    #[default]
    SystemWithLocalBypass,
    Direct,
}

#[derive(Clone, Debug)]
pub struct SessionConfig {
    pub idle_timeout: Duration,
    pub resource_timeout: Duration,
    pub maximum_connections_per_host: u16,
    pub proxy_policy: ProxyPolicy,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(30),
            resource_timeout: Duration::from_secs(60),
            maximum_connections_per_host: 16,
            proxy_policy: ProxyPolicy::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
    pub idle_timeout: Option<Duration>,
    pub deadline: Option<Duration>,
}

impl Request {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method: "GET".into(),
            headers: Vec::new(),
            body: None,
            idle_timeout: None,
            deadline: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    pub url: String,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub expected_content_length: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct WebSocketConfig {
    pub protocols: Vec<String>,
    pub maximum_message_size: usize,
}

impl Default for WebSocketConfig {
    fn default() -> Self {
        Self {
            protocols: Vec::new(),
            maximum_message_size: 64 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Metrics {
    pub protocol: String,
    pub tls_protocol: Option<u16>,
    pub reused_connection: bool,
    pub proxy_connection: bool,
    pub local_bypass: bool,
}

#[derive(Debug)]
pub enum Event {
    Response(Response),
    Data(Vec<u8>),
    UploadProgress {
        bytes_sent: u64,
        total_bytes: Option<u64>,
    },
    Redirect {
        response: Response,
        proposed_request: Request,
        decision: RedirectDecision,
    },
    Complete(Result<(), NetworkError>),
    WebSocketOpen {
        protocol: Option<String>,
    },
    Message(Result<Message, NetworkError>),
    WebSocketClose {
        code: u16,
        reason: Vec<u8>,
    },
    Sent {
        operation: i64,
        result: Result<(), NetworkError>,
    },
    Pong {
        operation: i64,
        result: Result<(), NetworkError>,
    },
    Metrics(Metrics),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCategory {
    InvalidArgument,
    Cancelled,
    Timeout,
    Dns,
    ConnectionRefused,
    ConnectionDropped,
    CannotConnect,
    Certificate,
    SecureConnection,
    TransportSecurityPolicy,
    InvalidResponse,
    MessageTooLarge,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkError {
    pub domain: String,
    pub code: i64,
    pub description: String,
    pub details: String,
    pub trust_details: Option<String>,
    pub socket_error: Option<i32>,
    pub underlying: Option<Box<Self>>,
}

impl NetworkError {
    #[must_use]
    pub fn category(&self) -> ErrorCategory {
        if self.domain == "FoundationNetwork" {
            return ErrorCategory::InvalidArgument;
        }
        if self.domain != "NSURLErrorDomain" {
            let code = if self.domain == "NSPOSIXErrorDomain" {
                Some(self.code)
            } else {
                self.socket_error.map(i64::from)
            };
            return match code {
                Some(40) => ErrorCategory::MessageTooLarge,
                Some(61) => ErrorCategory::ConnectionRefused,
                Some(53 | 54) => ErrorCategory::ConnectionDropped,
                Some(60) => ErrorCategory::Timeout,
                _ => ErrorCategory::Other,
            };
        }
        match self.code {
            -999 => ErrorCategory::Cancelled,
            -1001 => ErrorCategory::Timeout,
            -1003 | -1006 => ErrorCategory::Dns,
            -1004 => {
                if self
                    .underlying
                    .iter()
                    .any(|error| error.category() == ErrorCategory::ConnectionRefused)
                    || self.socket_error == Some(61)
                {
                    ErrorCategory::ConnectionRefused
                } else {
                    ErrorCategory::CannotConnect
                }
            }
            -1005 => ErrorCategory::ConnectionDropped,
            -1011 | -1017 => ErrorCategory::InvalidResponse,
            -1022 => ErrorCategory::TransportSecurityPolicy,
            -1204..=-1201 => ErrorCategory::Certificate,
            -1200 => ErrorCategory::SecureConnection,
            _ => self
                .underlying
                .as_ref()
                .map_or(ErrorCategory::Other, |error| error.category()),
        }
    }

    pub(super) fn argument(description: impl Into<String>) -> Self {
        Self {
            domain: "FoundationNetwork".into(),
            code: 1,
            description: description.into(),
            details: String::new(),
            trust_details: None,
            socket_error: None,
            underlying: None,
        }
    }

    pub(super) unsafe fn from_native(handle: *mut c_void, depth: u8) -> Self {
        let trust = unsafe { owned_string(ffi::foundation_network_error_trust(handle)) };
        let underlying = unsafe { ffi::foundation_network_error_underlying(handle) };
        let mut socket_code = 0;
        let has_socket_code =
            unsafe { ffi::foundation_network_error_socket_code(handle, &raw mut socket_code) };
        Self {
            domain: unsafe { owned_string(ffi::foundation_network_error_domain(handle)) },
            code: unsafe { ffi::foundation_network_error_code(handle) },
            description: unsafe { owned_string(ffi::foundation_network_error_description(handle)) },
            details: unsafe { owned_string(ffi::foundation_network_error_details(handle)) },
            trust_details: (!trust.is_empty()).then_some(trust),
            socket_error: has_socket_code.then_some(socket_code),
            underlying: if underlying.is_null() || depth >= 8 {
                None
            } else {
                Some(Box::new(unsafe {
                    Self::from_native(underlying, depth + 1)
                }))
            },
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} ({}): {}",
            self.domain, self.code, self.description
        )
    }
}

impl std::error::Error for NetworkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.underlying
            .as_ref()
            .map(|error| error.as_ref() as &(dyn std::error::Error + 'static))
    }
}
