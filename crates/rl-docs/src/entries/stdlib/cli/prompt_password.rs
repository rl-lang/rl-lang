use crate::entry::FnEntry;

pub static PROMPT_PASSWORD: FnEntry = FnEntry {
    signature: "prompt_password(msg)",
    description: "same as prompt, but terminal echo is disabled so the input is not displayed. returns an empty string on EOF or read error",
    example: r#"get prompt_password from std::cli

dec string pw = prompt_password("password: ")"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["prompt", "prompt_confirm", "prompt_choice"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
