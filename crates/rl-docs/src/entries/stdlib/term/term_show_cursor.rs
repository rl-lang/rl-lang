use crate::entry::FnEntry;

pub static TERM_SHOW_CURSOR: FnEntry = FnEntry {
    signature: "term_show_cursor()",
    description: "shows the terminal cursor",
    example: r#"get std::term::term_show_cursor

term_show_cursor()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_hide_cursor"],
    since: Some("v0.1.5"),
};
