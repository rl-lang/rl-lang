use crate::entry::FnEntry;

pub static TERM_ENABLE_WRAP: FnEntry = FnEntry {
    signature: "term_enable_wrap()",
    description: "enables automatic line wrapping",
    example: r#"get std::term::term_enable_wrap

term_enable_wrap()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_disable_wrap"],
    since: Some("v0.1.5"),
};
