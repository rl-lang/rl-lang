use crate::entry::FnEntry;

pub static TEST_SKIP_IF: FnEntry = FnEntry {
    signature: "test_skip_if(cond, reason)",
    description: "skips the current case when cond is true, continues otherwise",
    example: "get test_skip_if from std::test\n\ntest_skip_if(false, \"never skips\")",
    expected_output: None,
    returns: "null",
    errors: None,
    see_also: &["test_skip", "test_assert_eq"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
