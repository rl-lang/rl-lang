use crate::entry::FnEntry;

pub static LINES: FnEntry = FnEntry {
    signature: "lines(s)",
    description: "splits the string into lines by newline",
    example: "get std::str::lines\n\nlines(\"a\\nb\\nc\")",
    expected_output: Some("[\"a\", \"b\", \"c\"]"),
    returns: "arr[string]",
    errors: None,
    see_also: &["split"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
