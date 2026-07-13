use crate::entry::FnEntry;

pub static TERM_RESET_COLOR: FnEntry = FnEntry {
    signature: "term_reset_color()",
    description: "resets the foreground and background colors to their defaults",
    example: "get std::term::term_reset_color\n\nterm_reset_color()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
