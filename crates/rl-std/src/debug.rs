//! `std::debug` - assertions, panics, and debug utilities.
//!
//! Ported once from the former per-runtime `stdlib/debug/*.rs` copies. The
//! assertion / panic family were `with_raw_function`s (variadic `Vec<Value>` +
//! span) that raise a real runtime error on failure, so they are `untyped` and
//! return a PROPAGATING `Result<R::Value, Error>`. `dbg` / `type_of` / `bench`
//! were plain `with_function`s.

use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

// ---- shared helpers -------------------------------------------------------

/// The old `debug::common::as_f64`: numeric coercion used by the ordered /
/// approximate comparisons. Only `int` / `float` / `byte` coerce.
fn as_f64<R: Runtime>(v: &R::Value) -> Option<f64> {
    if let Some(i) = R::as_i64(v) {
        Some(i as f64)
    } else if let Some(f) = R::as_f64(v) {
        Some(f)
    } else {
        R::as_u8(v).map(|b| b as f64)
    }
}

/// Reproduces the old `assert_cmp` for the ordered comparisons
/// (`assert_lt/le/gt/ge`): coerce both operands to `f64`, apply `op`, and on
/// failure raise a runtime error carrying the (optionally custom-prefixed)
/// message.
fn assert_cmp<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
    name: &str,
    op: fn(f64, f64) -> bool,
) -> Result<R::Value, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(R::error(
            cx,
            format!("{}() expects 2 or 3 arguments, got {}", name, args.len()),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    let (fa, fb) = match (as_f64::<R>(a), as_f64::<R>(b)) {
        (Some(fa), Some(fb)) => (fa, fb),
        _ => {
            return Err(R::error(
                cx,
                format!(
                    "{}: expects numeric arguments, got {} and {}",
                    name,
                    R::type_name(a),
                    R::type_name(b)
                ),
                span,
            ));
        }
    };

    if !op(fa, fb) {
        let default_msg = format!(
            "{} failed: `{}` vs `{}`",
            name,
            R::display(a),
            R::display(b)
        );
        let message = match args.get(2) {
            Some(v) if R::as_str(v).is_some() => {
                format!("{}: {}", R::as_str(v).unwrap(), default_msg)
            }
            Some(other) => {
                return Err(R::error(
                    cx,
                    format!(
                        "{}: expects a string message, got {}",
                        name,
                        R::type_name(other)
                    ),
                    span,
                ));
            }
            None => default_msg,
        };
        return Err(R::error(cx, message, span));
    }

    Ok(R::null())
}

/// Reproduces the old `debug::common::assert_eq_message` collapsed with the
/// `extract_string` roundtrip: builds the failure message for `assert_eq` /
/// `assert_ne`, honoring an optional custom string prefix. Returns `Ok(msg)`
/// to raise, or `Err(msg)` when the custom argument is not a string (which the
/// old code surfaced as the raised error too).
fn assert_eq_message<R: Runtime>(
    a: &R::Value,
    b: &R::Value,
    custom: Option<&R::Value>,
    name: &str,
    expected_equal: bool,
) -> String {
    let op = if expected_equal { "!=" } else { "==" };
    let default_msg = format!(
        "{} failed: left `{}` ({}) {} right `{}` ({})",
        name,
        R::display(a),
        R::type_name(a),
        op,
        R::display(b),
        R::type_name(b)
    );

    match custom {
        Some(v) if R::as_str(v).is_some() => {
            format!("{}: {}", R::as_str(v).unwrap(), default_msg)
        }
        Some(other) => format!(
            "{}() expects a string message, got {}",
            name,
            R::type_name(other)
        ),
        None => default_msg,
    }
}

// ---- assertions -----------------------------------------------------------

#[native_fn(module = "debug", untyped)]
pub fn assert<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.is_empty() || args.len() > 2 {
        return Err(R::error(
            cx,
            format!("assert: expects 1 or 2 arguments, got {}", args.len()),
            span,
        ));
    }

    let cond = match R::as_bool(&args[0]) {
        Some(b) => b,
        None => {
            return Err(R::error(
                cx,
                format!(
                    "assert: expects a bool condition, got {}",
                    R::type_name(&args[0])
                ),
                span,
            ));
        }
    };

    if !cond {
        let message = match args.get(1) {
            Some(v) if R::as_str(v).is_some() => R::as_str(v).unwrap().to_string(),
            Some(other) => {
                return Err(R::error(
                    cx,
                    format!(
                        "assert: expects a string message, got {}",
                        R::type_name(other)
                    ),
                    span,
                ));
            }
            None => "assertion failed".to_string(),
        };
        return Err(R::error(cx, message, span));
    }

    Ok(R::null())
}

