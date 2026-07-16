use crate::entry::FnEntry;

pub static PATH_IS_FILE: FnEntry = FnEntry {
    signature: "path_is_file(path)",
    description: "returns true if the path is a file",
    example: r#"
get std::path::path_is_file

path_is_file("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
