use crate::entry::FnEntry;

pub static TERM_SET_TITLE: FnEntry = FnEntry {
    signature: "term_set_title(title)",
    description: "sets the terminal window title",
    example: "get std::term::term_set_title\n\nterm_set_title(\"my program\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
