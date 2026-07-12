use crate::docs::entry::FnEntry;

pub static TERM_REVERSE: FnEntry = FnEntry {
    signature: "term_reverse()",
    description: "swaps the foreground and background colors",
    example: "get std::term::term_reverse\n\nterm_reverse()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
