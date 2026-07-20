use crate::entry::FnEntry;

pub static PATH_STEM: FnEntry = FnEntry {
    signature: "path_stem(path)",
    description: "returns the filename without its extension",
    example: r#"get std::path::path_stem

path_stem("main.rl.txt")"#,
    expected_output: Some("main.rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
