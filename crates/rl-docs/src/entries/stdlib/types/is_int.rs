use crate::entry::FnEntry;

pub static IS_INT: FnEntry = FnEntry {
    signature: "is_int(x)",
    description: "true if x is an int",
    example: "get std::types::is_int\n\nis_int(42)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_int"],
    since: Some("v0.1.5"),
};
