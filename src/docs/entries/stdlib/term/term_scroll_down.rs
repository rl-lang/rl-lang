use crate::docs::entry::FnEntry;

pub static TERM_SCROLL_DOWN: FnEntry = FnEntry {
    signature: "term_scroll_down(n)",
    description: "scrolls the terminal content down by n lines",
    example: "get std::term::term_scroll_down\n\nterm_scroll_down(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
