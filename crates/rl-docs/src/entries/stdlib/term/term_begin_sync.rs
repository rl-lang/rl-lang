use crate::docs::entry::FnEntry;

pub static TERM_BEGIN_SYNC: FnEntry = FnEntry {
    signature: "term_begin_sync()",
    description: "begins a synchronized output update, batching subsequent writes",
    example: "get std::term::term_begin_sync\n\nterm_begin_sync()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
