use crate::entry::FnEntry;

pub static PATH_EXTENSION: FnEntry = FnEntry {
    signature: "path_extension(path)",
    description: "returns the file extension of the path",
    example: r#"
get std::path::path_extension

path_extension("main.rl")"#,
    expected_output: Some("rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
