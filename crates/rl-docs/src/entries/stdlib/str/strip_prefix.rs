use crate::entry::FnEntry;

pub static STRIP_PREFIX: FnEntry = FnEntry {
    signature: "strip_prefix(s, prefix)",
    description: "removes the given prefix from the start of the string",
    example: "get std::str::strip_prefix\n\nstrip_prefix(\"hello world\", \"hello\")",
    expected_output: Some("\" world\""),
    returns: "string",
    errors: None,
    see_also: &["strip_suffix", "trim_start"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
