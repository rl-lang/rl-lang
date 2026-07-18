use crate::entry::FnEntry;

pub static TO_FLOAT: FnEntry = FnEntry {
    signature: "to_float(x)",
    description: "converts int, bool, or numeric string to float",
    example: "get std::types::to_float\n\nto_float(3)",
    expected_output: Some("3.0"),
    returns: "result[float]",
    errors: Some("Will return err when it fails to parse to float"),
    see_also: &["is_float"],
    since: Some("v0.1.5"),
};
