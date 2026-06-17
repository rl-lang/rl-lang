use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static COMMENTS: ConceptEntry = ConceptEntry {
    name: "comments",
    descriptions: &[DescriptionEntry {
        description: "single-line comments start with `//` everything after is ignored",
        examples: &["// this is a comment\ndec int x = 10  // inline comment"],
    }],
};
