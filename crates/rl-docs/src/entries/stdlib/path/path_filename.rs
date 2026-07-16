use crate::entry::FnEntry;

pub static PATH_FILENAME: FnEntry = FnEntry {
    signature: "path_filename(path)",
    description: "returns the final component of the path",
    example: r#"
get std::path::path_filename

path_filename("/usr/bin/rl")"#,
    expected_output: Some("rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
