use crate::entry::FnEntry;

pub static TERM_HIDE_CURSOR: FnEntry = FnEntry {
    signature: "term_hide_cursor()",
    description: "hides the terminal cursor",
    example: r#"get std::term::term_hide_cursor

term_hide_cursor()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_show_cursor"],
    since: Some("v0.1.5"),
};
