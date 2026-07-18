use crate::entry::FnEntry;

pub static SPLIT: FnEntry = FnEntry {
    signature: "split(str, delim)",
    description: "splits str by delim and returns a string array",
    example: "get std::str::split\n\nsplit(\"a,b,c\", \",\")",
    expected_output: Some("[\"a\", \"b\", \"c\"]"),
    returns: "arr[string]",
    errors: None,
    see_also: &["join"],
    since: Some("v0.1.5"),
};
