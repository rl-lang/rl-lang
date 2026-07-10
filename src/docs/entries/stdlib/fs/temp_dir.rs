use crate::docs::entry::FnEntry;

pub static TEMP_DIR: FnEntry = FnEntry {
    signature: "temp_dir()",
    description: "returns the path of the system's temporary directory",
    example: r#"
get std::fs::temp_dir
get std::io::println

println(temp_dir())"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["mkdir", "rmdir"],
    since: Some("v0.1.5"),
};
