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
use crate::docs::entry::{ConceptEntry, FnEntry, StdEntry};

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

/// Renders all stdlib entries into a Markdown reference document.
///
/// Output is structured as:
/// ```text
/// # rl stdlib reference
/// ## std::<module>
/// ### `<signature>`
/// <description>
/// ```rl
/// <example>
/// ```
/// ```
pub fn std_to_markdown(entries: &[&StdEntry]) -> String {
    let mut output = String::from("# rl stdlib reference\n\n");
    for entry in entries {
        output.push_str(&format!("## std::{}\n\n", entry.name));
        output.push_str(&format!("{}\n\n", entry.description));
        for func in entry.functions {
            output.push_str(&format!("### `{}`\n\n", func.signature));
            output.push_str(&format!("{}\n\n", func.description));
            output.push_str(&format!("```rl\n{}\n```\n\n", func.example));
        }
    }

    output
}

/// shared renderer for any list of ConceptEntry under a custom header
fn entries_to_markdown(header: &str, entries: &[&ConceptEntry]) -> String {
    let mut output = format!("# {}\n\n", header);
    for entry in entries {
        output.push_str(&format!("## {}\n\n", entry.name));
        for description in entry.descriptions {
            output.push_str(&format!("{}\n\n", description.description));
            for example in description.examples {
                output.push_str(&format!("```rl\n{}\n```\n\n", example));
            }
        }
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
