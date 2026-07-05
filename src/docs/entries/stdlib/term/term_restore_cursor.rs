use crate::docs::entry::FnEntry;

pub static TERM_RESTORE_CURSOR: FnEntry = FnEntry {
    signature: "term_restore_cursor()",
    description: "restores the cursor to the last saved position",
    example: "get std::term::term_restore_cursor\n\nterm_restore_cursor()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
