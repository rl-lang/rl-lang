use crate::entry::FnEntry;

pub static TO_INT: FnEntry = FnEntry {
    signature: "to_int(x)",
    description: "converts float, bool, char, or string (including 0x hex strings) to int",
    example: "get std::types::to_int\n\nto_int(\"0xff\")",
    expected_output: Some("255"),
    returns: "result[int]",
    errors: Some("Will return err when it fails to parse to int"),
    see_also: &["is_int"],
    since: Some("v0.1.5"),
};
