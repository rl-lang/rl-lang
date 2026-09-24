use crate::entry::FnEntry;

pub static COPY_DIR: FnEntry = FnEntry {
    signature: "copy_dir(src, dest)",
    description: "recursively copies a directory from src to dest",
    example: r#"get std::fs::copy_dir

copy_dir("src", "backup")?"#,
    expected_output: Some("ok(\"done\")"),
    returns: "result[string]",
    errors: Some(
        r#"Will return errors on the following:

- `src` does not exist or is not a directory
- `dest` parent directory does not exist
- the current process does not have permission rights"#,
    ),
    see_also: &["copy_file", "walk_dir", "mkdir_all"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
