use crate::docs::entry::FnEntry;

pub static TODO: FnEntry = FnEntry {
    signature: "todo(msg?)",
    description: "marks unfinished code; errors with \"not yet implemented\" so incomplete functions fail loudly instead of silently returning null",
    example: r#"
get std::debug::todo

fn upcoming() {
    todo("implement retry logic")
}"#,
    expected_output: None,
    returns: "never returns",
    errors: Some("always raises a runtime error"),
    see_also: &["panic", "unreachable"],
    since: Some("v0.1.5"),
};
