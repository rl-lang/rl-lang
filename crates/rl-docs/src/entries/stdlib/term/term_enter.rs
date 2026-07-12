use crate::entry::FnEntry;

pub static TERM_ENTER: FnEntry = FnEntry {
    signature: "term_enter()",
    description: "enters raw mode and switches to the terminal's alternate screen buffer",
    example: "get std::term::term_enter\n\nterm_enter()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
