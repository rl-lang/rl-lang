use crate::entry::FnEntry;

pub static TO_HEX: FnEntry = FnEntry {
    signature: "to_hex(x)",
    description: "converts int, char, or string to a hexadecimal string representation",
    example: "get std::types::to_hex\n\nto_hex(255)",
    expected_output: Some("\"ff\""),
    returns: "result[string]",
    errors: Some("Will return err when it fails to parse to hexadecimal"),
    see_also: &[],
    since: Some("v0.1.5"),
};
