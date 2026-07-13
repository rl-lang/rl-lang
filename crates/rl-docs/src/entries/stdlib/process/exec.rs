use crate::entry::FnEntry;

pub static EXEC: FnEntry = FnEntry {
    signature: "exec(cmd)",
    description: "runs a shell command and returns its stdout as a trimmed string",
    example: r#"
get std::process::exec

dec string out = exec("echo hello")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec_code", "exec_lines"],
    since: Some("v0.1.5"),
};
