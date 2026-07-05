use crate::docs::entry::FnEntry;

pub static TERM_CLEAR: FnEntry = FnEntry {
    signature: "term_clear()",
    description: "clears the entire terminal screen",
    example: "get std::term::term_clear\n\nterm_clear()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
