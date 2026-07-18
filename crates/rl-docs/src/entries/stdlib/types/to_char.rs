use crate::entry::FnEntry;

pub static TO_CHAR: FnEntry = FnEntry {
    signature: "to_char(x)",
    description: "converts an int (unicode codepoint) or single-character string to char",
    example: "get std::types::to_char\n\nto_char(65)",
    expected_output: Some("\'A\'"),
    returns: "result[char]",
    errors: Some("Will return err when it fails to parse to character"),
    see_also: &["is_char"],
    since: Some("v0.1.5"),
};
