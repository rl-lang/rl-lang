use crate::entry::FnEntry;

pub static TERM_MOVE: FnEntry = FnEntry {
    signature: "term_move(x, y)",
    description: "moves the cursor to an absolute column and row",
    example: r#"get std::term::term_move

term_move(10, 5)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `x` or `y` is not an int
- `x` or `y` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_to_col", "term_move_to_row"],
    since: Some("v0.1.5"),
};
