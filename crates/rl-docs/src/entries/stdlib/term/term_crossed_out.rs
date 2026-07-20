use crate::entry::FnEntry;

pub static TERM_CROSSED_OUT: FnEntry = FnEntry {
    signature: "term_crossed_out()",
    description: "enables strikethrough text styling",
    example: r#"get std::term::term_crossed_out

term_crossed_out()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
