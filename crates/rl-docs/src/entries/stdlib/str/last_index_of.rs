use crate::entry::FnEntry;

pub static LAST_INDEX_OF: FnEntry = FnEntry {
    signature: "last_index_of(s, sub)",
    description: "returns the last index of sub in s, or -1 if not found",
    example: "get std::str::last_index_of\n\nlast_index_of(\"hello\", \"l\")",
    expected_output: Some("3"),
    returns: "int",
    errors: None,
    see_also: &["index_of", "contains"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
