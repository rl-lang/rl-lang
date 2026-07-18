use crate::entry::FnEntry;

static TRIM: FnEntry = FnEntry {
    signature: "trim(str)",
    description: "removes leading and trailing whitespace from str",
    example: "get std::str::trim\n\ntrim(\"  hi  \")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim_start", "trim_end"],
    since: Some("v0.1.5"),
};
