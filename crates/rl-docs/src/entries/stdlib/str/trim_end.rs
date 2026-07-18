use crate::entry::FnEntry;

static TRIM_END: FnEntry = FnEntry {
    signature: "trim_end(str)",
    description: "removes trailing whitespace from str",
    example: "get std::str::trim_end\n\ntrim_end(\"hi  \")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim", "trim_start"],
    since: Some("v0.1.5"),
};
