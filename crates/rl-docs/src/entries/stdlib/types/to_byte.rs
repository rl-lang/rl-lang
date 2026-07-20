use crate::entry::FnEntry;

pub static TO_BYTE: FnEntry = FnEntry {
    signature: "to_byte(x)",
    description: "converts int, float, bool, char, or string (including 0x hex strings) to byte",
    example: "get std::types::to_byte\n\nto_byte(\"25\")",
    expected_output: Some("25"),
    returns: "result[byte]",
    errors: Some("Will return err when it fails to parse to byte"),
    see_also: &["is_byte"],
    since: Some("v0.1.5"),
};
