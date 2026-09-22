use crate::entry::FnEntry;

pub static PROMPT: FnEntry = FnEntry {
    signature: "prompt(msg)",
    description: "prints msg without a trailing newline, flushes stdout, and reads one line from stdin with the trailing newline stripped. returns an empty string on EOF or read error",
    example: r#"get prompt from std::cli

dec string name = prompt("what is your name? ")"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["prompt_password", "prompt_confirm", "prompt_choice"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
