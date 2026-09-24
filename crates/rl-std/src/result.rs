//! `std::res` - functions for working with `result[T]` values.
//!
//! The rl module name is `res`; the Rust module is `result`. Ported once from
//! the former per-runtime `stdlib/result/*.rs` copies.
//!
//! These functions are value-polymorphic: they inspect / unwrap `Ok(..)` /
//! `Err(..)` values, so they take a raw `R::Value` and use the `R::as_ok_inner`
//! / `R::as_err_inner` accessors. Their explicit `sig(...)` overloads mirror
//! `rl-commons/src/stdlib_signatures/res.rs` (`result_unwrap_err`,
//! `result_map`, and `result_map_err` were registered untyped there).

use rl_std_core::Runtime;
use rl_std_macros::native_fn;
use rl_utils::errors::Error;

// ---- predicates -----------------------------------------------------------

#[native_fn(module = "res", sig(result[T] -> bool))]
pub fn is_ok<R: Runtime>(value: R::Value) -> bool {
    R::as_ok_inner(&value).is_some()
}

#[native_fn(module = "res", sig(result[T] -> bool))]
pub fn is_err<R: Runtime>(value: R::Value) -> bool {
    R::as_err_inner(&value).is_some()
}

// ---- unwrapping (raises a real language error on the wrong variant) --------

#[native_fn(module = "res", sig(result[T] -> T))]
pub fn result_unwrap<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if let Some(inner) = R::as_ok_inner(&value) {
        Ok(inner)
    } else if let Some(inner) = R::as_err_inner(&value) {
        Err(R::error(
            cx,
            format!("result_unwrap: called on Err({})", R::display(&inner)),
            span,
        ))
    } else {
        Err(R::error(
            cx,
            format!(
                "result_unwrap: expected result, got {}",
                R::type_name(&value)
            ),
            span,
        ))
    }
}

#[native_fn(module = "res", untyped)]
pub fn result_unwrap_err<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if let Some(inner) = R::as_err_inner(&value) {
        Ok(inner)
    } else if let Some(inner) = R::as_ok_inner(&value) {
        Err(R::error(
            cx,
            format!("result_unwrap_err: called on ok({})", R::display(&inner)),
            span,
        ))
    } else {
        Err(R::error(
            cx,
            format!(
                "result_unwrap_err: expected result, got {}",
                R::type_name(&value)
            ),
            span,
        ))
    }
}

#[native_fn(module = "res", sig(result[T], T -> T))]
pub fn result_unwrap_or<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    default: R::Value,
    span: R::Span,
) -> Result<R::Value, Error> {
    if let Some(inner) = R::as_ok_inner(&value) {
        Ok(inner)
    } else if R::as_err_inner(&value).is_some() {
        Ok(default)
    } else {
        Err(R::error(
            cx,
            format!(
                "result_unwrap_or: expected result, got {}",
                R::type_name(&value)
            ),
            span,
        ))
    }
}

// ---- mapping (applies a callable to the wrapped value) --------------------

#[native_fn(module = "res", untyped)]
pub fn result_map<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    f: R::Value,
    span: R::Span,
) -> R::Value {
    if let Some(inner) = R::as_ok_inner(&value) {
        let mapped = match R::call_value(cx, &f, &[inner], span) {
            Ok(mapped) => mapped,
            Err(e) => return R::err(R::from_string(e.message().to_string())),
        };
        R::ok(mapped)
    } else if R::as_err_inner(&value).is_some() {
        // pass error as is
        value
    } else {
        R::err(R::from_string(format!(
            "result_map: expected result, got {}",
            R::type_name(&value)
        )))
    }
}

#[native_fn(module = "res", untyped)]
pub fn result_map_err<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    f: R::Value,
    span: R::Span,
) -> R::Value {
    if let Some(inner) = R::as_err_inner(&value) {
        let mapped = match R::call_value(cx, &f, &[inner], span) {
            Ok(mapped) => mapped,
            Err(e) => return R::err(R::from_string(e.message().to_string())),
        };
        R::err(mapped)
    } else if R::as_ok_inner(&value).is_some() {
        // pass ok as is
        value
    } else {
        R::err(R::from_string(format!(
            "result_map_err: expected result, got {}",
            R::type_name(&value)
        )))
    }
}

// ---- chaining (fallible callbacks) ----------------------------------------

#[native_fn(module = "res", untyped)]
pub fn result_and_then<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    f: R::Value,
    span: R::Span,
) -> R::Value {
    if let Some(inner) = R::as_ok_inner(&value) {
        match R::call_value(cx, &f, &[inner], span) {
            Ok(mapped) => mapped,
            Err(e) => R::err(R::from_string(e.message().to_string())),
        }
    } else if R::as_err_inner(&value).is_some() {
        value
    } else {
        R::err(R::from_string(format!(
            "result_and_then: expected result, got {}",
            R::type_name(&value)
        )))
    }
}

#[native_fn(module = "res", untyped)]
pub fn result_unwrap_or_else<R: Runtime>(
    cx: &mut R::Cx,
    value: R::Value,
    f: R::Value,
    span: R::Span,
) -> R::Value {
    if let Some(inner) = R::as_ok_inner(&value) {
        inner
    } else if let Some(err) = R::as_err_inner(&value) {
        match R::call_value(cx, &f, &[err], span) {
            Ok(default) => default,
            Err(e) => R::err(R::from_string(e.message().to_string())),
        }
    } else {
        R::err(R::from_string(format!(
            "result_unwrap_or_else: expected result, got {}",
            R::type_name(&value)
        )))
    }
}

rl_std_core::native_module!("res";
    funcs: [
        is_ok, is_err,
        result_unwrap, result_unwrap_err, result_unwrap_or,
        result_map, result_map_err,
        result_and_then, result_unwrap_or_else,
    ],
);
