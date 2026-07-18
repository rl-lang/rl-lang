use crate::entry::FnEntry;

pub static IS_EMPTY: FnEntry = FnEntry {
    signature: "is_empty(str)",
    description: "true if the string has no characters",
    example: "get std::str::is_empty\n\nis_empty(\"\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
