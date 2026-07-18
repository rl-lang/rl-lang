//! Tooling support for rl projects.
//!
//! - [`new`] - project scaffolding (`rl new <name>`)
//! - [`dev`] - project manifest parsing (`rl.toml`)
pub mod dev;
pub mod format;
pub mod generate_docs;
pub mod new;
pub mod package;
pub mod workflows;
