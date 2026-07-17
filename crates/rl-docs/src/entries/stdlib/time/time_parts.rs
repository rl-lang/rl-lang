use crate::entry::FnEntry;

pub static TIME_PARTS: FnEntry = FnEntry {
    signature: "time_parts(timestamp)",
    description: "returns an array of [year, month, day, hour, minute, second] for the given unix timestamp (UTC)",
    example: r#"get std::time::time_parts

time_parts(1784306758)?"#,
    expected_output: Some("[2026, 7, 17, 16, 46, 0]"),
    returns: "result[arr[int]]",
    errors: Some("Will return error on negative timestamp"),
    see_also: &[],
    since: Some("v0.1.5"),
};
