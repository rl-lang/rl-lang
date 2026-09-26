//! `std::test` - the small runtime API for the attribute-driven test
//! framework (see `test-framework-plan.md`).
//!
//! Cases are discovered by the `rl test` runner from `!#[test]` functions,
//! not registered from RL: there is no `test_case`/`test_run_all`. What
//! attributes cannot express lives here: in-body control flow (`test_skip`,
//! `test_skip_if`) and non-fatal assertions (`test_assert_eq`,
//! `test_assert_ne`, `test_assert_panics`, `test_assert_no_panic`).
//! Outcomes accumulate in the runtime's [`TestState`] without aborting,
//! so one failure never hides the rest; the runner reports pass/fail/skip
//! with a non-zero exit on failure.
//!
//! Dynamic lookup (`test_run_registered`) runs the cases of one named
//! registry: the compiler records every `!#[test]` function (plus setup /
//! teardown hooks) into the registry through the hidden `__test_register`
//! native as definitions execute, so lookup works under `rl run` too.
//! Re-run semantics: every lookup re-executes (no cached verdicts).

use rl_std_core::{Runtime, TestCase};
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

/// Records one assertion outcome.
fn record<R: Runtime>(cx: &mut R::Cx, ok: bool, message: String) {
    let st = R::test_state(cx);
    if ok {
        st.passed += 1;
    } else {
        st.failed += 1;
        st.failures.push(format!("{} {message}", st.context()));
    }
}

fn eq_message<R: Runtime>(a: &R::Value, b: &R::Value, name: &str, equal: bool, msg: &str) -> String {
    let op = if equal { "!=" } else { "==" };
    format!(
        "{name} failed: {msg} (left `{}` ({}) {op} right `{}` ({}))",
        R::display(a),
        R::type_name(a),
        R::display(b),
        R::type_name(b)
    )
}

// ---- in-body control flow -------------------------------------------------

#[native_fn(module = "test", sig(string -> null))]
pub fn test_skip<R: Runtime>(
    cx: &mut R::Cx,
    reason: String,
    span: R::Span,
) -> Result<R::Value, Error> {
    let context = R::test_state(cx).context();
    R::test_state(cx).skipped.push(format!("{context} skipped: {reason}"));
    // Aborts the case body; the runner recognizes the `test_skip:` marker
    // and reports a skip instead of a failure.
    Err(R::error(cx, format!("test_skip: {reason}"), span))
}

#[native_fn(module = "test", sig(bool, string -> null))]
pub fn test_skip_if<R: Runtime>(
    cx: &mut R::Cx,
    cond: bool,
    reason: String,
    span: R::Span,
) -> Result<R::Value, Error> {
    if cond {
        test_skip::<R>(cx, reason, span)
    } else {
        Ok(R::null())
    }
}

// ---- non-fatal assertions -------------------------------------------------

#[native_fn(module = "test", sig(T, T, string -> null))]
pub fn test_assert_eq<R: Runtime>(
    cx: &mut R::Cx,
    a: R::Value,
    b: R::Value,
    msg: String,
) -> R::Value {
    let ok = R::values_equal(&a, &b);
    record::<R>(cx, ok, eq_message::<R>(&a, &b, "test_assert_eq", true, &msg));
    R::null()
}

#[native_fn(module = "test", sig(T, T, string -> null))]
pub fn test_assert_ne<R: Runtime>(
    cx: &mut R::Cx,
    a: R::Value,
    b: R::Value,
    msg: String,
) -> R::Value {
    let ok = !R::values_equal(&a, &b);
    record::<R>(cx, ok, eq_message::<R>(&a, &b, "test_assert_ne", false, &msg));
    R::null()
}

#[native_fn(module = "test", sig(callback(-> T) -> null))]
pub fn test_assert_panics<R: Runtime>(
    cx: &mut R::Cx,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if !R::is_callable(&f) {
        return Err(R::error(
            cx,
            format!("test_assert_panics: expected function or lambda, found {}", R::type_name(&f)),
            span,
        ));
    }
    match R::call_value(cx, &f, &[], span) {
        Err(_) => record::<R>(cx, true, String::new()),
        Ok(_) => record::<R>(
            cx,
            false,
            "test_assert_panics failed: block did not fail".to_string(),
        ),
    }
    Ok(R::null())
}

#[native_fn(module = "test", sig(callback(-> T) -> null))]
pub fn test_assert_no_panic<R: Runtime>(
    cx: &mut R::Cx,
    f: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if !R::is_callable(&f) {
        return Err(R::error(
            cx,
            format!(
                "test_assert_no_panic: expected function or lambda, found {}",
                R::type_name(&f)
            ),
            span,
        ));
    }
    match R::call_value(cx, &f, &[], span) {
        Ok(_) => record::<R>(cx, true, String::new()),
        Err(e) => record::<R>(
            cx,
            false,
            format!("test_assert_no_panic failed: {}", e.message()),
        ),
    }
    Ok(R::null())
}

