use crate::entry::FnEntry;

pub static PATH_JOIN: FnEntry = FnEntry {
    signature: "path_join(path, other)",
    description: "joins two paths together",
    example: r#"
get std::path::path_join

path_join("src", "main.rl")"#,
    expected_output: Some("src/main.rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
