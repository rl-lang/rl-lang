use crate::entry::FnEntry;

pub static TERM_SET_FG: FnEntry = FnEntry {
    signature: "term_set_fg(r, g, b)",
    description: "sets the foreground text color using an RGB value",
    example: r#"get std::term::term_set_fg

term_set_fg(255, 0, 0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `r`, `g`, or `b` is not a number
- writing to stdout fails"#,
    ),
    see_also: &["term_set_bg", "term_fg", "term_reset_color"],
    since: Some("v0.1.5"),
};
