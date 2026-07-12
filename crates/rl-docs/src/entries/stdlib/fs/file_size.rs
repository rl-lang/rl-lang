use crate::entry::FnEntry;

pub static FILE_SIZE: FnEntry = FnEntry {
    signature: "file_size(path)",
    description: "returns the size of the file in bytes",
    example: r#"
get std::fs::file_size

file_size("./Cargo.toml")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        r#"
Will return errors on the following:

- user lacks permission to read file at `path`
- `path` does not exist
        "#,
    ),
    see_also: &["file_modified"],
    since: Some("v0.1.5"),
};
