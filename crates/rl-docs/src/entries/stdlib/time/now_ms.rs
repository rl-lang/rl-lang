use crate::entry::FnEntry;

pub static NOW_MS: FnEntry = FnEntry {
    signature: "time_now_ms()",
    description: "returns the current unix timestamp in milliseconds, useful for generating unique IDs",
    example: "get std::time::time_now_ms\n\ntime_now_ms() // 1719000000000",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
