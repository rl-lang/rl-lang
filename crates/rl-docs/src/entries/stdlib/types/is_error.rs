use crate::entry::FnEntry;

pub static IS_ERROR: FnEntry = FnEntry {
    signature: "is_error(x)",
    description: "true if x is an error",
    example: "get std::types::is_error\n\nis_error(error(42))",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["unwrap_error"],
    since: Some("v0.1.5"),
};
