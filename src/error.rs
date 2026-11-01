use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FoundationError {
    InvalidArgument(String),
    Native {
        domain: String,
        code: i64,
        message: String,
    },
}

impl fmt::Display for FoundationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::Native {
                domain,
                code,
                message,
            } => write!(f, "{domain} ({code}): {message}"),
        }
    }
}

impl std::error::Error for FoundationError {}
