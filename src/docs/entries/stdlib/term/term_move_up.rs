use crate::docs::entry::FnEntry;

pub static TERM_MOVE_UP: FnEntry = FnEntry {
    signature: "term_move_up(n)",
    description: "moves the cursor up n rows",
    example: "get std::term::term_move_up\n\nterm_move_up(1)",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
