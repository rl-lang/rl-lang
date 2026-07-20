use crate::entry::FnEntry;

pub static TERM_SET_TITLE: FnEntry = FnEntry {
    signature: "term_set_title(title)",
    description: "sets the terminal window title",
    example: r#"get std::term::term_set_title

term_set_title(1)?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"Will return error on the following:

- `title` is not a number (the current implementation extracts a byte, not
  a string, despite the parameter name - passing a string always errors)
- writing to stdout fails"#,
    ),
    see_also: &[],
    since: Some("v0.1.5"),
};
