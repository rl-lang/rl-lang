use crate::docs::entry::FnEntry;

pub static TERM_BLINK: FnEntry = FnEntry {
    signature: "term_blink()",
    description: "enables blinking text styling",
    example: "get std::term::term_blink\n\nterm_blink()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
