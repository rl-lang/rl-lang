use crate::entry::FnEntry;

pub static TERM_GET_SIZE: FnEntry = FnEntry {
    signature: "term_get_size()",
    description: "returns the terminal size as (columns, rows)",
    example: "get std::term::term_get_size\n\nterm_get_size() // (80, 24)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
