use crate::docs::entry::FnEntry;

pub static ASSERT_LE: FnEntry = FnEntry {
    signature: "assert_le(a, b, msg?)",
    description: "errors if `a` is greater than `b`; accepts int, float, or byte",
    example: r#"
get std::debug::assert_le

assert_le(2, 2)"#,
    expected_output: None,
    returns: "null",
    errors: Some("raises a runtime error when `a > b`, or when either argument is not numeric"),
    see_also: &["assert_lt", "assert_gt", "assert_ge"],
    since: Some("v0.1.5"),
};
