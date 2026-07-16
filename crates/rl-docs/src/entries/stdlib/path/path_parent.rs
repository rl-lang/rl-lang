use crate::entry::FnEntry;

pub static PATH_PARENT: FnEntry = FnEntry {
    signature: "path_parent(path)",
    description: "returns the parent directory of the path",
    example: r#"
get std::path::path_parent

path_parent("/usr/bin/rl")"#,
    expected_output: Some("/usr/bin"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
