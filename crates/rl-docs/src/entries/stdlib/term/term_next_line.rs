use crate::entry::FnEntry;

pub static TERM_NEXT_LINE: FnEntry = FnEntry {
    signature: "term_next_line(n)",
    description: "moves the cursor to the beginning of the line n rows down",
    example: r#"get std::term::term_next_line

term_next_line(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_prev_line"],
    since: Some("v0.1.5"),
};
