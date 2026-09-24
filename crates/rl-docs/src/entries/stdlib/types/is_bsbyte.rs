use crate::entry::FnEntry;

pub static IS_BSBYTE: FnEntry = FnEntry {
    signature: "is_bsbyte(v)",
    description: "true if v is of type bsbyte (big signed byte)",
    example: "get std::types::is_bsbyte\n\nis_bsbyte(-100 as bsbyte)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_sbyte", "is_bbyte"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
