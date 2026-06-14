pub mod entries;
mod entry;
use crate::docs::entry::{ConceptEntry, StdEntry};

// helper functions that transform docs into readable mark down
pub fn std_to_markdown(entries: &[&StdEntry]) -> String {
    // markdown header
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

// returns concept with the example provided
pub fn concept_to_markdown(entries: &[&ConceptEntry]) -> String {
    let mut output = String::from("# rl concepts reference\n\n");
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
