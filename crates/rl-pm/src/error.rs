use std::fmt;

#[derive(Debug)]
pub enum PmError {
    Io(std::io::Error),
    Http(String),
    Sha256Mismatch { expected: String, actual: String },
    ExtractionFailed(String),
    MissingLib(String),
    TomlParse(String),
    NotFound(String),
    NoProject,
}

impl fmt::Display for PmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PmError::Io(e) => write!(f, "io error: {}", e),
            PmError::Http(msg) => write!(f, "http error: {}", msg),
            PmError::Sha256Mismatch { expected, actual } => {
                write!(f, "sha256 mismatch:\n  expected: {}\n  actual:   {}", expected, actual)
            }
            PmError::ExtractionFailed(msg) => write!(f, "extraction failed: {}", msg),
            PmError::MissingLib(name) => {
                write!(f, "dependency '{}' missing lib.rl at package root", name)
            }
            PmError::TomlParse(msg) => write!(f, "toml error: {}", msg),
            PmError::NotFound(msg) => write!(f, "not found: {}", msg),
            PmError::NoProject => write!(f, "no rl.toml found in current directory"),
        }
    }
}

impl std::error::Error for PmError {}

impl From<std::io::Error> for PmError {
    fn from(e: std::io::Error) -> Self {
        PmError::Io(e)
    }
}
