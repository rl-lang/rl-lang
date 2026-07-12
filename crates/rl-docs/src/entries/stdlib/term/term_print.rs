use crate::entry::FnEntry;

pub static TERM_PRINT: FnEntry = FnEntry {
    signature: "term_print(text)",
    description: "writes text directly to the terminal without a trailing newline",
    example: "get std::term::term_print\n\nterm_print(\"hello\")",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
