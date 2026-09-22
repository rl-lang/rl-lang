use crate::entry::FnEntry;

pub static PROMPT_CHOICE: FnEntry = FnEntry {
    signature: "prompt_choice(msg, options)",
    description: "prints msg with a numbered option list and reads the pick. accepts a 1-based number or the exact option text; repeats until valid. returns an empty string when options is empty or stdin closes",
    example: r#"get prompt_choice from std::cli

dec string color = prompt_choice("pick a color", ["red", "green", "blue"])"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["prompt", "prompt_password", "prompt_confirm"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
