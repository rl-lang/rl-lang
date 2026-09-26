use crate::entry::FnEntry;

pub static PATH_RELATIVE: FnEntry = FnEntry {
    signature: "path_relative(from, to)",
    description: "computes a relative path from the first path to the second path",
    example: r#"get std::fs::path_relative

dec string rel = path_relative("src/main.rs", "src/utils/helper.rs")?"#,
    expected_output: None,
    returns: "result[string]",
    errors: Some("Will return error if current working directory cannot be determined"),
    see_also: &["path_absolute", "path_join"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
