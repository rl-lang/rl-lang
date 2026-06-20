pub mod entries;
pub mod entry;
use crate::docs::entry::{ConceptEntry, FnEntry, StdEntry};

// finds the FnEntry whose signature starts with `fn_name(`
// can accept std::io::print() and print()
pub fn find_fn_doc(
    module: Option<&str>,
    fn_name: &str,
) -> Option<(&'static StdEntry, &'static FnEntry)> {
    for std_entry in entries::stdlib_entries() {
        if let Some(m) = module {
            if std_entry.name != m {
                continue;
            }
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
