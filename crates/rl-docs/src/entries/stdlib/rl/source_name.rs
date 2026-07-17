use crate::entry::FnEntry;

pub static SOURCE_NAME: FnEntry = FnEntry {
    signature: "source_name()",
    description: "returns the name of the file currently being run, or null if there is no source file",
    example: r#"get std::rl::source_name

source_name()"#,
    expected_output: Some("main.rl"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
