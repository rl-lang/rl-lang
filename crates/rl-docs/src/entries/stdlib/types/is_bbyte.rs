use crate::entry::FnEntry;

pub static IS_BBYTE: FnEntry = FnEntry {
    signature: "is_bbyte(v)",
    description: "true if v is of type bbyte (big byte)",
    example: "get std::types::is_bbyte\n\nis_bbyte(200 as bbyte)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_byte", "is_bsbyte"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
