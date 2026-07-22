//! Typed signatures for `std::math::constants`. Mirrors
//! `rl-interpreter/src/stdlib/math/constants/*.rs`.
//!
//! Every constant is a zero-argument function returning `float` (called as
//! `PI()`, not accessed as a bare value) - see the module doc comment on
//! `rl-interpreter/src/stdlib/math/constants/mod.rs`. `is_inf`/`is_nan` are
//! the two exceptions: they take a `float` and return `bool`.

use super::params;
use crate::{ModuleNames, StdFn, keywords::math::constants::KEYWORDS};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    let mut m = ModuleNames::new("consts");
    for name in KEYWORDS {
        m = m.with_typed_function(no_arg_float(name));
    }
    m.with_typed_function(is_inf_or_nan("is_inf"))
        .with_typed_function(is_inf_or_nan("is_nan"))
}

fn no_arg_float(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![]), T::Float)])
}

fn is_inf_or_nan(name: &'static str) -> StdFn {
    StdFn::typed(name, vec![(params(vec![T::Float]), T::Bool)])
}
