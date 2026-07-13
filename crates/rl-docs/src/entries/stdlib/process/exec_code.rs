use crate::entry::FnEntry;

pub static EXEC_CODE: FnEntry = FnEntry {
    signature: "exec_code(cmd)",
    description: "runs a shell command and returns its exit code as an int",
    example: r#"
get std::process::exec_code

dec int code = exec_code("ls /nonexistent")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on faailed command run"),
    see_also: &["exec", "exec_lines"],
    since: Some("v0.1.5"),
};
