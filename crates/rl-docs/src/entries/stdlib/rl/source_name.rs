use crate::entry::FnEntry;

pub static SOURCE_NAME: FnEntry = FnEntry {
    signature: "source_name()",
    description: "returns the name of the file currently being run, or null if there is no source file (e.g. running in the REPL)",
    example: "get std::rl::source_name\n\nsource_name() // \"main.rl\"",
    expected_output: None,
    returns: "string | null",
    errors: None,
    see_also: &[],
    since: None,
};
