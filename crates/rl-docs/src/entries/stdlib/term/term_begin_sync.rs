use crate::entry::FnEntry;

pub static TERM_BEGIN_SYNC: FnEntry = FnEntry {
    signature: "term_begin_sync()",
    description: "begins a synchronized output update, batching subsequent writes",
    example: r#"get std::term::term_begin_sync

term_begin_sync()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_end_sync"],
    since: Some("v0.1.5"),
};