#[native_fn(module = "debug", untyped)]
pub fn assert_eq<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(R::error(
            cx,
            format!("assert_eq: expects 2 or 3 arguments, got {}", args.len()),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    if !R::values_equal(a, b) {
        let err_string = assert_eq_message::<R>(a, b, args.get(2), "assert_eq", true);
        return Err(R::error(cx, err_string, span));
    }
    Ok(R::null())
}

#[native_fn(module = "debug", untyped)]
pub fn assert_ne<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(R::error(
            cx,
            format!("assert_ne: expects 2 or 3 arguments, got {}", args.len()),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    if R::values_equal(a, b) {
        let err_string = assert_eq_message::<R>(a, b, args.get(2), "assert_ne", false);
        return Err(R::error(cx, err_string, span));
    }
    Ok(R::null())
}

#[native_fn(module = "debug", untyped)]
pub fn assert_lt<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    assert_cmp::<R>(cx, args, span, "assert_lt", |a, b| a < b)
}

#[native_fn(module = "debug", untyped)]
pub fn assert_le<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    assert_cmp::<R>(cx, args, span, "assert_le", |a, b| a <= b)
}

#[native_fn(module = "debug", untyped)]
pub fn assert_gt<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    assert_cmp::<R>(cx, args, span, "assert_gt", |a, b| a > b)
}

#[native_fn(module = "debug", untyped)]
pub fn assert_ge<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    assert_cmp::<R>(cx, args, span, "assert_ge", |a, b| a >= b)
}

#[native_fn(module = "debug", untyped)]
pub fn assert_approx_eq<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    // third spot IS NOT MESSAGE it is epsilon
    if args.len() < 2 || args.len() > 3 {
        return Err(R::error(
            cx,
            format!(
                "assert_approx_eq: expects 2 or 3 arguments, got {}",
                args.len()
            ),
            span,
        ));
    }

    let (a, b) = (&args[0], &args[1]);
    let (fa, fb) = match (as_f64::<R>(a), as_f64::<R>(b)) {
        (Some(fa), Some(fb)) => (fa, fb),
        _ => {
            return Err(R::error(
                cx,
                format!(
                    "assert_approx_eq: expects numeric arguments, got {} and {}",
                    R::type_name(a),
                    R::type_name(b)
                ),
                span,
            ));
        }
    };

    let epsilon = match args.get(2) {
        Some(v) => match as_f64::<R>(v) {
            Some(e) => e,
            None => {
                return Err(R::error(
                    cx,
                    format!(
                        "assert_approx_eq: expects a numeric epsilon, got {}",
                        R::type_name(v)
                    ),
                    span,
                ));
            }
        },
        None => 1e-9,
    };

    if (fa - fb).abs() > epsilon {
        return Err(R::error(
            cx,
            format!(
                "assert_approx_eq failed: `{}` and `{}` differ by more than {}",
                R::display(a),
                R::display(b),
                epsilon
            ),
            span,
        ));
    }

    Ok(R::null())
}

// ---- panics ---------------------------------------------------------------

#[native_fn(module = "debug", untyped)]
pub fn panic<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() > 1 {
        return Err(R::error(
            cx,
            format!("panic: expects 0 or 1 arguments, got {}", args.len()),
            span,
        ));
    }
    let message = match args.into_iter().next() {
        Some(v) if R::as_str(&v).is_some() => R::as_str(&v).unwrap().to_string(),
        Some(other) => {
            return Err(R::error(
                cx,
                format!(
                    "panic: expects a string message, got {}",
                    R::type_name(&other)
                ),
                span,
            ));
        }
        None => "explicit panic".to_string(),
    };
    Err(R::error(cx, message, span))
}

