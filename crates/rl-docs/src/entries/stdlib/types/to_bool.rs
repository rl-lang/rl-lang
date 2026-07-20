use crate::entry::FnEntry;

pub static TO_BOOL: FnEntry = FnEntry {
    signature: "to_bool(x)",
    description: "converts int, float, string, or null to bool - 0/0.0/\"\"/null are false, everything else is true",
    example: "get std::types::to_bool\n\nto_bool(0)",
    expected_output: Some("false"),
    returns: "result[bool]",
    errors: Some("Will return err when it fails to parse to boolean"),
    see_also: &["is_bool"],
    since: Some("v0.1.5"),
};
