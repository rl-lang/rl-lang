//! Typed signatures for `std::array`

use super::{params, result};
use crate::{ModuleNames, StdFn};
use rl_ast::statements::TypeAnnotation as T;
use std::rc::Rc;

pub fn module() -> ModuleNames {
    ModuleNames::new("array")
        .with_functions(&["len"])
        .with_typed_function(arr_push())
        .with_typed_function(arr_pop())
        .with_typed_function(arr_insert())
        .with_typed_function(arr_remove())
        .with_typed_function(arr_reverse())
        .with_typed_function(arr_concat())
        .with_typed_function(arr_first())
        .with_typed_function(arr_last())
        .with_typed_function(arr_unique())
        .with_typed_function(arr_is_empty())
        .with_typed_function(arr_count())
        .with_typed_function(arr_contains())
        .with_typed_function(arr_index_of())
        .with_typed_function(arr_slice())
        .with_typed_function(arr_flatten())
        .with_typed_function(arr_fill())
        .with_typed_function(arr_zip())
        .with_typed_function(arr_map())
        .with_typed_function(arr_filter())
        .with_typed_function(arr_find())
        .with_typed_function(arr_find_index())
        .with_typed_function(arr_flat_map())
        .with_typed_function(arr_for_each())
        .with_typed_function(arr_all())
        .with_typed_function(arr_any())
        .with_typed_function(arr_reduce())
        .with_typed_function(arr_sort_by())
        .with_typed_function(arr_max())
        .with_typed_function(arr_min())
        .with_typed_function(arr_sum())
        .with_typed_function(arr_product())
        .with_typed_function(arr_sort())
        .with_typed_function(arr_range())
}

fn t() -> T {
    T::Generic("T".into())
}
fn u() -> T {
    T::Generic("U".into())
}
fn arr_t() -> T {
    T::Array(Box::new(t()))
}
fn arr_u() -> T {
    T::Array(Box::new(u()))
}
fn predicate() -> T {
    T::Callback(vec![t()], Box::new(T::Bool))
}

fn arr_push() -> StdFn {
    StdFn::typed(
        "arr_push",
        vec![(params(vec![arr_t(), t()]), result(arr_t()))],
    )
}
fn arr_pop() -> StdFn {
    StdFn::typed("arr_pop", vec![(params(vec![arr_t()]), result(arr_t()))])
}
fn arr_insert() -> StdFn {
    StdFn::typed(
        "arr_insert",
        vec![(params(vec![arr_t(), t(), T::Int]), result(arr_t()))],
    )
}
fn arr_remove() -> StdFn {
    StdFn::typed(
        "arr_remove",
        vec![(params(vec![arr_t(), T::Int]), result(arr_t()))],
    )
}
fn arr_reverse() -> StdFn {
    StdFn::typed(
        "arr_reverse",
        vec![(params(vec![arr_t()]), result(arr_t()))],
    )
}
fn arr_concat() -> StdFn {
    StdFn::typed(
        "arr_concat",
        vec![(params(vec![arr_t(), arr_t()]), result(arr_t()))],
    )
}
fn arr_first() -> StdFn {
    StdFn::typed("arr_first", vec![(params(vec![arr_t()]), result(t()))])
}
fn arr_last() -> StdFn {
    StdFn::typed("arr_last", vec![(params(vec![arr_t()]), result(t()))])
}
fn arr_unique() -> StdFn {
    StdFn::typed("arr_unique", vec![(params(vec![arr_t()]), result(arr_t()))])
}
fn arr_is_empty() -> StdFn {
    StdFn::typed(
        "arr_is_empty",
        vec![(params(vec![arr_t()]), result(T::Bool))],
    )
}
fn arr_count() -> StdFn {
    StdFn::typed("arr_count", vec![(params(vec![arr_t()]), result(T::Int))])
}
fn arr_contains() -> StdFn {
    StdFn::typed(
        "arr_contains",
        vec![(params(vec![arr_t(), t()]), result(T::Bool))],
    )
}
fn arr_index_of() -> StdFn {
    StdFn::typed(
        "arr_index_of",
        vec![(params(vec![arr_t(), t()]), result(T::Int))],
    )
}
fn arr_slice() -> StdFn {
    StdFn::typed(
        "arr_slice",
        vec![(params(vec![arr_t(), T::Int, T::Int]), result(arr_t()))],
    )
}

