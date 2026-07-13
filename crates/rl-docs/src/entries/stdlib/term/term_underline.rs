use crate::entry::FnEntry;

pub static TERM_UNDERLINE: FnEntry = FnEntry {
    signature: "term_underline()",
    description: "enables underlined text styling",
    example: "get std::term::term_underline\n\nterm_underline()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
