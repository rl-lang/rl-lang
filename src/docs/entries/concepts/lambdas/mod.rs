use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static LAMBDAS: ConceptEntry = ConceptEntry {
    name: "lambdas",
    summary: "anonymous, inline functions that can capture their surrounding scope",
    category: ConceptCategory::Functions,
    prerequisites: &["functions", "variables", "types"],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("defining a lambda"),
            description: "lambdas are anonymous functions defined inline with `fn(<type> <param>, ...) { <body> }`",
            examples: &[
                "dec fn square = fn(int x) -> int {\n    return x * x\n}\n\nprintln(square(5))  // 25",
            ],
            expected_output: &["25"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("closures"),
            description: "lambdas capture variables from their surrounding scope (closures)",
            examples: &[
                "dec int factor = 3\n\ndec fn triple = fn(int x) -> int {\n    return x * factor\n}\n\nprintln(triple(4))  // 12",
            ],
            expected_output: &["12"],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: Some("captured values are snapshotted, not live"),
            description: "a lambda captures the value of a variable at creation time - reassigning the outer variable afterward does not change what the lambda sees",
            examples: &[
                "dec int factor = 3\ndec fn triple = fn(int x) -> int {\n    return x * factor\n}\n\nfactor = 100\nprintln(triple(4))  // still 12, not 400",
            ],
            expected_output: &["12"],
        },
    ],
    pitfalls: &["captured variables are snapshotted at creation, not live-referenced"],
    related: &["functions", "closures"],
    related_stdlib: &[],
    since: None,
};
