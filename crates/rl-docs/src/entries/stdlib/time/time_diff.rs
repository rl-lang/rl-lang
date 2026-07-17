use crate::entry::FnEntry;

pub static TIME_DIFF: FnEntry = FnEntry {
    signature: "time_diff(a, b)",
    description: "returns the difference between two timestamps in seconds (a - b)",
    example: r#"get std::time::time_diff

time_diff(1719003600, 1719000000)"#,
    expected_output: Some("3600"),
    returns: "int",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
