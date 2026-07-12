use crate::entry::FnEntry;

pub static TERM_SCROLL_UP: FnEntry = FnEntry {
    signature: "term_scroll_up(n)",
    description: "scrolls the terminal content up by n lines",
    example: "get std::term::term_scroll_up\n\nterm_scroll_up(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
