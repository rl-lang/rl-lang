use crate::entry::FnEntry;

pub static IS_BOOL: FnEntry = FnEntry {
    signature: "is_bool(x)",
    description: "true if x is a bool",
    example: "get std::types::is_bool\n\nis_bool(true)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_bool"],
    since: Some("v0.1.5"),
};
