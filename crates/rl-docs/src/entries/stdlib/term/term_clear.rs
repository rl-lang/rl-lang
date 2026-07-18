use crate::entry::FnEntry;

pub static TERM_CLEAR: FnEntry = FnEntry {
    signature: "term_clear()",
    description: "clears the entire terminal screen",
    example: r#"get std::term::term_clear

term_clear()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing the clear sequence to stdout fails"#),
    see_also: &["term_clear_line"],
    since: Some("v0.1.5"),
};
