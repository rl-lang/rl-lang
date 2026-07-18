use crate::entry::FnEntry;

pub static TERM_UNDERLINE: FnEntry = FnEntry {
    signature: "term_underline()",
    description: "enables underlined text styling",
    example: r#"get std::term::term_underline

term_underline()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
