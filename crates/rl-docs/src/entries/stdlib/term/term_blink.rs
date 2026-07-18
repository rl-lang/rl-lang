use crate::entry::FnEntry;

pub static TERM_BLINK: FnEntry = FnEntry {
    signature: "term_blink()",
    description: "enables blinking text styling",
    example: r#"get std::term::term_blink

term_blink()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
