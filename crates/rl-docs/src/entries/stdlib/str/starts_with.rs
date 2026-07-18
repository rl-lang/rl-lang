use crate::entry::FnEntry;

pub static STARTS_WITH: FnEntry = FnEntry {
    signature: "starts_with(str, sub)",
    description: "true if str starts with sub",
    example: "get std::str::starts_with\n\nstarts_with(\"hello\", \"he\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["ends_with", "contains"],
    since: Some("v0.1.5"),
};
