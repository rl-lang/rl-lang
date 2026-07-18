use crate::entry::FnEntry;

pub static REPLACE: FnEntry = FnEntry {
    signature: "replace(str, from, to)",
    description: "replaces all occurrences of from with to in str",
    example: "get std::str::replace\n\nreplace(\"foo bar foo\", \"foo\", \"baz\")",
    expected_output: Some("\"baz bar baz\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
