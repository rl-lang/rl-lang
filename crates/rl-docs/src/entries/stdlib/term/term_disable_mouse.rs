use crate::entry::FnEntry;

pub static TERM_DISABLE_MOUSE: FnEntry = FnEntry {
    signature: "term_disable_mouse()",
    description: "disables capturing of mouse events",
    example: r#"get std::term::term_disable_mouse

term_disable_mouse()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_enable_mouse"],
    since: Some("v0.1.5"),
};
