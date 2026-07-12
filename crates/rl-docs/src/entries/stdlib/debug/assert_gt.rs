use crate::entry::FnEntry;

pub static ASSERT_GT: FnEntry = FnEntry {
    signature: "assert_gt(a, b, msg?)",
    description: "errors if `a` is not greater than `b`; accepts int, float, or byte",
    example: r#"
get std::debug::assert_gt

assert_gt(2, 1)"#,
    expected_output: None,
    returns: "null",
    errors: Some("raises a runtime error when `a <= b`, or when either argument is not numeric"),
    see_also: &["assert_ge", "assert_lt", "assert_le"],
    since: Some("v0.1.5"),
};
