use crate::entry::FnEntry;

pub static FORMAT_TIME: FnEntry = FnEntry {
    signature: "format_time(timestamp, pattern)",
    description: "formats a unix timestamp into a readable string using a pattern. supported tokens: %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)",
    example: r#"get std::time::format_time

format_time(1784305948, "%Y-%m-%d %H:%M:%S")?"#,
    expected_output: Some("2026-07-17 16:32:29"),
    returns: "result[string]",
    errors: Some("Will return error on negative timestamp"),
    see_also: &[],
    since: Some("v0.1.5"),
};
