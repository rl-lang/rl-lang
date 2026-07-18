use crate::entry::FnEntry;

pub static TO_UPPER: FnEntry = FnEntry {
    signature: "to_upper(str)",
    description: "returns str with all characters converted to uppercase",
    example: "get std::str::to_upper\n\nto_upper(\"hello\")",
    expected_output: Some("\"HELLO\""),
    returns: "string",
    errors: None,
    see_also: &["to_lower"],
    since: Some("v0.1.5"),
};
