//! Registry for `std::test` - test cases, grouping, and results.
//!
//! The registry lives in the runtime context (like the PRNG behind
//! [`Runtime::rng`]), so each `Vm` gets an isolated registry and REPL
//! sessions do not leak cases into each other. `test_run_all` drives the
//! stored closures through [`Runtime::call_value`]; assertion helpers only
//! append outcomes here, never aborting the run.

/// One registered case: the display name, the enclosing group (if any),
/// the named registry (if any), the property count (if any), and the
/// zero-argument closure to invoke.
#[derive(Clone)]
pub struct TestCase<V> {
    pub name: String,
    pub group: Option<String>,
    pub register: Option<String>,
    pub cases: Option<u64>,
    pub func: V,
}

impl<V> TestCase<V> {
    /// `group::name`, or the bare name outside any group.
    pub fn full_name(&self) -> String {
        match &self.group {
            Some(g) => format!("{g}::{}", self.name),
            None => self.name.clone(),
        }
    }
}

/// Mutable `std::test` state threaded through the runtime context.
#[derive(Clone)]
pub struct TestState<V> {
    /// Cases in registration order.
    pub cases: Vec<TestCase<V>>,
    /// Group stack for `test_group` immediate execution.
    pub group_stack: Vec<String>,
    /// File-level setup closures, run before each case.
    pub setups: Vec<V>,
    /// File-level teardown closures, run after each case.
    pub teardowns: Vec<V>,
    /// Full name of the case currently running (assertion attribution).
    pub current: Option<String>,
    /// Passing assertions since the last reset.
    pub passed: usize,
    /// Failing assertions since the last reset.
    pub failed: usize,
    /// Failure messages in order (`[context] message`).
    pub failures: Vec<String>,
    /// Skipped cases in order (`[context] skipped: reason`).
    pub skipped: Vec<String>,
}

impl<V> Default for TestState<V> {
    fn default() -> Self {
        Self {
            cases: Vec::new(),
            group_stack: Vec::new(),
            setups: Vec::new(),
            teardowns: Vec::new(),
            current: None,
            passed: 0,
            failed: 0,
            failures: Vec::new(),
            skipped: Vec::new(),
        }
    }
}

impl<V> TestState<V> {
    /// Attribution prefix for the next recorded outcome.
    pub fn context(&self) -> String {
        match &self.current {
            Some(name) => format!("[{name}]"),
            None => "[top-level]".to_string(),
        }
    }

    /// Records one assertion outcome, returning it for the `bool` result.
    pub fn record(&mut self, ok: bool, message: String) -> bool {
        if ok {
            self.passed += 1;
        } else {
            self.failed += 1;
            self.failures.push(format!("{} {message}", self.context()));
        }
        ok
    }
}
