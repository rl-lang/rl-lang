use crate::entry::FnEntry;

pub static PROMPT_CONFIRM: FnEntry = FnEntry {
    signature: "prompt_confirm(msg)",
    description: "asks a yes/no question (appends [y/n] to msg) and returns true for y or yes (case-insensitive), false for anything else",
    example: r#"get prompt_confirm from std::cli

if prompt_confirm("delete all files?") {
    println("deleting...")
}"#,
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["prompt", "prompt_password", "prompt_choice"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
