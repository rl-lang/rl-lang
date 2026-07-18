use crate::entry::FnEntry;

pub static TERM_MOVE_DOWN: FnEntry = FnEntry {
    signature: "term_move_down(n)",
    description: "moves the cursor down n rows",
    example: r#"get std::term::term_move_down

term_move_down(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_up"],
    since: Some("v0.1.5"),
};
