use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FoundationError {
    InvalidArgument(String),
}

impl fmt::Display for FoundationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
        }
    }
}

impl std::error::Error for FoundationError {}
