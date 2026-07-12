//! The rl tree-walking interpreter.
//!
//! # Pipeline
//!
//! ```text
//! source text
//!   └─ Tokenizer::lex()       → Vec<Token>
//!   └─ Parser::parse()        → Vec<Statement>
//!   └─ Resolver::resolve()    → Vec<Statement>  (names → depth/slot)
//!   └─ Evaluator::evaluate_program() → ()
//! ```
//!
//! # Modules
//!
//! - [`evaluator`] - the core [`Evaluator`] struct and expression evaluation
//! - [`values`] - the [`Value`] enum representing all runtime values
//! - [`native`] - [`Module`], [`NativeFn`], and the trait system for binding Rust functions
//! - [`scopes`] - environment stack: push/pop/insert/assign/get
//! - [`stdlib`] - all built-in stdlib modules
//! - [`utils`] - binary/unary operator dispatch and statement evaluation
pub mod evaluator;
mod evaluator_types;
pub mod native;
mod scopes;
pub mod stdlib;
mod utils;
pub mod values;
