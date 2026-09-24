use crate::entry::FnEntry;

pub static IS_ARRAY: FnEntry = FnEntry {
    signature: "is_array(v)",
    description: "true if v is of type arr[T]",
    example: "get std::types::is_array\n\nis_array([1, 2, 3])",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_map", "is_set"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
