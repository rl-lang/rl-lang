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

use rl_std_core::Runtime;
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

rl_std_core::native_module!("test";
    funcs: [
        test_skip, test_skip_if,
        test_assert_eq, test_assert_ne,
        test_assert_panics, test_assert_no_panic,
    ],
);
