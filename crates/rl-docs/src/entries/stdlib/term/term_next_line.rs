use crate::entry::FnEntry;

pub static TERM_NEXT_LINE: FnEntry = FnEntry {
    signature: "term_next_line(n)",
    description: "moves the cursor to the beginning of the line n rows down",
    example: "get std::term::term_next_line\n\nterm_next_line(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
