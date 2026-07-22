//! Typed signatures for `std::debug`. Mirrors
//! `rl-interpreter/src/stdlib/debug/*.rs`.

use super::{NUMERIC, params};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;

pub fn module() -> ModuleNames {
    ModuleNames::new("debug")
        .with_functions(&["assert_eq", "assert_ne", "dbg", "type_of"])
        .with_typed_function(assert())
        .with_typed_function(assert_cmp("assert_lt"))
        .with_typed_function(assert_cmp("assert_le"))
        .with_typed_function(assert_cmp("assert_gt"))
        .with_typed_function(assert_cmp("assert_ge"))
        .with_typed_function(assert_approx_eq())
        .with_typed_function(message_or_not("panic"))
        .with_typed_function(message_or_not("unreachable"))
        .with_typed_function(message_or_not("todo"))
        .with_typed_function(bench())
}

fn numeric_combinations(n: usize) -> Vec<Vec<T>> {
    let mut combos = vec![vec![]];
    for _ in 0..n {
        combos = combos
            .into_iter()
            .flat_map(|prefix| {
                NUMERIC.iter().map(move |t| {
                    let mut next = prefix.clone();
                    next.push(t.clone());
                    next
                })
            })
            .collect();
    }
    combos
}

fn assert() -> StdFn {
    StdFn::typed(
        "assert",
        vec![
            (params(vec![T::Bool]), T::Null),
            (params(vec![T::Bool, T::String]), T::Null),
        ],
    )
}

fn assert_cmp(name: &'static str) -> StdFn {
    let mut signatures = Vec::with_capacity(18);
    for combo in numeric_combinations(2) {
        signatures.push((params(combo.clone()), T::Null));
        let mut with_message = combo;
        with_message.push(T::String);
        signatures.push((params(with_message), T::Null));
    }
    StdFn::typed(name, signatures)
}

fn assert_approx_eq() -> StdFn {
    let mut signatures = Vec::with_capacity(36);
    for combo in numeric_combinations(2) {
        signatures.push((params(combo), T::Null));
    }
    for combo in numeric_combinations(3) {
        signatures.push((params(combo), T::Null));
    }
    StdFn::typed("assert_approx_eq", signatures)
}

fn message_or_not(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![
            (params(vec![]), T::Null),
            (params(vec![T::String]), T::Null),
        ],
    )
}

fn bench() -> StdFn {
    StdFn::typed(
        "bench",
        vec![(params(vec![T::Fn, T::Int]), T::Result(Box::new(T::Float)))],
    )
}
