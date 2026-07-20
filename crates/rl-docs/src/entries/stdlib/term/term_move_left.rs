use crate::entry::FnEntry;

pub static TERM_MOVE_LEFT: FnEntry = FnEntry {
    signature: "term_move_left(n)",
    description: "moves the cursor left n columns",
    example: r#"get std::term::term_move_left

term_move_left(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `n` is not an int
- `n` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_move_right"],
    since: Some("v0.1.5"),
};
