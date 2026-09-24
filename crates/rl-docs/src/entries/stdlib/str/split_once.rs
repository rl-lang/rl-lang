use crate::entry::FnEntry;

pub static SPLIT_ONCE: FnEntry = FnEntry {
    signature: "split_once(s, sep)",
    description: "splits the string into two parts at the first occurrence of sep",
    example: "get std::str::split_once\n\nsplit_once(\"a-b-c\", \"-\")",
    expected_output: Some("[\"a\", \"b-c\"]"),
    returns: "tuple[string, string]",
    errors: None,
    see_also: &["split"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
