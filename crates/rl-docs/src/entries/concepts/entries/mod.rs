use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

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
        DescriptionEntry {
            description: "`!#[init]` marks a zero-argument function to run once before the entry point",
            examples: &["!#[init]\nfn setup() {\n    std::io::println(\"starting up\")\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`!#[final]` marks a zero-argument function to run once after the entry point finishes",
            examples: &["!#[final]\nfn cleanup() {\n    std::io::println(\"shutting down\")\n}"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`!#[test]` marks a zero-argument function as a test, run by the test runner instead of the normal entry point",
            examples: &[
                "!#[test]\nfn adds_correctly() {\n    if 2 + 2 != 4 {\n        panic(\"math is broken\")\n    }\n}",
            ],
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
