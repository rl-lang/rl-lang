use crate::entry::FnEntry;

pub static TERM_CLEAR_LINE: FnEntry = FnEntry {
    signature: "term_clear_line()",
    description: "clears the current line",
    example: r#"get std::term::term_clear_line

term_clear_line()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing the clear sequence to stdout fails"#),
    see_also: &["term_clear"],
    since: Some("v0.1.5"),
};
