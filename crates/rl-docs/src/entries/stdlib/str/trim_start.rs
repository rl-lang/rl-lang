use crate::entry::FnEntry;

pub static TRIM_START: FnEntry = FnEntry {
    signature: "trim_start(str)",
    description: "removes leading whitespace from str",
    example: "get std::str::trim_start\n\ntrim_start(\"  hi\")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim", "trim_end"],
    since: Some("v0.1.5"),
};
