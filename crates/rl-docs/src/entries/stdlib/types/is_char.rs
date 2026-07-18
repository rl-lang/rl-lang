use crate::entry::FnEntry;

pub static IS_CHAR: FnEntry = FnEntry {
    signature: "is_char(x)",
    description: "true if x is a char",
    example: "get std::types::is_char\n\nis_char('a')",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_char"],
    since: Some("v0.1.5"),
};
