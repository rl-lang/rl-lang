use crate::entry::FnEntry;

pub static TERM_RESTORE_CURSOR: FnEntry = FnEntry {
    signature: "term_restore_cursor()",
    description: "restores the cursor to the last saved position",
    example: r#"get std::term::term_restore_cursor

term_restore_cursor()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_save_cursor"],
    since: Some("v0.1.5"),
};
