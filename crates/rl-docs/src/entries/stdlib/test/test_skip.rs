use crate::entry::FnEntry;

pub static TEST_SKIP: FnEntry = FnEntry {
    signature: "test_skip(reason)",
    description: "marks the current case skipped with a reason and stops its body",
    example: "get test_skip from std::test\n\ntest_skip(\"needs a database\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_skip_if", "test_assert_eq"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
