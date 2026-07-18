use crate::entry::FnEntry;

pub static NOW_MS: FnEntry = FnEntry {
    signature: "time_now_ms()",
    description: "returns the current unix timestamp in milliseconds, useful for generating unique IDs",
    example: r#"get std::time::time_now_ms

time_now_ms()"#,
    expected_output: Some("1784305315000"),
    returns: "int",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
