pub mod error;
pub mod install;
pub mod platform;
pub mod variants;
pub mod version;
#[cfg(feature = "tui")]
pub mod tui;

pub use error::{RlmError, Result};
pub use platform::{Arch, Platform};
pub use variants::Variant;
pub use version::Version;
