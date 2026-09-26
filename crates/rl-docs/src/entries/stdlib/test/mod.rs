use crate::entry::{FnEntry, StdEntry};

mod test_skip;
mod test_skip_if;
mod test_assert_eq;
mod test_assert_ne;
mod test_assert_panics;
mod test_run_registered;
mod test_assert_no_panic;

pub static TEST: StdEntry = StdEntry {
    name: "test",
    description: "runtime helpers for attribute-driven tests: skips and non-fatal assertions",
    functions: FUNCTIONS,
    since: Some("v2.3.0"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &test_skip::TEST_SKIP,
    &test_skip_if::TEST_SKIP_IF,
    &test_assert_eq::TEST_ASSERT_EQ,
    &test_assert_ne::TEST_ASSERT_NE,
    &test_assert_panics::TEST_ASSERT_PANICS,
    &test_run_registered::TEST_RUN_REGISTERED,
    &test_assert_no_panic::TEST_ASSERT_NO_PANIC,
];
