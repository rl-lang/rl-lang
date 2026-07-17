use crate::entry::FnEntry;

pub static TIME_ADD: FnEntry = FnEntry {
    signature: "time_add(timestamp, seconds)",
    description: "adds a number of seconds to a timestamp and returns the new timestamp",
    example: "get std::time::time_add\n\ntime_add(1719000000, 3600) // 1719003600 (one hour later)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
