use crate::entry::FnEntry;

pub static TERM_RESET_ATTR: FnEntry = FnEntry {
    signature: "term_reset_attr()",
    description: "resets all text styling attributes to their defaults",
    example: "get std::term::term_reset_attr\n\nterm_reset_attr()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
