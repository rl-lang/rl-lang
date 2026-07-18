use crate::entry::FnEntry;

static COUNT: FnEntry = FnEntry {
    signature: "count(str, sub)",
    description: "returns the number of non-overlapping occurrences of sub in str",
    example: "get std::str::count\n\ncount(\"banana\", \"an\")",
    expected_output: Some("2"),
    returns: "int",
    errors: None,
    see_also: &["contains", "index_of"],
    since: Some("v0.1.5"),
};
