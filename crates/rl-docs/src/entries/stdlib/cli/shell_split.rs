use crate::entry::FnEntry;

pub static SHELL_SPLIT: FnEntry = FnEntry {
    signature: "shell_split(s)",
    description: "splits a string into shell words, honoring single/double quotes and backslash escapes",
    example: r#"get shell_split from std::cli
get result_unwrap from std::res

dec parts = result_unwrap(shell_split("rl run demo.rl -- -o \\"my file.txt\\""))"#,
    expected_output: None,
    returns: "result[array[string]]",
    errors: Some("unterminated quote or escape"),
    see_also: &["shell_join"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
