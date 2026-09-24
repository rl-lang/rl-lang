use crate::entry::FnEntry;

pub static DEDENT: FnEntry = FnEntry {
    signature: "dedent(s)",
    description: "removes common leading whitespace from all lines",
    example: "get std::str::dedent\n\ndedent(\"  hello\\n  world\")",
    expected_output: Some("\"hello\\nworld\""),
    returns: "string",
    errors: None,
    see_also: &["indent"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
