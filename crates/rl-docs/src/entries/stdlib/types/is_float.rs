use crate::entry::FnEntry;

pub static IS_FLOAT: FnEntry = FnEntry {
    signature: "is_float(x)",
    description: "true if x is a float",
    example: "get std::types::is_float\n\nis_float(3.14)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_float"],
    since: Some("v0.1.5"),
};
