use std::fmt;

#[derive(Debug)]
pub enum RlmError {
    Io(std::io::Error),
    Http(String),
    ChecksumMismatch { expected: String, actual: String },
    ExtractionFailed(String),
    VersionNotFound(String),
    NoVariantSelected,
    SelfUpdateFailed(String),
}

impl fmt::Display for RlmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RlmError::Io(e) => write!(f, "IO error: {}", e),
            RlmError::Http(msg) => write!(f, "HTTP error: {}", msg),
            RlmError::ChecksumMismatch { expected, actual } => {
                write!(f, "checksum mismatch:\n  expected: {}\n  actual:   {}", expected, actual)
            }
            RlmError::ExtractionFailed(msg) => write!(f, "extraction failed: {}", msg),
            RlmError::VersionNotFound(v) => write!(f, "version '{}' not found on GitHub", v),
            RlmError::NoVariantSelected => write!(f, "no variant selected"),
            RlmError::SelfUpdateFailed(msg) => write!(f, "self-update failed: {}", msg),
        }
    }
}

impl std::error::Error for RlmError {}

impl From<std::io::Error> for RlmError {
    fn from(e: std::io::Error) -> Self {
        RlmError::Io(e)
    }
}

pub type Result<T> = std::result::Result<T, RlmError>;
