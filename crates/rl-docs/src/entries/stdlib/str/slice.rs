use crate::entry::FnEntry;

pub static SLICE: FnEntry = FnEntry {
    signature: "slice(str, start, end)",
    description: "returns a substring from start to end (exclusive)",
    example: "get std::str::slice\n\nslice(\"hello\", 1, 4)?",
    expected_output: Some("\"ell\""),
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `start` or `end` is out of bounds for `str`\n\nNote: passing `end` < `start` is not validated as an error and will panic\nat runtime (integer underflow) rather than returning a `result[string]` err.",
    ),
    see_also: &["char_at"],
    since: Some("v0.1.5"),
};