fn arr_flatten() -> StdFn {
    StdFn::typed(
        "arr_flatten",
        vec![(params(vec![T::Array(Box::new(arr_t()))]), result(arr_t()))],
    )
}

fn arr_fill() -> StdFn {
    StdFn::typed("arr_fill", vec![(params(vec![t(), T::Int]), arr_t())])
}

fn arr_zip() -> StdFn {
    StdFn::typed(
        "arr_zip",
        vec![(
            params(vec![arr_t(), arr_u()]),
            T::Array(Box::new(T::Tuple(Rc::new(vec![t(), u()])))),
        )],
    )
}

fn arr_map() -> StdFn {
    StdFn::typed(
        "arr_map",
        vec![(
            params(vec![arr_t(), T::Callback(vec![t()], Box::new(u()))]),
            result(arr_u()),
        )],
    )
}
fn arr_filter() -> StdFn {
    StdFn::typed(
        "arr_filter",
        vec![(params(vec![arr_t(), predicate()]), result(arr_t()))],
    )
}
fn arr_find() -> StdFn {
    StdFn::typed(
        "arr_find",
        vec![(params(vec![arr_t(), predicate()]), result(t()))],
    )
}
fn arr_find_index() -> StdFn {
    StdFn::typed(
        "arr_find_index",
        vec![(params(vec![arr_t(), predicate()]), result(T::Int))],
    )
}

fn arr_flat_map() -> StdFn {
    StdFn::typed(
        "arr_flat_map",
        vec![(
            params(vec![arr_t(), T::Callback(vec![t()], Box::new(arr_u()))]),
            result(arr_u()),
        )],
    )
}

fn arr_for_each() -> StdFn {
    StdFn::typed(
        "arr_for_each",
        vec![(
            params(vec![arr_t(), T::Callback(vec![t()], Box::new(T::Null))]),
            result(T::Null),
        )],
    )
}

fn arr_all() -> StdFn {
    StdFn::typed(
        "arr_all",
        vec![(params(vec![arr_t(), predicate()]), result(T::Bool))],
    )
}
fn arr_any() -> StdFn {
    StdFn::typed(
        "arr_any",
        vec![(params(vec![arr_t(), predicate()]), result(T::Bool))],
    )
}

fn arr_reduce() -> StdFn {
    StdFn::typed(
        "arr_reduce",
        vec![(
            params(vec![
                arr_t(),
                T::Callback(vec![u(), t()], Box::new(u())),
                u(),
            ]),
            result(u()),
        )],
    )
}

fn arr_sort_by() -> StdFn {
    StdFn::typed(
        "arr_sort_by",
        vec![(
            params(vec![arr_t(), T::Callback(vec![t(), t()], Box::new(T::Int))]),
            result(arr_t()),
        )],
    )
}

fn numeric_array_reduction(name: &'static str) -> StdFn {
    StdFn::typed(
        name,
        vec![
            (params(vec![T::Array(Box::new(T::Int))]), result(T::Int)),
            (params(vec![T::Array(Box::new(T::Float))]), result(T::Float)),
        ],
    )
}

fn arr_max() -> StdFn {
    numeric_array_reduction("arr_max")
}
fn arr_min() -> StdFn {
    numeric_array_reduction("arr_min")
}
fn arr_sum() -> StdFn {
    numeric_array_reduction("arr_sum")
}
fn arr_product() -> StdFn {
    numeric_array_reduction("arr_product")
}

fn arr_sort() -> StdFn {
    StdFn::typed(
        "arr_sort",
        vec![
            (
                params(vec![T::Array(Box::new(T::Int))]),
                result(T::Array(Box::new(T::Int))),
            ),
            (
                params(vec![T::Array(Box::new(T::Float))]),
                result(T::Array(Box::new(T::Float))),
            ),
        ],
    )
}

fn arr_range() -> StdFn {
    StdFn::typed(
        "arr_range",
        vec![(
            params(vec![T::Int, T::Int, T::Int]),
            result(T::Array(Box::new(T::Int))),
        )],
    )
}
