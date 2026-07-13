use crate::entry::FnEntry;

pub static EXIT: FnEntry = FnEntry {
    signature: "exit(code)",
    description: "terminates the process with the given exit code",
    example: r#"
get std::process::exit

exit(0)"#,
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
