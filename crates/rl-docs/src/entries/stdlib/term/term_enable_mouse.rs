use crate::entry::FnEntry;

pub static TERM_ENABLE_MOUSE: FnEntry = FnEntry {
    signature: "term_enable_mouse()",
    description: "enables capturing of mouse events",
    example: r#"get std::term::term_enable_mouse

term_enable_mouse()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_disable_mouse"],
    since: Some("v0.1.5"),
};
