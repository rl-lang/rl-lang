use crate::docs::entry::FnEntry;

pub static TERM_SET_BG: FnEntry = FnEntry {
    signature: "term_set_bg(r, g, b)",
    description: "sets the background color using an RGB value",
    example: "get std::term::term_set_bg\n\nterm_set_bg(0, 0, 0)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
