use crate::entry::FnEntry;

pub static IS_UINT: FnEntry = FnEntry {
    signature: "is_uint(v)",
    description: "true if v is of type uint",
    example: "get std::types::is_uint\n\nis_uint(42)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_int"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
