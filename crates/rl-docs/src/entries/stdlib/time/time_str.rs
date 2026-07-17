use crate::entry::FnEntry;

pub static TIME_STR: FnEntry = FnEntry {
    signature: "format_time_str(timestamp)",
    description: "shorthand for format_time with \"%H:%M:%S\", returns the time portion of a unix timestamp",
    example: "get std::time::format_time_str\n\nformat_time_str(1719000000) // \"20:00:00\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
