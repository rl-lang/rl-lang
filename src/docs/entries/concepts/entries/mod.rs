use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static ENTRY_POINTS: ConceptEntry = ConceptEntry {
    name: "entry points",
    descriptions: &[
        DescriptionEntry {
            description: "source files work as scripts when no entry function is present",
            examples: &[],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "if a file declares `fn main()`, `rl run` registers declarations and runs `main()` instead of evaluating top-level expressions",
            examples: &["fn main() {\n    std::io::println(\"hello\")\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "a different zero-argument function can be selected as the entry point with `!#[entry]`",
            examples: &["!#[entry]\nfn start() {\n    std::io::println(\"hello\")\n}"],
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
