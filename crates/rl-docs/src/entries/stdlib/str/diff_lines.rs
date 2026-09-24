use crate::entry::FnEntry;

pub static DIFF_LINES: FnEntry = FnEntry {
    signature: "diff_lines(a, b)",
    description: "returns a list of line-by-line differences between two strings",
    example: "get std::str::diff_lines\n\ndiff_lines(\"a\\nb\", \"a\\nc\")",
    expected_output: Some("[\"- b\", \"+ c\"]"),
    returns: "arr[string]",
    errors: None,
    see_also: &[],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
