use crate::docs::entry::FnEntry;

pub static TERM_ENABLE_MOUSE: FnEntry = FnEntry {
    signature: "term_enable_mouse()",
    description: "enables capturing of mouse events",
    example: "get std::term::term_enable_mouse\n\nterm_enable_mouse()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
