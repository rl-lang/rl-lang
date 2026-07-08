use crate::docs::entry::FnEntry;

pub static MOVE_FILE: FnEntry = FnEntry {
    signature: "move_file(src, dst)",
    description: "moves a file from src to dst",
    example: r#"
get std::fs::move_file

move_file("/tmp/a.txt", "/tmp/b.txt")?"#,
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        r#"
Will return errors on the following:

- `src` does not exist
- user lacks permissions to view contents
- `src` and `dst` are on separate filesystems
"#,
    ),
    see_also: &["copy_file"],
    since: Some("v0.1.5"),
};
