use crate::entry::FnEntry;

pub static TERM_RESET_COLOR: FnEntry = FnEntry {
    signature: "term_reset_color()",
    description: "resets the foreground and background colors to their defaults",
    example: r#"get std::term::term_reset_color

term_reset_color()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_set_fg", "term_set_bg", "term_reset_attr"],
    since: Some("v0.1.5"),
};
