use crate::docs::entry::FnEntry;

pub static TERM_BOLD: FnEntry = FnEntry {
    signature: "term_bold()",
    description: "enables bold text styling",
    example: "get std::term::term_bold\n\nterm_bold()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
