use crate::docs::entry::FnEntry;

pub static TERM_MOVE_TO_COL: FnEntry = FnEntry {
    signature: "term_move_to_col(col)",
    description: "moves the cursor to an absolute column, keeping the current row",
    example: "get std::term::term_move_to_col\n\nterm_move_to_col(0)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
