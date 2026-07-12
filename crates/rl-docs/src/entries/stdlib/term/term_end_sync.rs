use crate::docs::entry::FnEntry;

pub static TERM_END_SYNC: FnEntry = FnEntry {
    signature: "term_end_sync()",
    description: "ends a synchronized output update, flushing batched writes",
    example: "get std::term::term_end_sync\n\nterm_end_sync()",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
