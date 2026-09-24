use crate::entry::FnEntry;

pub static IS_NUMERIC: FnEntry = FnEntry {
    signature: "is_numeric(s)",
    description: "true if the string contains only digit characters",
    example: "get std::str::is_numeric\n\nis_numeric(\"12345\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_alpha", "is_whitespace"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
