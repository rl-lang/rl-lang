use crate::entry::FnEntry;

pub static REPEAT: FnEntry = FnEntry {
    signature: "repeat(str, count)",
    description: "returns str repeated count times",
    example: "get std::str::repeat\n\nrepeat(\"ab\", 3)",
    expected_output: Some("\"ababab\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
