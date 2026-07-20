use crate::entry::FnEntry;

pub static CONTAINS: FnEntry = FnEntry {
    signature: "contains(str, sub)",
    description: "true if str contains the substring sub",
    example: "get std::str::contains\n\ncontains(\"hello\", \"ell\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["starts_with", "ends_with", "index_of"],
    since: Some("v0.1.5"),
};
