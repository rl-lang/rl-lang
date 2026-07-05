use crate::docs::entry::FnEntry;

pub static TERM_FLUSH: FnEntry = FnEntry {
    signature: "term_flush()",
    description: "flushes any buffered terminal output",
    example: "get std::term::term_flush\n\nterm_flush()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
