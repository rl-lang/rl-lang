use crate::entry::FnEntry;

pub static TERM_MOVE_RIGHT: FnEntry = FnEntry {
    signature: "term_move_right(n)",
    description: "moves the cursor right n columns",
    example: "get std::term::term_move_right\n\nterm_move_right(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
