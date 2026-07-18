use crate::entry::FnEntry;

pub static TERM_GET_SIZE: FnEntry = FnEntry {
    signature: "term_get_size()",
    description: "returns the terminal size as (columns, rows)",
    example: r#"get std::term::term_get_size

term_get_size()?"#,
    expected_output: None,
    returns: "result[array[int]]",
    errors: Some(r#"Will return error if retrieving the terminal size fails"#),
    see_also: &["term_set_size"],
    since: Some("v0.1.5"),
};
