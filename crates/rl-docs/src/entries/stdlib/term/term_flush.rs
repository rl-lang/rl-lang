use crate::entry::FnEntry;

pub static TERM_FLUSH: FnEntry = FnEntry {
    signature: "term_flush()",
    description: "flushes any buffered terminal output",
    example: r#"get std::term::term_flush

term_flush()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if flushing stdout fails"#),
    see_also: &["term_print"],
    since: Some("v0.1.5"),
};
