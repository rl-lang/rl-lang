use crate::docs::entry::FnEntry;

pub static TERM_MOVE: FnEntry = FnEntry {
    signature: "term_move(x, y)",
    description: "moves the cursor to an absolute column and row",
    example: "get std::term::term_move\n\nterm_move(10, 5)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
