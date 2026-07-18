use crate::entry::FnEntry;

pub static TERM_DIM: FnEntry = FnEntry {
    signature: "term_dim()",
    description: "enables dim text styling",
    example: r#"get std::term::term_dim

term_dim()?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(r#"Will return error if writing to stdout fails"#),
    see_also: &["term_reset_attr"],
    since: Some("v0.1.5"),
};
