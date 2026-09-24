use crate::entry::FnEntry;

pub static INDENT: FnEntry = FnEntry {
    signature: "indent(s, n)",
    description: "adds n spaces to the beginning of each line",
    example: "get std::str::indent\n\nindent(\"hello\", 2)",
    expected_output: Some("\"  hello\""),
    returns: "string",
    errors: None,
    see_also: &["dedent"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
