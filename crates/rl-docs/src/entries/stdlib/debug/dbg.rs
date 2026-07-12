use crate::docs::entry::FnEntry;

pub static DBG: FnEntry = FnEntry {
    signature: "dbg(value)",
    description: "prints `value` and its type, then returns `value` unchanged, so it can be dropped inline into an expression without restructuring code",
    example: r#"
get std::debug::dbg

dec int x = dbg(2 + 2)
        "#,
    expected_output: Some("[dbg] 4 (int)"),
    returns: "same type as `value`",
    errors: None,
    see_also: &["type_of"],
    since: Some("v0.1.5"),
};
