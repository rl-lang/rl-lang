use crate::entry::FnEntry;

pub static TERM_SHOW_CURSOR: FnEntry = FnEntry {
    signature: "term_show_cursor()",
    description: "shows the terminal cursor",
    example: "get std::term::term_show_cursor\n\nterm_show_cursor()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
