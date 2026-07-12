use crate::docs::entry::FnEntry;

pub static TERM_SAVE_CURSOR: FnEntry = FnEntry {
    signature: "term_save_cursor()",
    description: "saves the current cursor position",
    example: "get std::term::term_save_cursor\n\nterm_save_cursor()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
