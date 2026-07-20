use crate::entry::FnEntry;

pub static TO_BIN: FnEntry = FnEntry {
    signature: "to_bin(x)",
    description: "converts int, char, or string to a binary string representation",
    example: "get std::types::to_bin\n\nto_bin(10)?",
    expected_output: Some("\"1010\""),
    returns: "result[string]",
    errors: Some("Will return err when it fails to parse to binary"),
    see_also: &[],
    since: Some("v0.1.5"),
};
