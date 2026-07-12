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
    /// The expected stdout of `example`, for doctest-style checking.
    /// `None` when the example's result is shown inline (e.g. as a trailing
    /// comment) rather than printed.
    pub expected_output: Option<&'static str>,
    /// The return type/value description (e.g. `"float"`, `"Result[int]"`).
    pub returns: &'static str,
    /// When and why this function raises a runtime error. `None` if the
    /// function is infallible.
    pub errors: Option<&'static str>,
    /// Related function names to cross-link (e.g. `["to_hex", "to_oct"]`).
    /// Empty if there's nothing closely related.
    pub see_also: &'static [&'static str],
    /// The version this function was introduced in (e.g. `"v0.1.4"`).
    /// `None` for functions that predate version tracking.
    pub since: Option<&'static str>,
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
    /// The version this module was introduced in (e.g. `"v0.1.4"`).
    /// `None` for modules that predate version tracking.
    pub since: Option<&'static str>,
    /// Whether this module's API is still subject to change and shouldn't
    /// be relied on for backward compatibility yet.
    pub unstable: bool,
}

/// Coarse grouping used to organize the concept index / nav.
#[derive(Serialize)]
pub enum ConceptCategory {
    /// Basic language syntax (comments, literals, general structure).
    Syntax,
    /// Type system concepts (casting, type annotations, `Result[T]`, etc.).
    Types,
    /// Branching and looping constructs (`if`/`elif`/`else`, `for`, `while`).
    ControlFlow,
    /// Function definitions, lambdas, and closures.
    Functions,
    /// Imports and the module system.
    Modules,
    /// CLI, LSP, REPL, and other developer-tooling concepts.
    Tooling,
    /// Errors, `Result[T]`, and failure handling.
    ErrorHandling,
}

/// What role a [`DescriptionEntry`] plays, so the renderer can style it
/// differently (e.g. a pitfall as a warning callout).
#[derive(Serialize)]
pub enum DescriptionKind {
    /// Ordinary prose explaining how or why something works.
    Explanation,
    /// A syntax reference block, usually paired with a minimal example.
    Syntax,
    /// A common mistake or gotcha readers should watch out for.
    Pitfall,
    /// A supplementary aside that isn't essential to the main explanation.
    Note,
}

/// Documentation for a language concept (variables, loops, types, etc.).
#[derive(Serialize)]
pub struct ConceptEntry {
    /// The concept name shown as a section header (e.g. `"arrays"`, `"for loops"`).
    pub name: &'static str,
    /// One-line blurb for index/nav pages, distinct from the full descriptions.
    pub summary: &'static str,
    /// Coarse category for grouping in the concept index.
    pub category: ConceptCategory,
    /// Concept names a reader should understand first. Empty if this is a
    /// foundational concept with no prerequisites.
    pub prerequisites: &'static [&'static str],
    /// One or more description+example pairs explaining this concept.
    pub descriptions: &'static [DescriptionEntry],
    /// Common mistakes specific to this concept. Empty if there are none
    /// worth calling out.
    pub pitfalls: &'static [&'static str],
    /// Related concept names to cross-link (e.g. `["arrays", "for loops"]`).
    pub related: &'static [&'static str],
    /// stdlib module names that implement or relate to this concept
    /// (e.g. `"arrays"` concept relates to the `"array"` module).
    pub related_stdlib: &'static [&'static str],
    /// The version this concept/syntax was introduced or last changed in.
    /// `None` for concepts that predate version tracking.
    pub since: Option<&'static str>,
}

/// A single description with one or more accompanying rl code examples.
#[derive(Serialize)]
pub struct DescriptionEntry {
    /// What role this block plays (explanation, syntax, pitfall, note).
    pub kind: DescriptionKind,
    /// Optional subsection heading, for concepts with multiple distinct parts.
    pub title: Option<&'static str>,
    /// Prose explanation of this aspect of the concept.
    pub description: &'static str,
    /// One or more rl snippets illustrating the description.
    pub examples: &'static [&'static str],
    /// Expected stdout for each entry in `examples`, same length as
    /// `examples` if present, empty if outputs aren't checked.
    pub expected_output: &'static [&'static str],
}
