use crate::entry::FnEntry;

pub static EXEC_LINES: FnEntry = FnEntry {
    signature: "exec_lines(cmd)",
    description: "runs a shell command and returns its stdout split into an array of lines",
    example: r#"
get std::process::exec_lines

dec arr[string] files = exec_lines("ls src")?"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec", "exec_code"],
    since: Some("v0.1.5"),
};
