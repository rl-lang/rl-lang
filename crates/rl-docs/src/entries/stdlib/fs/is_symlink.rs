use crate::entry::FnEntry;

pub static IS_SYMLINK: FnEntry = FnEntry {
    signature: "is_symlink(path)",
    description: "returns true if path is a symbolic link",
    example: r#"get std::fs::is_symlink

is_symlink("/tmp/link")"#,
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["symlink", "readlink", "realpath"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
