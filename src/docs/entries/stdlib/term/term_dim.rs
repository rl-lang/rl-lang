use crate::docs::entry::FnEntry;

pub static TERM_DIM: FnEntry = FnEntry {
    signature: "term_dim()",
    description: "enables dim text styling",
    example: "get std::term::term_dim\n\nterm_dim()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
