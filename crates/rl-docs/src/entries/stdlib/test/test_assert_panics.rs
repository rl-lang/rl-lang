use crate::entry::FnEntry;

pub static TEST_ASSERT_PANICS: FnEntry = FnEntry {
    signature: "test_assert_panics(f)",
    description: "records a pass when calling f fails, otherwise records a failure",
    example: "get test_assert_panics from std::test\n\nget result_unwrap from std::res\ntest_assert_panics(fn() {\n    result_unwrap(err(\"boom\"))\n})",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_assert_no_panic", "test_assert_eq"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
