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

use rl_ast::statements::TypeAnnotation;
use std::rc::Rc;

pub mod bitwise;
pub mod constants;
pub mod io;
pub mod math;
pub mod terminal;
pub mod types;

/// Builds the "input" half of a signature pair: a `Tuple` of the expected
/// argument types, in order. An empty `Vec` means "no arguments".
pub fn params(types: Vec<TypeAnnotation>) -> TypeAnnotation {
    TypeAnnotation::Tuple(Rc::new(types))
}

/// Wraps a type as `Result[T]` - the return type of every stdlib function
/// that can fail and follows the `vok!`/`verr!` runtime convention.
pub fn result(inner: TypeAnnotation) -> TypeAnnotation {
    TypeAnnotation::Result(Box::new(inner))
}
