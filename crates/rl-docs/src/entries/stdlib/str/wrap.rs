use crate::entry::FnEntry;

pub static WRAP: FnEntry = FnEntry {
    signature: "wrap(s, width)",
    description: "wraps the string to the given width, breaking at word boundaries",
    example: "get std::str::wrap\n\nwrap(\"hello world foo\", 10)",
    expected_output: Some("\"hello\\nworld foo\""),
    returns: "string",
    errors: None,
    see_also: &["indent", "dedent"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
