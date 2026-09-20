use crate::entry::FnEntry;

pub static EXEC_FG: FnEntry = FnEntry {
    signature: "exec_fg(cmd)",
    description: "runs a shell command in the foreground with inherited stdin/stdout/stderr, returning its exit code. Unlike exec (which pipes stdout), exec_fg lets interactive programs (editors, TUIs, pagers) access the terminal directly. Call term_leave() before and term_enter() after to manage terminal state.",
    example: r#"get std::process::exec_fg
get std::term::term_leave, term_enter

term_leave()
dec int code = exec_fg("hx notes.md")?
term_enter()"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error on failed command run"),
    see_also: &["exec", "exec_code", "with_exec_fg"],
    since: Some("v0.4.0"),
    deprecated: None,
    updated: Some("v0.4.0"),
};
