use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static TOOLING: ConceptEntry = ConceptEntry {
    name: "tooling",
    descriptions: &[
        DescriptionEntry {
            description: "rl run <file> runs a .rl source file",
            examples: &["rl run main.rl"],
        },
        DescriptionEntry {
            description: "rl check <file> type-checks a file and reports errors without running it",
            examples: &["rl check main.rl"],
        },
        DescriptionEntry {
            description: "rl repl starts an interactive session",
            examples: &["rl repl"],
        },
        DescriptionEntry {
            description: "rl dev runs the project in the current directory using rl.toml",
            examples: &["rl dev"],
        },
        DescriptionEntry {
            description: "rl new <name> scaffolds a new project directory with rl.toml and main.rl",
            examples: &["rl new my-project"],
        },
        DescriptionEntry {
            description: "rl docs [topic] shows language reference and stdlib documentation",
            examples: &["rl docs", "rl docs byte"],
        },
    ],
};
