use crate::entry::FnEntry;

pub static PATH_EXISTS: FnEntry = FnEntry {
    signature: "path_exists(path)",
    description: "returns true if the path exists on the filesystem",
    example: r#"
get std::path::path_exists

path_exists("./Cargo.toml")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
