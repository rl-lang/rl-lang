use crate::entry::FnEntry;

pub static IS_WHITESPACE: FnEntry = FnEntry {
    signature: "is_whitespace(s)",
    description: "true if the string contains only whitespace characters",
    example: "get std::str::is_whitespace\n\nis_whitespace(\"  \\t\\n\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_alpha", "is_numeric"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
