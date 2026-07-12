use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static LOGICAL_OPERATORS: ConceptEntry = ConceptEntry {
    name: "logical operators",
    descriptions: &[
        DescriptionEntry {
            description: "`and` evaluates to true only if both sides are true",
            examples: &["true and true    // true", "true and false   // false"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`or` evaluates to true if either side is true",
            examples: &["false or true    // true", "false or false   // false"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`and`/`or` combine naturally with comparisons inside conditions",
            examples: &[
                "dec int age = 20\ndec bool has_id = true\n\nif age >= 18 and has_id {\n    println(\"allowed\")\n}",
            ],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
        DescriptionEntry {
            description: "`!` (not) negates a bool and binds tighter than `and`/`or`",
            examples: &["dec bool ok = !false and true  // true"],
            kind: DescriptionKind::Explanation,
            title: None,
            expected_output: &[],
        },
    ],
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &["operators"],
    pitfalls: &[],
    related: &["operators"],
    related_stdlib: &[],
    since: None,
};