#[native_fn(module = "debug", untyped)]
pub fn unreachable<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() > 1 {
        return Err(R::error(
            cx,
            format!("unreachable: expects 0 or 1 arguments, got {}", args.len()),
            span,
        ));
    }

    let message = match args.into_iter().next() {
        Some(v) if R::as_str(&v).is_some() => {
            format!(
                "internal error: entered unreachable code: {}",
                R::as_str(&v).unwrap()
            )
        }
        Some(other) => {
            return Err(R::error(
                cx,
                format!(
                    "unreachable: expects a string message, got {}",
                    R::type_name(&other)
                ),
                span,
            ));
        }
        None => "internal error: entered unreachable code".to_string(),
    };

    Err(R::error(cx, message, span))
}

#[native_fn(module = "debug", untyped)]
pub fn todo<R: Runtime>(
    cx: &mut R::Cx,
    args: Vec<R::Value>,
    span: R::Span,
) -> Result<R::Value, Error> {
    if args.len() > 1 {
        return Err(R::error(
            cx,
            format!("todo: expects 0 or 1 arguments, got {}", args.len()),
            span,
        ));
    }
    let message = match args.into_iter().next() {
        Some(v) if R::as_str(&v).is_some() => {
            format!("not yet implemented: {}", R::as_str(&v).unwrap())
        }
        Some(other) => {
            return Err(R::error(
                cx,
                format!(
                    "todo: expects a string message, got {}",
                    R::type_name(&other)
                ),
                span,
            ));
        }
        None => "not yet implemented".to_string(),
    };
    Err(R::error(cx, message, span))
}

// ---- debug utilities ------------------------------------------------------

#[native_fn(module = "debug", sig(_ -> string))]
pub fn type_of<R: Runtime>(v: R::Value) -> String {
    R::type_name(&v).to_string()
}

#[native_fn(module = "debug", sig(_ -> _))]
pub fn dbg<R: Runtime>(cx: &mut R::Cx, value: R::Value) -> R::Value {
    let text = format!("[dbg] {} ({})\n", R::display(&value), R::type_name(&value));

    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
    } else {
        eprint!("{}", text);
    }

    value
}

// ---- warn / stack_trace ---------------------------------------------------

#[native_fn(module = "debug", sig(string -> null))]
pub fn warn<R: Runtime>(cx: &mut R::Cx, msg: String) -> R::Value {
    let text = format!("\x1b[33m[warn]\x1b[0m {}\n", msg);

    if let Some(buffer) = R::output_buffer(cx) {
        buffer.push_str(&text);
    } else {
        eprint!("{}", text);
    }

    R::null()
}

#[native_fn(module = "debug", sig( -> string))]
pub fn stack_trace() -> String {
    let bt = std::backtrace::Backtrace::force_capture();
    bt.to_string()
}

#[native_fn(module = "debug", sig(fn, int -> result[float]))]
pub fn bench<R: Runtime>(
    cx: &mut R::Cx,
    function: R::Value,
    iterations: i64,
    span: R::Span,
) -> Result<f64, String> {
    if !R::is_callable(&function) {
        return Err(format!(
            "bench: expects a function or lambda, got {}",
            R::type_name(&function)
        ));
    }

    // The old body matched the raw value; a non-int is already rejected by the
    // typed `int` extraction, so the only surviving failure here is a
    // non-positive int, whose original `type_name()` was `"int"`.
    if iterations <= 0 {
        return Err("bench: expects a positive int for iterations, got int".to_string());
    }
    let iterations = iterations as u64;

    let start = std::time::Instant::now();
    for _ in 0..iterations {
        if let Err(e) = R::call_value(cx, &function, &[], span) {
            return Err(format!(
                "bench: error executing the function: {}",
                e.message()
            ));
        }
    }
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;

    Ok(elapsed_ms)
}

rl_std_core::native_module!("debug";
    funcs: [
        assert, assert_eq, assert_ne,
        assert_lt, assert_le, assert_gt, assert_ge,
        assert_approx_eq,
        panic, unreachable, todo,
        dbg, type_of, bench,
        warn, stack_trace,
    ],
);
