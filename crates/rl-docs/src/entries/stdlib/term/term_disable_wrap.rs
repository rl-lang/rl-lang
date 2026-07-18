use crate::entry::FnEntry;

pub static TERM_DISABLE_WRAP: FnEntry = FnEntry {
    signature: "term_disable_wrap()",
    description: "disables automatic line wrapping",
    example: r#"get std::term::term_disable_wrap

term_disable_wrap()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_enable_wrap"],
    since: Some("v0.1.5"),
};
