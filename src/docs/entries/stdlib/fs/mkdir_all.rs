use crate::docs::entry::FnEntry;

pub static MKDIR_ALL: FnEntry = FnEntry {
    signature: "mkdir_all(path)",
    description: "creates a directory along with any missing parent directories",
    example: r#"
get std::fs::mkdir_all

mkdir_all("./build/assets/css")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
Will return errors on the following:

- user lacks permission to create directory at `path`
- `path` already exists"#,
    ),
    see_also: &["rmdir", "mkdir_all"],
    since: Some("v0.1.5"),
};
