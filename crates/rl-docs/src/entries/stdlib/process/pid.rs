use crate::entry::FnEntry;

pub static PID: FnEntry = FnEntry {
    signature: "pid()",
    description: "returns the process ID of the current process",
    example: r#"
get std::process::pid

pid()"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
