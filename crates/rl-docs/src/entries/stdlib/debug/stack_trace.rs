use crate::entry::FnEntry;

pub static STACK_TRACE: FnEntry = FnEntry {
    signature: "stack_trace()",
    description: "returns the current call stack trace as a string",
    example: r#"get std::debug::stack_trace

stack_trace()"#,
    expected_output: Some("\"main -> foo -> bar\""),
    returns: "string",
    errors: None,
    see_also: &["panic", "todo"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
