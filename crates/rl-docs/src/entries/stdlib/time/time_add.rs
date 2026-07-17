use crate::entry::FnEntry;

pub static TIME_ADD: FnEntry = FnEntry {
    signature: "time_add(timestamp, seconds)",
    description: "adds a number of seconds to a timestamp and returns the new timestamp",
    example: r#"get std::time::time_add

time_add(1719000000, 3600)"#,
    expected_output: Some("1719003600"),
    returns: "int",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
