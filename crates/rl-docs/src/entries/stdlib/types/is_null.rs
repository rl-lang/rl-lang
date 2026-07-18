use crate::entry::FnEntry;

pub static IS_NULL: FnEntry = FnEntry {
    signature: "is_null(x)",
    description: "true if x is null",
    example: "get std::types::is_null\n\nis_null(null)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
