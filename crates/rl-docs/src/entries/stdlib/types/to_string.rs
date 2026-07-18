use crate::entry::FnEntry;

pub static TO_STRING: FnEntry = FnEntry {
    signature: "to_string(x)",
    description: "converts int, float, bool, or char to string",
    example: "get std::types::to_string\n\nto_string(42)",
    expected_output: Some("\"42\""),
    returns: "result[string]",
    errors: Some("Will return err when it fails to parse to string"),
    see_also: &["is_string"],
    since: Some("v0.1.5"),
};
