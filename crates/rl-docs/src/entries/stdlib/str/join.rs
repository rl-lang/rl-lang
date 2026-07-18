use crate::entry::FnEntry;

static JOIN: FnEntry = FnEntry {
    signature: "join(arr, delim)",
    description: "joins an array into a string with delim between each element",
    example: "get std::str::join\n\njoin([\"a\", \"b\", \"c\"], \"-\")?",
    expected_output: Some("\"a-b-c\""),
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` contains a function, lambda, or enclosure value\n\nNote: array elements that are themselves arrays, records, or tags are\nsilently dropped from the joined output rather than erroring or being\nstringified.",
    ),
    see_also: &["split", "concat"],
    since: Some("v0.1.5"),
};
