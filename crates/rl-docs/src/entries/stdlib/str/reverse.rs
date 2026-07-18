use crate::entry::FnEntry;

pub static REVERSE: FnEntry = FnEntry {
    signature: "reverse(str)",
    description: "returns str with characters in reverse order",
    example: "get std::str::reverse\n\nreverse(\"hello\")",
    expected_output: Some("\"olleh\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
