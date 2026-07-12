use crate::entry::FnEntry;

pub static TERM_ITALIC: FnEntry = FnEntry {
    signature: "term_italic()",
    description: "enables italic text styling",
    example: "get std::term::term_italic\n\nterm_italic()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
