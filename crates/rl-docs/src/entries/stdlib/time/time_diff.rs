use crate::entry::FnEntry;

pub static TIME_DIFF: FnEntry = FnEntry {
    signature: "time_diff(a, b)",
    description: "returns the difference between two timestamps in seconds (a - b)",
    example: "get std::time::time_diff\n\ntime_diff(1719003600, 1719000000) // 3600",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
