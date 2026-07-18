use crate::entry::FnEntry;

pub static PRINTLN: FnEntry = FnEntry {
    signature: "println(x, ...)",
    description: "prints any number of values followed by a newline",
    example: "get std::io::println\n\nprintln(\"hello\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["print"],
    since: Some("v0.1.5"),
};
