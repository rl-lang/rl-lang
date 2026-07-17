use crate::entry::FnEntry;

pub static NOW: FnEntry = FnEntry {
    signature: "time_now()",
    description: "returns the current unix timestamp as an int (seconds since epoch)",
    example: "get std::time::time_now\n\ntime_now() // 1719000000",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
