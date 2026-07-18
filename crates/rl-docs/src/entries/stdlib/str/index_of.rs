use crate::entry::FnEntry;

pub static INDEX_OF: FnEntry = FnEntry {
    signature: "index_of(str, sub)",
    description: "returns the character index of the first occurrence of sub, or -1 if not found",
    example: "get std::str::index_of\n\nindex_of(\"hello\", \"ll\")",
    expected_output: Some("2"),
    returns: "int",
    errors: None,
    see_also: &["contains", "count"],
    since: Some("v0.1.5"),
};
