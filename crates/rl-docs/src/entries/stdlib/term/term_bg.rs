use crate::entry::FnEntry;

pub static TERM_BG: FnEntry = FnEntry {
    signature: "term_bg(name)",
    description: "sets the background color using a named color",
    example: r#"get std::term::term_bg

term_bg("blue")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `name` is not a string
- `name` is not a recognized color name
- writing to stdout fails"#,
    ),
    see_also: &["term_fg", "term_set_bg"],
    since: Some("v0.1.5"),
};
