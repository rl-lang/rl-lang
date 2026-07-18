use crate::entry::FnEntry;

pub static TERM_SCROLL_DOWN: FnEntry = FnEntry {
    signature: "term_scroll_down(n)",
    description: "scrolls the terminal content down by n lines",
    example: r#"get std::term::term_scroll_down

term_scroll_down(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_scroll_up"],
    since: Some("v0.1.5"),
};
