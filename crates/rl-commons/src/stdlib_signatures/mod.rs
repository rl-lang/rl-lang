//! Typed `(params, return_type)` signatures for `std` functions (rl-lang#250).
//!
//! Each submodule here corresponds to one `std::*` module and builds
//! [`crate::StdFn`] values for the functions in it that have a known,
//! non-generic signature. Modules are typed incrementally - anything not
//! covered yet stays registered via `ModuleNames::with_functions` (untyped,
//! unchecked) in [`crate::stdlib_names`].
//!
//! Functions that are generic over their element type (`arr_push`,
//! `arr_map`, `set_add`, `result_map`, ...) or variadic (`print`, `format`)
//! are intentionally left untyped for now: the checker has no generics
//! system, so a fixed `TypeAnnotation` signature for them would either
//! reject valid calls or silently accept bad ones.

use crate::StdFn;
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub mod array;
pub mod bitwise;
pub mod collections;
pub mod constants;
pub mod debug;
pub mod fs;
pub mod http;
pub mod io;
pub mod math;
pub mod net;
pub mod path;
pub mod process;
pub mod random;
pub mod res;
pub mod rl;
pub mod str;
pub mod terminal;
pub mod time;
pub mod types;

/// Builds the "input" half of a signature pair: a `Tuple` of the expected
/// argument types, in order. An empty `Vec` means "no arguments".
pub fn params(types: Vec<T>) -> T {
    T::Tuple(Rc::new(types))
}

/// Wraps a type as `Result[T]` - the return type of every stdlib function
/// that can fail and follows the `vok!`/`verr!` runtime convention.
pub fn result(inner: T) -> T {
    T::Result(Box::new(inner))
}

pub const NUMERIC: [T; 3] = [T::Int, T::Float, T::Byte];

/// The generic placeholder `T` - shared by every module (`array`,
/// `collections`, `random`, `res`) whose functions are generic over their
/// element type but still have a fixed argument/return "shape" (e.g.
/// `arr_first(array[T]) -> T`, `set_len(set[T]) -> int`).
pub fn t() -> T {
    T::Generic("T".into())
}

/// `array[T]` - shorthand for "array of the generic element", shared by
/// `array` and `random`.
pub fn arr_t() -> T {
    T::Array(Box::new(t()))
}

/// A handle-typed argument slot, accepted as either `int` or `byte`.
/// Shared by `http`/`net`, whose functions take socket/connection handles.
pub fn handle() -> Vec<T> {
    vec![T::Int, T::Byte]
}

/// A single fixed-type argument slot, for use alongside [`handle`] in
/// [`combos`]/[`overloads`].
pub fn fixed(t: T) -> Vec<T> {
    vec![t]
}

/// Expands one option list per positional argument slot into every
/// combination, e.g. `combos(vec![vec![a, b], vec![c]])` =>
/// `[[a, c], [b, c]]`. Shared by `http`/`net`.
pub fn combos(parts: Vec<Vec<T>>) -> Vec<Vec<T>> {
    parts.into_iter().fold(vec![vec![]], |acc, options| {
        acc.into_iter()
            .flat_map(|prefix| {
                options.iter().map(move |o| {
                    let mut next = prefix.clone();
                    next.push(o.clone());
                    next
                })
            })
            .collect()
    })
}

/// Expands [`combos`] into `(params, return_type)` overload pairs sharing
/// one return type. Shared by `http`/`net`.
pub fn overloads(parts: Vec<Vec<T>>, ret: T) -> Vec<(T, T)> {
    combos(parts)
        .into_iter()
        .map(|combo| (params(combo), ret.clone()))
        .collect()
}

/// `handle_arg -> Result[string]` - a handle-only call that yields a
/// string (e.g. an address, a header value). Shared by `http`/`net`.
pub fn handle_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, overloads(vec![handle()], result(T::String)))
}

/// `(string) -> string` - shared by `path` and `str` for their many
/// scalar transforms (`path_stem`, `to_lower`, `trim`, ...).
pub fn string_to_string(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::String]), T::String)])
}
