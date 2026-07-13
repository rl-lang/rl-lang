use crate::entry::FnEntry;

pub static TERM_POLL: FnEntry = FnEntry {
    signature: "term_poll(ms)",
    description: "returns true if an input event becomes available within ms milliseconds",
    example: "get std::term::term_poll\n\nterm_poll(100) // false",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
