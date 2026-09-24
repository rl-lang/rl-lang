use crate::entry::FnEntry;

pub static TEST_RUN_REGISTERED: FnEntry = FnEntry {
    signature: "test_run_registered(name)",
    description: "runs every case in the named registry (re-run semantics: no cached verdicts), with setup/teardown hooks around each case like the runner. Property cases run once (generation lives in `rl test`). Returns the new assertion failures",
    example: "get test_run_registered from std::test\n\ntest_run_registered(\"math\")",
    expected_output: None,
    returns: "int",
    errors: Some("Will raise an error for an unknown registry name"),
    see_also: &["test_skip", "test_assert_eq"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
