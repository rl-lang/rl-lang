use crate::entry::FnEntry;

pub static TERM_SET_SIZE: FnEntry = FnEntry {
    signature: "term_set_size(cols, rows)",
    description: "sets the terminal size in columns and rows",
    example: r#"get std::term::term_set_size

term_set_size(80, 24)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `cols` or `rows` is not an int
- `cols` or `rows` is negative
- writing to stdout fails"#,
    ),
    see_also: &["term_get_size"],
    since: Some("v0.1.5"),
};
