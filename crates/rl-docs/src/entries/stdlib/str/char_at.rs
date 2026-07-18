use crate::entry::FnEntry;

pub static CHAR_AT: FnEntry = FnEntry {
    signature: "char_at(str, index)",
    description: "returns the character at the given index",
    example: "get std::str::char_at\n\nchar_at(\"hello\", 1)?",
    expected_output: Some("'e'"),
    returns: "result[char]",
    errors: Some(
        "Will return error on the following:\n\n- `index` is negative\n- `index` is out of bounds for `str`",
    ),
    see_also: &["chars", "slice"],
    since: Some("v0.1.5"),
};
