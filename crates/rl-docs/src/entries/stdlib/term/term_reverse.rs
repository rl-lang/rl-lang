use crate::entry::FnEntry;

pub static TERM_REVERSE: FnEntry = FnEntry {
    signature: "term_reverse()",
    description: "swaps the foreground and background colors",
    example: r#"get std::term::term_reverse

term_reverse()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
