use crate::entry::FnEntry;

pub static TERM_FG: FnEntry = FnEntry {
    signature: "term_fg(name)",
    description: "sets the foreground text color using a named color",
    example: r#"get std::term::term_fg

term_fg("red")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `name` is not a string
- `name` is not a recognized color name
- writing to stdout fails"#,
    ),
    see_also: &["term_bg", "term_set_fg"],
    since: Some("v0.1.5"),
};
