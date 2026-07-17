use crate::entry::FnEntry;

pub static DATE_STR: FnEntry = FnEntry {
    signature: "format_date_str(timestamp)",
    description: "shorthand for format_time with \"%Y-%m-%d\", returns the date portion of a unix timestamp",
    example: "get std::time::format_date_str\n\nformat_date_str(1719000000) // \"2024-06-21\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
