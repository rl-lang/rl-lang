use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static MATCH: ConceptEntry = ConceptEntry {
    name: "match",
    summary: "branch on a value against literal patterns, with `_` as a catch-all",
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Syntax,
            title: None,
            description: "`match <value> { <pattern> => { <block> } ... }` runs the block whose pattern matches the value",
            examples: &[
                "match x {\n    1 => { println(\"one\") }\n    2 => { println(\"two\") }\n    _ => { println(\"other\") }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: Some("wildcard arm"),
            description: "`_` matches anything not caught by an earlier arm; without it, a value that matches no arm falls through silently",
            examples: &[
                "dec int x = 5\n\nmatch x {\n    1 => { println(\"one\") }\n    _ => { println(\"something else\") }\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Pitfall,
            title: None,
            description: "each arm only matches a single literal value; there's no OR-pattern (`1 | 2 =>`) or variable binding yet, so ranges or destructuring need `if`/`elif` instead",
            examples: &[],
            expected_output: &[],
        },
    ],
    category: ConceptCategory::ControlFlow,
    prerequisites: &["control flow"],
    pitfalls: &["arms only support literals and `_`, no OR-patterns or bindings"],
    related: &["control flow"],
    related_stdlib: &[],
    since: None,
};
