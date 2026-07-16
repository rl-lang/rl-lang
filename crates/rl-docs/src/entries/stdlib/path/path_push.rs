use crate::entry::FnEntry;

pub static PATH_PUSH: FnEntry = FnEntry {
    signature: "path_push(path, target)",
    description: "appends a component to the path and returns the result",
    example: r#"
get std::path::path_push

path_push("/usr/bin", "rl")"#,
    expected_output: Some("/usr/bin/rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
