use crate::docs::entry::FnEntry;

pub static TERM_MOVE_TO_ROW: FnEntry = FnEntry {
    signature: "term_move_to_row(row)",
    description: "moves the cursor to an absolute row, keeping the current column",
    example: "get std::term::term_move_to_row\n\nterm_move_to_row(0)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
