//! Static data types used to represent all documentation entries.
//!
//! Every entry is a `'static` struct so the entire doc system lives in the
//! binary with zero heap allocation at startup.

use serde::Serialize;

/// A single stdlib function's documentation.
#[derive(Serialize)]
pub struct FnEntry {
    /// The function signature as it appears in rl (e.g. `"arr_push(arr, value)"`).
    pub signature: &'static str,
    /// What the function does, including edge cases and constraints.
    pub description: &'static str,
    /// A runnable rl example showing typical usage.
    pub example: &'static str,
}

/// A stdlib module's documentation, grouping related [`FnEntry`]s together.
#[derive(Serialize)]
pub struct StdEntry {
    /// The module name as used in imports (e.g. `"io"`, `"math::consts"`).
    pub name: &'static str,
    /// A short description of the module's purpose.
    pub description: &'static str,
    /// All documented functions in this module.
    pub functions: &'static [&'static FnEntry],
}

/// Documentation for a language concept (variables, loops, types, etc.).
///
/// Each concept has one or more [`DescriptionEntry`]s, each of which
/// pairs a prose explanation with one or more runnable rl examples.
#[derive(Serialize)]
pub struct ConceptEntry {
    /// The concept name shown as a section header (e.g. `"arrays"`, `"for loops"`).
    pub name: &'static str,
    /// One or more description+example pairs explaining this concept.
    pub descriptions: &'static [DescriptionEntry],
}

/// A single description with one or more accompanying rl code examples.
#[derive(Serialize)]
pub struct DescriptionEntry {
    /// Prose explanation of this aspect of the concept.
    pub description: &'static str,
    /// One or more rl snippets illustrating the description.
    pub examples: &'static [&'static str],
}
