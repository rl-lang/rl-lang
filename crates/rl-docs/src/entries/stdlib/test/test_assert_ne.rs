use crate::entry::FnEntry;

pub static TEST_ASSERT_NE: FnEntry = FnEntry {
    signature: "test_assert_ne(a, b, msg)",
    description: "records a pass when a differs from b, otherwise records msg with both values",
    example: "get test_assert_ne from std::test\n\ntest_assert_ne(1, 2, \"different\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_assert_eq", "test_assert_no_panic"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
