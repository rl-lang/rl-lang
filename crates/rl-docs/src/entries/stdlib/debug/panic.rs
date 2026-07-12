use crate::docs::entry::FnEntry;

pub static PANIC: FnEntry = FnEntry {
    signature: "panic(msg?)",
    description: "unconditionally errors with `msg`, or \"explicit panic\" if omitted; signals a programming bug rather than a recoverable failure value",
    example: r#"
get std::debug::panic

panic("unreachable configuration")"#,
    expected_output: None,
    returns: "never returns",
    errors: Some("always raises a runtime error"),
    see_also: &["unreachable", "todo"],
    since: Some("v0.1.5"),
};
