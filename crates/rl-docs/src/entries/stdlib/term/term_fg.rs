use crate::entry::FnEntry;

pub static TERM_FG: FnEntry = FnEntry {
    signature: "term_fg(name)",
    description: "sets the foreground text color using a named color",
    example: "get std::term::term_fg\n\nterm_fg(\"red\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
