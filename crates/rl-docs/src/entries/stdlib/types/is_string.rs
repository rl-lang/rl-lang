use crate::entry::FnEntry;

pub static IS_STRING: FnEntry = FnEntry {
    signature: "is_string(x)",
    description: "true if x is a string",
    example: "get std::types::is_string\n\nis_string(\"hi\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["to_string"],
    since: Some("v0.1.5"),
};
