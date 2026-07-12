use crate::docs::entry::FnEntry;

pub static ASSERT: FnEntry = FnEntry {
    signature: "assert(cond, msg?)",
    description: "errors if `cond` is false, using `msg` if provided or the default \"assertion failed\"",
    example: r#"
get std::debug::assert

assert(1 + 1 == 2)
assert(false, \"should never happen\")"#,
    expected_output: Some("errors: should never happen"),
    returns: "null",
    errors: Some("raises a runtime error when `cond` is not true"),
    see_also: &["assert_eq", "assert_ne"],
    since: Some("v0.1.5"),
};
