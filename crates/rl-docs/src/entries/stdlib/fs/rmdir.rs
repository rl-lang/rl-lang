use crate::entry::FnEntry;

pub static RMDIR: FnEntry = FnEntry {
    signature: "rmdir(path)",
    description: "removes an empty directory, fails if it is not empty",
    example: r#"
get std::fs::rmdir

rmdir("./build")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
Will return error on the following:

- `path` does not exist
- `path` is not directory
- user lacks permission to remove the directory at provided `path`
- directory is not empty"#,
    ),
    see_also: &["rmdir_all", "mkdir"],
    since: Some("v0.1.5"),
};
