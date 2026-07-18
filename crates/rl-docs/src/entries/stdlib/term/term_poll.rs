use crate::entry::FnEntry;

pub static TERM_POLL: FnEntry = FnEntry {
    signature: "term_poll(ms)",
    description: "returns true if an input event becomes available within ms milliseconds",
    example: r#"get std::term::term_poll

term_poll(100)?"#,
    expected_output: Some("false"),
    returns: "result[bool]",
    errors: Some(
        r#"Will return error on the following:

- `ms` is not a number
- polling for input fails"#,
    ),
    see_also: &["term_read_key"],
    since: Some("v0.1.5"),
};
