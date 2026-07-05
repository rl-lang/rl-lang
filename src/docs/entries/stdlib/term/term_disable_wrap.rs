use crate::docs::entry::FnEntry;

pub static TERM_DISABLE_WRAP: FnEntry = FnEntry {
    signature: "term_disable_wrap()",
    description: "disables automatic line wrapping",
    example: "get std::term::term_disable_wrap\n\nterm_disable_wrap()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
