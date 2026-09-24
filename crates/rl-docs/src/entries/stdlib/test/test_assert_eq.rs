use crate::entry::FnEntry;

pub static TEST_ASSERT_EQ: FnEntry = FnEntry {
    signature: "test_assert_eq(a, b, msg)",
    description: "records a pass when a equals b, otherwise records msg with both values",
    example: "get test_assert_eq from std::test\n\ntest_assert_eq(1 + 1, 2, \"math works\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_assert_ne", "test_assert_panics"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
