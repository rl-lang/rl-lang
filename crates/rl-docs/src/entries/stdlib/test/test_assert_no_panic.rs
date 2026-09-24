use crate::entry::FnEntry;

pub static TEST_ASSERT_NO_PANIC: FnEntry = FnEntry {
    signature: "test_assert_no_panic(f)",
    description: "records a pass when calling f succeeds, otherwise records the failure",
    example: "get test_assert_no_panic from std::test\n\ntest_assert_no_panic(fn() {\n    1 + 1\n})",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_assert_panics", "test_assert_ne"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
