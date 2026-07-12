use crate::docs::entry::FnEntry;

pub static TERM_READ_KEY: FnEntry = FnEntry {
    signature: "term_read_key()",
    description: "blocks until a key, mouse, or resize event occurs and returns it",
    example: "get std::term::term_read_key\n\nterm_read_key() // \"Char:a\"",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
