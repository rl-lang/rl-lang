use crate::entry::FnEntry;

pub static IS_MAP: FnEntry = FnEntry {
    signature: "is_map(v)",
    description: "true if v is of type map[K, V]",
    example: "get std::types::is_map\n\nis_map({\"a\": 1})",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_array", "is_set"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
