use crate::entry::FnEntry;

pub static ASSERT_GE: FnEntry = FnEntry {
    signature: "assert_ge(a, b, msg?)",
    description: "errors if `a` is less than `b`; accepts int, float, or byte",
    example: r#"
get std::debug::assert_ge

assert_ge(2, 2)"#,
    expected_output: None,
    returns: "null",
    errors: Some("raises a runtime error when `a < b`, or when either argument is not numeric"),
    see_also: &["assert_gt", "assert_lt", "assert_le"],
    since: Some("v0.1.5"),
};
