use crate::docs::entry::FnEntry;

pub static ASSERT_APPROX_EQ: FnEntry = FnEntry {
    signature: "assert_approx_eq(a, b, epsilon?)",
    description: "errors if `a` and `b` differ by more than `epsilon` (default 1e-9); use for float comparisons instead of assert_eq, since exact float equality is unreliable",
    example: r#"
get std::debug::assert_approx_eq

assert_approx_eq(0.1 + 0.2, 0.3)
assert_approx_eq(1.0, 1.1, 0.2)"#,
    expected_output: None,
    returns: "null",
    errors: Some(
        "raises a runtime error when the absolute difference exceeds `epsilon`, or when the arguments aren't numeric",
    ),
    see_also: &["assert_eq"],
    since: Some("v0.1.5"),
};
