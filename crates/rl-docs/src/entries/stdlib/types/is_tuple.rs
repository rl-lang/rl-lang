use crate::entry::FnEntry;

pub static IS_TUPLE: FnEntry = FnEntry {
    signature: "is_tuple(v)",
    description: "true if v is of type tuple",
    example: "get std::types::is_tuple\n\nis_tuple((1, 2, 3))",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_array"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
