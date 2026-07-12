use crate::docs::entry::FnEntry;

pub static COPY_FILE: FnEntry = FnEntry {
    signature: "copy_file(src, dst)",
    description: "copies a file from src to dst, returns the number of bytes copied",
    example: r#"
get std::fs::copy_file

copy_file("a.txt", "b.txt")?"#,
    expected_output: None,
    returns: "result[int]",
    errors: Some(
        r#"
Will return errors on the following:

- `src` does not exist
- `src` not regular file nor symlink to regular file
- the current process does not have permission rights to read from `src` and write to `dst`
- the parent directory of `dst` does not exist"#,
    ),
    see_also: &["move_file"],
    since: Some("v0.1.5"),
};
