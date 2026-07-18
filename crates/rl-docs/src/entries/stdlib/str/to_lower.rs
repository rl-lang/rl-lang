use crate::entry::FnEntry;

pub static TO_LOWER: FnEntry = FnEntry {
    signature: "to_lower(str)",
    description: "returns str with all characters converted to lowercase",
    example: "get std::str::to_lower\n\nto_lower(\"HELLO\")",
    expected_output: Some("\"hello\""),
    returns: "string",
    errors: None,
    see_also: &["to_upper"],
    since: Some("v0.1.5"),
};
