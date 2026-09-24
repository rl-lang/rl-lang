use crate::entry::FnEntry;

pub static IS_SBYTE: FnEntry = FnEntry {
    signature: "is_sbyte(v)",
    description: "true if v is of type sbyte (signed byte)",
    example: "get std::types::is_sbyte\n\nis_sbyte(-42 as sbyte)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_byte", "is_bsbyte"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
