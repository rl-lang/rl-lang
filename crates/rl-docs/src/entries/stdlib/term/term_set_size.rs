use crate::entry::FnEntry;

pub static TERM_SET_SIZE: FnEntry = FnEntry {
    signature: "term_set_size(cols, rows)",
    description: "sets the terminal size in columns and rows",
    example: "get std::term::term_set_size\n\nterm_set_size(80, 24)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
