//! Small shared utilities for the REPL.

use crate::lines_types::OutputLine;
use rl_utils::errors::Error;

/// Appends an [`OutputLine::Error`] derived from `e` to `output`.
pub fn push_error(output: &mut Vec<OutputLine>, e: &Error) {
    output.push(OutputLine::Error(format!("error: {}", e.message())));
}

/// Converts a char-indexed position in `s` to its byte offset.
///
/// Returns `s.len()` if `char_idx` is past the end, making it safe to use
/// directly with [`String::insert`] and [`String::remove`].
pub fn char_to_byte(s: &str, char_idx: usize) -> usize {
    s.char_indices()
        .nth(char_idx)
        .map(|(b, _)| b)
        .unwrap_or(s.len())
}
