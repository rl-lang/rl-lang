use crate::entry::FnEntry;

pub static TERM_PREV_LINE: FnEntry = FnEntry {
    signature: "term_prev_line(n)",
    description: "moves the cursor to the beginning of the line n rows up",
    example: r#"get std::term::term_prev_line

term_prev_line(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_next_line"],
    since: Some("v0.1.5"),
};
