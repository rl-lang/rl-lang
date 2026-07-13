use crate::entry::FnEntry;

pub static RMDIR_ALL: FnEntry = FnEntry {
    signature: "rmdir_all(path)",
    description: "removes a directory and all of its contents recursively",
    example: r#"
get std::fs::rmdir_all

rmdir_all("./build")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
Will return error on the following:

- `path` does not exist
- `path` is not directory
- user lacks permission to remove the directory at provided `path`"#,
    ),
    see_also: &["rmdir", "mkdir"],
    since: Some("v0.1.5"),
};
