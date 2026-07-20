//! Shared utilities used across the rl pipeline.
//!
//! - [`errors`] - [`Error`] type and [`Reason`] categories
//! - [`line_index`] - [`line_index::LineIndex`] for source-free `file:line:col` diagnostics
//! - [`source`] - [`SourceFile`] carrying source text through the pipeline
//! - [`span`] - [`Span`] for pointing errors at exact source locations
//! - [`suggest`] - fuzzy name matching for "did you mean?" hints
pub mod errors;
pub mod line_index;
pub mod source;
pub mod span;
pub mod suggest;
