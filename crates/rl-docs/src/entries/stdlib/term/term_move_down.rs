use crate::entry::FnEntry;

pub static TERM_MOVE_DOWN: FnEntry = FnEntry {
    signature: "term_move_down(n)",
    description: "moves the cursor down n rows",
    example: "get std::term::term_move_down\n\nterm_move_down(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
