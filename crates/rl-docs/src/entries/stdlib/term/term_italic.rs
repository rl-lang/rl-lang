use crate::entry::FnEntry;

pub static TERM_ITALIC: FnEntry = FnEntry {
    signature: "term_italic()",
    description: "enables italic text styling",
    example: r#"get std::term::term_italic

term_italic()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
