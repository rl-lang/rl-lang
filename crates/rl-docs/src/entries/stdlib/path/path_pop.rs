use crate::entry::FnEntry;

pub static PATH_POP: FnEntry = FnEntry {
    signature: "path_pop(path)",
    description: "removes the last component of the path and returns the result",
    example: r#"
get std::path::path_pop

path_pop("/usr/bin/rl")"#,
    expected_output: Some("/usr/bin"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
