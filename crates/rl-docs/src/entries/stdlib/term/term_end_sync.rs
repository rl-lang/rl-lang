use crate::entry::FnEntry;

pub static TERM_END_SYNC: FnEntry = FnEntry {
    signature: "term_end_sync()",
    description: "ends a synchronized output update, flushing batched writes",
    example: r#"get std::term::term_end_sync

term_end_sync()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_begin_sync"],
    since: Some("v0.1.5"),
};
