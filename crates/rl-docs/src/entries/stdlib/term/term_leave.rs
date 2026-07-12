use crate::entry::FnEntry;

pub static TERM_LEAVE: FnEntry = FnEntry {
    signature: "term_leave()",
    description: "leaves the alternate screen buffer and disables raw mode",
    example: "get std::term::term_leave\n\nterm_leave()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
