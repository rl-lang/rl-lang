use crate::entry::FnEntry;

pub static TERM_PRINT: FnEntry = FnEntry {
    signature: "term_print(text)",
    description: "writes text directly to the terminal without a trailing newline",
    example: r#"get std::term::term_print

term_print("hello")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_flush"],
    since: Some("v0.1.5"),
};
