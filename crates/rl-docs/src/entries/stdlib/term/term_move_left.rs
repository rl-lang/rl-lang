use crate::entry::FnEntry;

pub static TERM_MOVE_LEFT: FnEntry = FnEntry {
    signature: "term_move_left(n)",
    description: "moves the cursor left n columns",
    example: "get std::term::term_move_left\n\nterm_move_left(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
