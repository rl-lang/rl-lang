//! `rl_lang` - the core library crate.
//!
//! Re-exports all subsystems so both the CLI binary (`main.rs`) and external
//! consumers (LSP, tests, benchmarks) can reach them through one crate.
//!
//! Feature flags gate optional subsystems:
//!
//! | Feature | Enables |
//! |---|---|
//! | `eval` | [`checker`], [`interpreter`] |
//! | `repl_tui` | [`repl`] |
//! | `lsp` | [`lsp`] |
//! | `debug` | verbose `log::` output throughout the pipeline |
pub mod ast;
#[cfg(feature = "eval")]
pub mod checker;
pub mod docs;
#[cfg(feature = "eval")]
pub mod interpreter;
pub mod lexer;
pub mod logic_loops;
#[cfg(feature = "lsp")]
pub mod lsp;
pub mod parser;
#[cfg(feature = "repl_tui")]
pub mod repl;
pub mod resolver;
pub mod tooling;
pub mod utils;
#[cfg(feature = "vm")]
pub mod vm;
