use crate::entry::FnEntry;

pub static IS_SINT: FnEntry = FnEntry {
    signature: "is_sint(v)",
    description: "true if v is of type sint (small int)",
    example: "get std::types::is_sint\n\nis_sint(42 as sint)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_int", "is_suint"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
