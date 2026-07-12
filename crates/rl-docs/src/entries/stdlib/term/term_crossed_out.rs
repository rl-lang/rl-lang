use crate::docs::entry::FnEntry;

pub static TERM_CROSSED_OUT: FnEntry = FnEntry {
    signature: "term_crossed_out()",
    description: "enables strikethrough text styling",
    example: "get std::term::term_crossed_out\n\nterm_crossed_out()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
