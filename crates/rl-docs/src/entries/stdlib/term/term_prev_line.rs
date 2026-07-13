use crate::entry::FnEntry;

pub static TERM_PREV_LINE: FnEntry = FnEntry {
    signature: "term_prev_line(n)",
    description: "moves the cursor to the beginning of the line n rows up",
    example: "get std::term::term_prev_line\n\nterm_prev_line(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
