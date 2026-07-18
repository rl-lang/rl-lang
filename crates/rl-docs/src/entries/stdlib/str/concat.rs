use crate::entry::FnEntry;

static CONCAT: FnEntry = FnEntry {
    signature: "concat(a, b, ...)",
    description: "concatenates any number of values into a single string",
    example: "get std::str::concat\n\nconcat(\"hello\", \" \", \"world\")",
    expected_output: Some("\"hello world\""),
    returns: "string",
    errors: None,
    see_also: &["join", "format"],
    since: Some("v0.1.5"),
};
