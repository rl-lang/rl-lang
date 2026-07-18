use crate::entry::FnEntry;

pub static TO_OCT: FnEntry = FnEntry {
    signature: "to_oct(x)",
    description: "converts int, char, or string to an octal string representation",
    example: "get std::types::to_oct\n\nto_oct(8)",
    expected_output: Some("\"10\""),
    returns: "result[string]",
    errors: Some("Will return err when it fails to parse to octal"),
    see_also: &[],
    since: Some("v0.1.5"),
};
