use crate::entry::FnEntry;

pub static ASSERT_NE: FnEntry = FnEntry {
    signature: "assert_ne(a, b, msg?)",
    description: "errors if `a` and `b` are equal",
    example: r#"
get std::debug::assert_ne

assert_ne(1, 2)"#,
    expected_output: None,
    returns: "null",
    errors: Some("raises a runtime error when `a == b`"),
    see_also: &["assert_eq"],
    since: Some("v0.1.5"),
};
