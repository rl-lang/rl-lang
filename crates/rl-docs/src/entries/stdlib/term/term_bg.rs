use crate::docs::entry::FnEntry;

pub static TERM_BG: FnEntry = FnEntry {
    signature: "term_bg(name)",
    description: "sets the background color using a named color",
    example: "get std::term::term_bg\n\nterm_bg(\"blue\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
