use crate::docs::entry::FnEntry;

pub static MKDIR: FnEntry = FnEntry {
    signature: "mkdir(path)",
    description: "creates a directory, fails if the parent directory does not exist",
    example: r#"
get std::fs::mkdir

mkdir("./build")"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
Will return errors on the following:

- user lacks permission to create directory at `path`
- a parent of given `path` does not exist
- `path` already exists"#,
    ),
    see_also: &["rmdir", "mkdir_all"],
    since: Some("v0.1.5"),
};
