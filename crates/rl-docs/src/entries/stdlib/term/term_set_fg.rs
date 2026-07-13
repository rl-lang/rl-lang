use crate::entry::FnEntry;

pub static TERM_SET_FG: FnEntry = FnEntry {
    signature: "term_set_fg(r, g, b)",
    description: "sets the foreground text color using an RGB value",
    example: "get std::term::term_set_fg\n\nterm_set_fg(255, 0, 0)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
