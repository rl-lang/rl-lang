use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static NULL: ConceptEntry = ConceptEntry {
    name: "null",
    descriptions: &[DescriptionEntry {
        description: "`null` represents the absence of a value also functions that return nothing implicitly return null",
        examples: &[
            "dec int x = null  // x holds null",
            "fn do_nothing() {\n    // implicitly returns null\n}",
        ],
    }],
};
