use crate::entry::FnEntry;

pub static TERM_ENABLE_WRAP: FnEntry = FnEntry {
    signature: "term_enable_wrap()",
    description: "enables automatic line wrapping",
    example: "get std::term::term_enable_wrap\n\nterm_enable_wrap()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
