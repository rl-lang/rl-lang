use crate::entry::FnEntry;

pub static TERM_ENTER: FnEntry = FnEntry {
    signature: "term_enter()",
    description: "enters raw mode and switches to the terminal's alternate screen buffer",
    example: r#"get std::term::term_enter

term_enter()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- enabling raw mode fails
- switching to the alternate screen buffer fails"#,
    ),
    see_also: &["term_leave"],
    since: Some("v0.1.5"),
};
