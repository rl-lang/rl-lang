use crate::entry::FnEntry;

pub static TERM_HIDE_CURSOR: FnEntry = FnEntry {
    signature: "term_hide_cursor()",
    description: "hides the terminal cursor",
    example: "get std::term::term_hide_cursor\n\nterm_hide_cursor()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
