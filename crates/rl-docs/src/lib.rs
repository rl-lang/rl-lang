//! In-process documentation system for the `rl docs` CLI command.
//!
//! Stores all language reference material as static data compiled into the binary -
//! no external files needed at runtime. Organized into three categories:
//!
//! - **stdlib** - [`StdEntry`] / [`FnEntry`] for every stdlib module and function
//! - **concepts** - [`ConceptEntry`] for language features (types, loops, imports, etc.)
//! - **tutorials** - [`ConceptEntry`] steps for the beginner and advanced tutorials
//!
//! The [`std_to_markdown`], [`concept_to_markdown`], and [`tutorial_to_markdown`]
//! functions render these entries into Markdown for display in the terminal.

pub mod entries;
pub mod entry;
use serde::Serialize;
use serde_json::to_string_pretty;

use crate::entry::{
    ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind, FnEntry, StdEntry,
};

/// Searches all stdlib entries for a function whose signature starts with `fn_name(`.
///
/// `module` narrows the search to a specific stdlib module (e.g. `"io"`, `"math"`).
/// Accepts both bare names (`print`) and qualified names (`std::io::print`).
///
/// Returns `None` if no match is found.
pub fn find_fn_doc(
    module: Option<&str>,
    fn_name: &str,
) -> Option<(&'static StdEntry, &'static FnEntry)> {
    for std_entry in entries::stdlib_entries() {
        if let Some(m) = module
            && std_entry.name != m
        {
            continue;
        }
        for func in std_entry.functions {
            let bare = func.signature.split('(').next().unwrap_or(func.signature);
            if bare == fn_name {
                return Some((std_entry, func));
            }
        }
    }
    None
}

impl ConceptCategory {
    /// A short human-readable label used as a section tag in Markdown output.
    fn label(&self) -> &'static str {
        match self {
            ConceptCategory::Syntax => "syntax",
            ConceptCategory::Types => "types",
            ConceptCategory::ControlFlow => "control flow",
            ConceptCategory::Functions => "functions",
            ConceptCategory::Modules => "modules",
            ConceptCategory::Tooling => "tooling",
            ConceptCategory::ErrorHandling => "error handling",
        }
    }
}

impl DescriptionKind {
    /// The Markdown admonition label used to prefix a description block.
    /// `None` for plain explanations, which render with no prefix.
    fn label(&self) -> Option<&'static str> {
        match self {
            DescriptionKind::Explanation => None,
            DescriptionKind::Syntax => Some("Syntax"),
            DescriptionKind::Pitfall => Some("⚠ Pitfall"),
            DescriptionKind::Note => Some("Note"),
        }
    }
}

/// Renders a `see_also`/`related`/`related_stdlib` list as a Markdown line,
/// or an empty string if the list is empty.
fn render_related(label: &str, names: &[&str], prefix: &str) -> String {
    if names.is_empty() {
        return String::new();
    }
    let links = names
        .iter()
        .map(|n| format!("`{}{}`", prefix, n))
        .collect::<Vec<_>>()
        .join(", ");
    format!("**{}:** {}\n\n", label, links)
}

/// Renders one [`DescriptionEntry`] (title, kind label, prose, examples,
/// and expected output) into the given output buffer.
fn render_description(output: &mut String, description: &DescriptionEntry) {
    if let Some(title) = description.title {
        output.push_str(&format!("#### {}\n\n", title));
    }
    if let Some(kind_label) = description.kind.label() {
        output.push_str(&format!("**{}:** ", kind_label));
    }
    output.push_str(&format!("{}\n\n", description.description));

    for (i, example) in description.examples.iter().enumerate() {
        output.push_str(&format!("```rl\n{}\n```\n\n", example));
        if let Some(expected) = description.expected_output.get(i) {
            output.push_str(&format!("output:\n```text\n{}\n```\n\n", expected));
        }
    }
}

/// Renders all stdlib entries into a Markdown reference document.
///
/// Output is structured as:
/// ```text
/// # rl stdlib reference
/// ## std::<module> (since v0.1.0) [unstable]
/// ### `<signature>`
/// <description>
/// **Returns:** <returns>
/// **Errors:** <errors>
/// ```rl
/// <example>
/// ```
/// output:
/// ```text
/// <expected_output>
/// ```
/// **See also:** `<see_also>`
/// ```
pub fn std_to_markdown(entries: &[&StdEntry]) -> String {
    let mut output = String::from("# rl stdlib reference\n\n");
    for entry in entries {
        output.push_str(&format!("## std::{}", entry.name));
        if let Some(since) = entry.since {
            output.push_str(&format!(" (since {})", since));
        }
        if entry.unstable {
            output.push_str(" `unstable`");
        }
        output.push_str("\n\n");
        output.push_str(&format!("{}\n\n", entry.description));

        for func in entry.functions {
            output.push_str(&format!("### `{}`", func.signature));
            if let Some(since) = func.since {
                output.push_str(&format!(" (since {})", since));
            }
            output.push_str("\n\n");
            output.push_str(&format!("{}\n\n", func.description));
            output.push_str(&format!("**Returns:** {}\n\n", func.returns));
            if let Some(errors) = func.errors {
                output.push_str(&format!("**Errors:** {}\n\n", errors));
            }
            output.push_str(&format!("```rl\n{}\n```\n\n", func.example));
            if let Some(expected) = func.expected_output {
                output.push_str(&format!("output:\n```text\n{}\n```\n\n", expected));
            }
            output.push_str(&render_related("See also", func.see_also, ""));
        }
    }

    output
}

/// shared renderer for any list of ConceptEntry under a custom header
fn entries_to_markdown(header: &str, entries: &[&ConceptEntry]) -> String {
    let mut output = format!("# {}\n\n", header);
    for entry in entries {
        output.push_str(&format!("## {} ({})", entry.name, entry.category.label()));
        if let Some(since) = entry.since {
            output.push_str(&format!(" (since {})", since));
        }
        output.push_str("\n\n");
        output.push_str(&format!("{}\n\n", entry.summary));
        output.push_str(&render_related("Prerequisites", entry.prerequisites, ""));

        for description in entry.descriptions {
            render_description(&mut output, description);
        }

        if !entry.pitfalls.is_empty() {
            output.push_str("**Pitfalls:**\n\n");
            for pitfall in entry.pitfalls {
                output.push_str(&format!("- {}\n", pitfall));
            }
            output.push('\n');
        }

        output.push_str(&render_related("Related concepts", entry.related, ""));
        output.push_str(&render_related(
            "Related stdlib",
            entry.related_stdlib,
            "std::",
        ));
    }
    output
}

/// Renders concept entries into a Markdown language reference document.
pub fn concept_to_markdown(entries: &[&ConceptEntry]) -> String {
    entries_to_markdown("rl concepts reference", entries)
}

/// Renders tutorial entries into a Markdown step-by-step tutorial document.
pub fn tutorial_to_markdown(entries: &[&ConceptEntry]) -> String {
    entries_to_markdown("rl tutorial", entries)
}

#[derive(Serialize)]
pub struct DocsJson<'a> {
    stdlib: &'a [&'static StdEntry],
    concepts: &'a [&'static ConceptEntry],
    tutorial: &'a [&'static ConceptEntry],
}

pub fn docs_to_json(
    stdlib: &[&'static StdEntry],
    concepts: &[&'static ConceptEntry],
    tutorial: &[&'static ConceptEntry],
) -> Result<String, String> {
    let doc = DocsJson {
        stdlib,
        concepts,
        tutorial,
    };

    to_string_pretty(&doc).map_err(|e| e.to_string())
}
