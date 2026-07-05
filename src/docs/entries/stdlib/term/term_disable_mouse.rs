use crate::docs::entry::FnEntry;

pub static TERM_DISABLE_MOUSE: FnEntry = FnEntry {
    signature: "term_disable_mouse()",
    description: "disables capturing of mouse events",
    example: "get std::term::term_disable_mouse\n\nterm_disable_mouse()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
