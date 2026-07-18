use crate::entry::FnEntry;

pub static CHARS: FnEntry = FnEntry {
    signature: "chars(str)",
    description: "returns a char array of each character in the string",
    example: "get std::str::chars\n\nchars(\"hi\")",
    expected_output: Some("['h', 'i']"),
    returns: "arr[char]",
    errors: None,
    see_also: &["bytes", "char_at"],
    since: Some("v0.1.5"),
};
