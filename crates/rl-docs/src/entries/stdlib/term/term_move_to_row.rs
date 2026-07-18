use crate::entry::FnEntry;

pub static TERM_MOVE_TO_ROW: FnEntry = FnEntry {
    signature: "term_move_to_row(row)",
    description: "moves the cursor to an absolute row, keeping the current column",
    example: r#"get std::term::term_move_to_row

term_move_to_row(0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `row` is not an int
- `row` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_to_col", "term_move"],
    since: Some("v0.1.5"),
};
