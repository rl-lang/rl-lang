use crate::entry::FnEntry;

pub static IS_SET: FnEntry = FnEntry {
    signature: "is_set(v)",
    description: "true if v is of type set[T]",
    example: "get std::types::is_set\n\nis_set({1, 2, 3})",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_array", "is_map"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
