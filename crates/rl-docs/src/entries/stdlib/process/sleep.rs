use crate::entry::FnEntry;

pub static SLEEP: FnEntry = FnEntry {
    signature: "sleep(ms)",
    description: "pauses the process for the given number of milliseconds",
    example: r#"
get std::process::sleep

sleep(1000)"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
