use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static TOOLING: ConceptEntry = ConceptEntry {
    name: "tooling",
    descriptions: &[
        DescriptionEntry {
            description: "rl run <file> runs a .rl source file",
            examples: &["rl run main.rl"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "rl check <file> type-checks a file and reports errors without running it",
            examples: &["rl check main.rl"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "rl repl starts an interactive session",
            examples: &["rl repl"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "rl dev runs the project in the current directory using rl.toml",
            examples: &["rl dev"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "rl new <name> scaffolds a new project directory with rl.toml and main.rl",
            examples: &["rl new my-project"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "rl docs [topic] shows language reference and stdlib documentation",
            examples: &["rl docs", "rl docs byte"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
    ],
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
