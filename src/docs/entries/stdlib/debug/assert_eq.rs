use crate::docs::entry::FnEntry;

pub static ASSERT_EQ: FnEntry = FnEntry {
    signature: "assert_eq(a, b, msg?)",
    description: "errors if `a` and `b` are not equal, reporting both values and their types in the default message",
    example: r#"
get std::debug::assert_eq

assert_eq(2 + 2, 4)
assert_eq(1, 2, \"math is broken\")"#,
    expected_output: None,
    returns: "null",
    errors: Some("raises a runtime error when `a != b`"),
    see_also: &["assert_ne", "assert_approx_eq"],
    since: Some("v0.1.5"),
};
