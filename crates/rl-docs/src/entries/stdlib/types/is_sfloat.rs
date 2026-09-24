use crate::entry::FnEntry;

pub static IS_SFLOAT: FnEntry = FnEntry {
    signature: "is_sfloat(v)",
    description: "true if v is of type sfloat (small float)",
    example: "get std::types::is_sfloat\n\nis_sfloat(3.14 as sfloat)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_float"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
