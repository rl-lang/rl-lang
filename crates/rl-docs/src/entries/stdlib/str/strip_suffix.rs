use crate::entry::FnEntry;

pub static STRIP_SUFFIX: FnEntry = FnEntry {
    signature: "strip_suffix(s, suffix)",
    description: "removes the given suffix from the end of the string",
    example: "get std::str::strip_suffix\n\nstrip_suffix(\"hello world\", \"world\")",
    expected_output: Some("\"hello \""),
    returns: "string",
    errors: None,
    see_also: &["strip_prefix", "trim_end"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
