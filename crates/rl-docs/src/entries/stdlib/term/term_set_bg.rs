use crate::entry::FnEntry;

pub static TERM_SET_BG: FnEntry = FnEntry {
    signature: "term_set_bg(r, g, b)",
    description: "sets the background color using an RGB value",
    example: r#"get std::term::term_set_bg

term_set_bg(0, 0, 0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `r`, `g`, or `b` is not a number
- writing to stdout fails"#,
    ),
    see_also: &["term_set_fg", "term_bg", "term_reset_color"],
    since: Some("v0.1.5"),
};
