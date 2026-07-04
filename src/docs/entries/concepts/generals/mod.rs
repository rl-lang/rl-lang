use crate::docs::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};

pub static COMMENTS: ConceptEntry = ConceptEntry {
    name: "comments",
    summary: "",
    category: ConceptCategory::Syntax,
    prerequisites: &[],
    descriptions: &[DescriptionEntry {
        kind: DescriptionKind::Explanation,
        title: None,
        description: "single-line comments start with `//` everything after is ignored",
        examples: &["// this is a comment\ndec int x = 10  // inline comment"],
        expected_output: &[],
    }],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
