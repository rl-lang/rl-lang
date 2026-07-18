use crate::entry::FnEntry;

pub static PATH_IS_DIR: FnEntry = FnEntry {
    signature: "path_is_dir(path)",
    description: "returns true if the path is a directory",
    example: r#"get std::path::path_is_dir

path_is_dir("./src")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
