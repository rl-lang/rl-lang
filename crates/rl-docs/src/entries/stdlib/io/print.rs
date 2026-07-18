use crate::entry::FnEntry;

pub static PRINT: FnEntry = FnEntry {
    signature: "print(x, ...)",
    description: "prints any number of values without a trailing newline",
    example: "get std::io::print\n\nprint(\"hello\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["println"],
    since: Some("v0.1.5"),
};
