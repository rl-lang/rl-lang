use crate::entry::FnEntry;

pub static TERM_SAVE_CURSOR: FnEntry = FnEntry {
    signature: "term_save_cursor()",
    description: "saves the current cursor position",
    example: r#"get std::term::term_save_cursor

term_save_cursor()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_restore_cursor"],
    since: Some("v0.1.5"),
};
