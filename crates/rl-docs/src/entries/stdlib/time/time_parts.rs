use crate::entry::FnEntry;

pub static TIME_PARTS: FnEntry = FnEntry {
    signature: "time_parts(timestamp)",
    description: "returns an array of [year, month, day, hour, minute, second] for the given unix timestamp (UTC)",
    example: "get std::time::time_parts\n\ntime_parts(1719000000) // [2024, 6, 21, 20, 0, 0]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
