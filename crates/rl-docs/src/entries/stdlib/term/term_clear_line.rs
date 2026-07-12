use crate::entry::FnEntry;

pub static TERM_CLEAR_LINE: FnEntry = FnEntry {
    signature: "term_clear_line()",
    description: "clears the current line",
    example: "get std::term::term_clear_line\n\nterm_clear_line()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
