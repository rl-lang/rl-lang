use crate::entry::FnEntry;

pub static IS_SUINT: FnEntry = FnEntry {
    signature: "is_suint(v)",
    description: "true if v is of type suint (small uint)",
    example: "get std::types::is_suint\n\nis_suint(42 as suint)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_uint", "is_sint"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
