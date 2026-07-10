use crate::docs::entry::FnEntry;

pub static LIST_DIR: FnEntry = FnEntry {
    signature: "list_dir(path)",
    description: "returns an array of paths for the entries in the directory",
    example: r#"
get std::fs::list_dir

list_dir("./src")"#,
    expected_output: None,
    returns: "result[arr[string]]",
    errors: Some(
        r#"
Will return errors on the following:

- `path` points to non directory
- `path` does not exist
- user has no permission to read `path` content
"#,
    ),
    see_also: &["mkdir", "rmdir"],
    since: Some("v0.1.5"),
};
