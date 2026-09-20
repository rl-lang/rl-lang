use crate::entry::FnEntry;

pub static FORMAT_TIME: FnEntry = FnEntry {
    signature: "format_time(timestamp, pattern)",
    description: "formats a unix timestamp into a readable string using a pattern. tokens: %Y (4-digit year), %y (2-digit year), %m (month 01-12), %B (full month name), %b (abbreviated month), %d (day 01-31), %A (full weekday name), %a (abbreviated weekday), %w (weekday 0-6), %j (day of year 001-366), %U (week number, Sunday start), %W (week number, Monday start), %V (ISO week number), %H (hour 00-23), %I (hour 01-12), %M (minute), %S (second), %p (AM/PM), %P (am/pm), %z (+0000 offset), %Z (timezone name)",
    example: r#"get std::time::format_time

format_time(1784305948, "%Y-%m-%d %H:%M:%S")?
format_time(1784305948, "%A, %B %d %Y")?
format_time(1784305948, "%I:%M %p")?"#,
    expected_output: Some("2026-07-17 16:32:29\nThursday, July 17 2026\n04:32 PM"),
    returns: "result[string]",
    errors: Some("Will return error on negative timestamp"),
    see_also: &[],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
