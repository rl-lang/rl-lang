use crate::entry::FnEntry;

pub static TERM_LEAVE: FnEntry = FnEntry {
    signature: "term_leave()",
    description: "leaves the alternate screen buffer and disables raw mode",
    example: r#"get std::term::term_leave

term_leave()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- disabling raw mode fails
- leaving the alternate screen buffer fails"#,
    ),
    see_also: &["term_enter"],
    since: Some("v0.1.5"),
};
