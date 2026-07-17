use crate::entry::FnEntry;

pub static NOW: FnEntry = FnEntry {
    signature: "time_now()",
    description: "returns the current unix timestamp as an int (seconds since epoch)",
    example: r#"get std::time::time_now

time_now()"#,
    expected_output: Some("1784305315"),
    returns: "int",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
