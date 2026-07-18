use crate::entry::FnEntry;

pub static TERM_MOVE_TO_COL: FnEntry = FnEntry {
    signature: "term_move_to_col(col)",
    description: "moves the cursor to an absolute column, keeping the current row",
    example: r#"get std::term::term_move_to_col

term_move_to_col(0)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `col` is not an int
- `col` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_to_row", "term_move"],
    since: Some("v0.1.5"),
};
