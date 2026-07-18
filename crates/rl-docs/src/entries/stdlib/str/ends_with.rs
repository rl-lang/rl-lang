use crate::entry::FnEntry;

pub static ENDS_WITH: FnEntry = FnEntry {
    signature: "ends_with(str, sub)",
    description: "true if str ends with sub",
    example: "get std::str::ends_with\n\nends_with(\"hello\", \"lo\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["starts_with", "contains"],
    since: Some("v0.1.5"),
};
