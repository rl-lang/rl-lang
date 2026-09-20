use crate::entry::FnEntry;

pub static WITH_EXEC_FG: FnEntry = FnEntry {
    signature: "with_exec_fg(exe, cmd)",
    description: "runs an executable directly (without the platform shell) in the foreground with inherited stdin/stdout/stderr, splitting cmd into arguments. Returns the exit code. Useful for interactive programs that need terminal access.",
    example: r#"get std::process::with_exec_fg
get std::term::term_leave, term_enter

term_leave()
dec int code = with_exec_fg("hx", "notes.md")?
term_enter()"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on invalid arguments or failed command run"),
    see_also: &["with_exec", "exec_fg", "with_exec_code"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
