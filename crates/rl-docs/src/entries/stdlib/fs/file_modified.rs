use crate::entry::FnEntry;

pub static FILE_MODIFIED: FnEntry = FnEntry {
    signature: "file_modified(path)",
    description: "returns the last modification time of the file as a unix timestamp (seconds since epoch)",
    example: r#"
get std::fs::file_modified

file_modified("./Cargo.toml")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        r#"
Will return errors on the following:

- user lacks permission to read file at `path`
- `path` does not exist
        "#,
    ),
    see_also: &["file_size"],
    since: Some("v0.1.5"),
};
