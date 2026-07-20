use crate::entry::FnEntry;

pub static IS_BYTE: FnEntry = FnEntry {
    signature: "is_byte(x)",
    description: "true if x is an byte",
    example: "get std::types::is_byte\n\nis_byte(42 as byte)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_byte"],
    since: Some("v0.1.5"),
};
