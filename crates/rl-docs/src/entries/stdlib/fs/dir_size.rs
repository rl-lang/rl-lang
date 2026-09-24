use crate::entry::FnEntry;

pub static DIR_SIZE: FnEntry = FnEntry {
    signature: "dir_size(path)",
    description: "returns the total size in bytes of all files in the directory recursively",
    example: r#"get std::fs::dir_size

dir_size("src")?"#,
    expected_output: Some("ok(123456)"),
    returns: "result[int]",
    errors: Some(
        r#"Will return errors on the following:

- `path` does not exist or is not a directory
- the current process does not have permission to read files"#,
    ),
    see_also: &["file_size", "list_dir", "walk_dir"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
