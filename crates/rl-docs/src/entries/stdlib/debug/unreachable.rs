use crate::docs::entry::FnEntry;

pub static UNREACHABLE: FnEntry = FnEntry {
    signature: "unreachable(msg?)",
    description: "marks a code path that should never execute; errors immediately if it is ever reached",
    example: r#"
get std::debug::unreachable

match true {
    true => {}
    false => {
        unreachable("default arm should be impossible")
        }
}"#,
    expected_output: None,
    returns: "never returns",
    errors: Some("always raises a runtime error"),
    see_also: &["panic", "todo"],
    since: Some("v0.1.5"),
};
