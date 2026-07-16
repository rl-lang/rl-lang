use crate::entry::FnEntry;

pub static PATH_SET_EXTENSION: FnEntry = FnEntry {
    signature: "path_set_extension(path, extension)",
    description: "sets or replaces the extension of the path and returns the result",
    example: r#"
get std::path::path_set_extension

path_set_extension("main.rl", "txt")"#,
    expected_output: Some("main.txt"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
