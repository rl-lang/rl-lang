use crate::docs::entry::{FnEntry, StdEntry};

pub static TIME: StdEntry = StdEntry {
    name: "time",
    description: "functions for getting the current time, formatting timestamps, and time arithmetic",
    functions: FUNCTIONS,
};

static FUNCTIONS: &[&FnEntry] = &[
    &NOW,
    &NOW_MS,
    &FORMAT_TIME,
    &DATE_STR,
    &TIME_STR,
    &TIME_ADD,
    &TIME_DIFF,
    &TIME_PARTS,
];

static NOW: FnEntry = FnEntry {
    signature: "time_now()",
    description: "returns the current unix timestamp as an int (seconds since epoch)",
    example: "get std::time::time_now\n\ntime_now() // 1719000000",
};

static NOW_MS: FnEntry = FnEntry {
    signature: "time_now_ms()",
    description: "returns the current unix timestamp in milliseconds, useful for generating unique IDs",
    example: "get std::time::time_now_ms\n\ntime_now_ms() // 1719000000000",
};

static FORMAT_TIME: FnEntry = FnEntry {
    signature: "format_time(timestamp, pattern)",
    description: "formats a unix timestamp into a readable string using a pattern. supported tokens: %Y (year), %m (month), %d (day), %H (hour), %M (minute), %S (second)",
    example: "get std::time::format_time\n\nformat_time(1719000000, \"%Y-%m-%d %H:%M:%S\") // \"2024-06-21 20:00:00\"",
};

static DATE_STR: FnEntry = FnEntry {
    signature: "format_date_str(timestamp)",
    description: "shorthand for format_time with \"%Y-%m-%d\", returns the date portion of a unix timestamp",
    example: "get std::time::format_date_str\n\nformat_date_str(1719000000) // \"2024-06-21\"",
};

static TIME_STR: FnEntry = FnEntry {
    signature: "format_time_str(timestamp)",
    description: "shorthand for format_time with \"%H:%M:%S\", returns the time portion of a unix timestamp",
    example: "get std::time::format_time_str\n\nformat_time_str(1719000000) // \"20:00:00\"",
};

static TIME_ADD: FnEntry = FnEntry {
    signature: "time_add(timestamp, seconds)",
    description: "adds a number of seconds to a timestamp and returns the new timestamp",
    example: "get std::time::time_add\n\ntime_add(1719000000, 3600) // 1719003600 (one hour later)",
};

static TIME_DIFF: FnEntry = FnEntry {
    signature: "time_diff(a, b)",
    description: "returns the difference between two timestamps in seconds (a - b)",
    example: "get std::time::time_diff\n\ntime_diff(1719003600, 1719000000) // 3600",
};

static TIME_PARTS: FnEntry = FnEntry {
    signature: "time_parts(timestamp)",
    description: "returns an array of [year, month, day, hour, minute, second] for the given unix timestamp (UTC)",
    example: "get std::time::time_parts\n\ntime_parts(1719000000) // [2024, 6, 21, 20, 0, 0]",
};
