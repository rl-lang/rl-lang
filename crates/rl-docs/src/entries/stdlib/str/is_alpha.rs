use crate::entry::FnEntry;

pub static IS_ALPHA: FnEntry = FnEntry {
    signature: "is_alpha(s)",
    description: "true if the string contains only alphabetic characters",
    example: "get std::str::is_alpha\n\nis_alpha(\"hello\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_numeric", "is_whitespace"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
