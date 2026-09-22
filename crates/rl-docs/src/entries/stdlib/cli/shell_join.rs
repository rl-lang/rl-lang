use crate::entry::FnEntry;

pub static SHELL_JOIN: FnEntry = FnEntry {
    signature: "shell_join(parts)",
    description: "joins an array of strings into a shell command line, quoting parts that contain spaces or special characters",
    example: r#"get shell_join from std::cli

dec string cmd = shell_join(["rl", "run", "my file.rl"])"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["shell_split"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