/// Runs every case in the named registry (re-run semantics: no cached
/// verdicts), with setup/teardown hooks around each case like the
/// runner. Property cases run once (generation lives in `rl test`).
/// Returns the new assertion failures. Unknown registries raise.
#[native_fn(module = "test", sig(string -> int))]
pub fn test_run_registered<R: Runtime>(
    cx: &mut R::Cx,
    name: String,
    span: R::Span,
) -> Result<i64, Error> {
    let st = R::test_state(cx);
    let matched: Vec<TestCase<R::Value>> = st
        .cases
        .iter()
        .filter(|c| c.register.as_deref() == Some(name.as_str()))
        .cloned()
        .collect();
    if matched.is_empty() {
        return Err(R::error(
            cx,
            format!("test_run_registered: unknown registry `{name}`"),
            span,
        ));
    }
    let setups: Vec<R::Value> = R::test_state(cx).setups.clone();
    let teardowns: Vec<R::Value> = R::test_state(cx).teardowns.clone();
    let failed_before = R::test_state(cx).failed;
    for case in &matched {
        let full = case.full_name();
        R::test_state(cx).current = Some(full.clone());
        let failures_before = R::test_state(cx).failures.len();
        let skipped_before = R::test_state(cx).skipped.len();
        let mut hard_error: Option<String> = None;
        for setup in &setups {
            match R::call_value(cx, setup, &[], span) {
                Ok(_) => {}
                Err(e) => {
                    hard_error = Some(format!("setup failed: {}", e.message()));
                    break;
                }
            }
        }
        if hard_error.is_none() {
            // Property cases run once: generation lives in `rl test`.
            match R::call_value(cx, &case.func, &[], span) {
                Ok(_) => {}
                Err(e) => hard_error = Some(e.message().to_string()),
            }
        }
        if hard_error.is_none() {
            for teardown in &teardowns {
                match R::call_value(cx, teardown, &[], span) {
                    Ok(_) => {}
                    Err(e) => {
                        hard_error = Some(format!("teardown failed: {}", e.message()));
                        break;
                    }
                }
            }
        }
        let st = R::test_state(cx);
        if st.skipped.len() == skipped_before
            && hard_error.is_none()
            && st.failures.len() == failures_before
        {
            // Quiet pass: verdicts belong to the runner, not the value.
        } else if st.skipped.len() > skipped_before {
            // Skip recorded already; nothing to add.
        } else if let Some(msg) = hard_error {
            st.failed += 1;
            st.failures.push(format!("[{full}] {msg}"));
        }
        R::test_state(cx).current = None;
    }
    Ok((R::test_state(cx).failed - failed_before) as i64)
}

/// Hidden registration hook invoked by compiler-emitted calls as function
/// definitions execute. `kind` is `case`, `setup`, or `teardown`; empty
/// group/register strings mean absent. Never called by hand.
#[native_fn(module = "test", untyped)]
pub fn __test_register<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() != 5 {
        return Err(R::error(
            cx,
            format!("__test_register: expects 5 arguments, got {}", args.len()),
            span,
        ));
    }
    let mut strings = Vec::with_capacity(4);
    for arg in &args[..4] {
        match R::as_str(arg) {
            Some(s) => strings.push(s.to_string()),
            None => {
                return Err(R::error(
                    cx,
                    format!(
                        "__test_register: expected a string, found {}",
                        R::type_name(arg)
                    ),
                    span,
                ))
            }
        }
    }
    let func = args[4].clone();
    if !R::is_callable(&func) {
        return Err(R::error(cx, "__test_register: expected a function".to_string(), span));
    }
    let st = R::test_state(cx);
    let kind = strings[0].clone();
    if kind == "setup" {
        st.setups.push(func);
    } else if kind == "teardown" {
        st.teardowns.push(func);
    } else if kind == "case" {
        let opt = |s: String| if s.is_empty() { None } else { Some(s) };
        st.cases.push(TestCase {
            name: strings[1].clone(),
            group: opt(strings[2].clone()),
            register: opt(strings[3].clone()),
            cases: None,
            func,
        });
    } else {
        return Err(R::error(
            cx,
            format!("__test_register: unknown kind `{kind}`"),
            span,
        ));
    }
    Ok(R::null())
}

rl_std_core::native_module!("test";
    funcs: [
        test_skip, test_skip_if,
        test_assert_eq, test_assert_ne,
        test_assert_panics, test_assert_no_panic,
        test_run_registered, __test_register,
    ],
);
