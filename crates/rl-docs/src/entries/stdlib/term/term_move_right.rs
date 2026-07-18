use crate::entry::FnEntry;

pub static TERM_MOVE_RIGHT: FnEntry = FnEntry {
    signature: "term_move_right(n)",
    description: "moves the cursor right n columns",
    example: r#"get std::term::term_move_right

term_move_right(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_left"],
    since: Some("v0.1.5"),
};
