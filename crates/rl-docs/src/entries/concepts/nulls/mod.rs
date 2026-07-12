use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static NULL: ConceptEntry = ConceptEntry {
    name: "null",
    summary: "",
    category: ConceptCategory::Types,
    prerequisites: &[],
    descriptions: &[DescriptionEntry {
        kind: DescriptionKind::Explanation,
        title: None,
        description: "`null` represents the absence of a value also functions that return nothing implicitly return null",
        examples: &[
            "dec int x = null  // x holds null",
            "fn do_nothing() {\n    // implicitly returns null\n}",
        ],
        expected_output: &[],
    }],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
