use crate::entry::FnEntry;

pub static TERM_RESET_ATTR: FnEntry = FnEntry {
    signature: "term_reset_attr()",
    description: "resets all text styling attributes to their defaults",
    example: r#"get std::term::term_reset_attr

term_reset_attr()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &[
        "term_bold",
        "term_dim",
        "term_italic",
        "term_underline",
        "term_blink",
        "term_reverse",
        "term_crossed_out",
    ],
    since: Some("v0.1.5"),
};
