use crate::entry::FnEntry;

pub static FORMAT_TIME: FnEntry = FnEntry {
    signature: "format_time(timestamp, pattern)",
    description: "formats a unix timestamp into a readable string using a pattern. supported tokens: %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)",
    example: "get std::time::format_time\n\nformat_time(1719000000, \"%Y-%m-%d %H:%M:%S\") // \"2024-06-21 20:00:00\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
