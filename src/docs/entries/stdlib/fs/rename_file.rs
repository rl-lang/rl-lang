use crate::docs::entry::FnEntry;

pub static RENAME_FILE: FnEntry = FnEntry {
    signature: "rename_file(path, new_name)",
    description: "renames a file, keeping it in its current directory, and returns the new path",
    example: r#"
get std::fs::rename_file

rename_file("/usr/bin/rl", "rl-old")"#,
    expected_output: Some("/usr/bin/rl-old"),
    returns: "result[string]",
    errors: Some(
        r#"
    Will return errors on the following:

    - `src` does not exist
    - user lacks permissions to view contents
    - `src` and `dst` are on separate filesystems
    "#,
    ),
    see_also: &["move_file", "rename_file"],
    since: Some("v0.1.5"),
};
