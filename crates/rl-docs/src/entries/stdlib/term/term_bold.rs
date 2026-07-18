use crate::entry::FnEntry;

pub static TERM_BOLD: FnEntry = FnEntry {
    signature: "term_bold()",
    description: "enables bold text styling",
    example: r#"get std::term::term_bold

term_bold()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
