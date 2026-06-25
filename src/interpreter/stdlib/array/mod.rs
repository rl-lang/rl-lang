//! `std::array` - array manipulation functions.
//!
//! All functions operate on [`Value::Values`] and return errors for non-array inputs.
//! Higher-order functions (`map`, `filter`, `reduce`, etc.) require a [`Value::Function`]
//! with the correct return type annotation.

use super::len;
use crate::interpreter::native::Module;

mod arr_all;
mod arr_any;
mod arr_concat;
mod arr_contains;
mod arr_count;
mod arr_fill;
mod arr_filter;
mod arr_find;
mod arr_find_index;
mod arr_first;
mod arr_flat_map;
mod arr_flatten;
mod arr_for_each;
mod arr_index_of;
mod arr_insert;
mod arr_is_empty;
mod arr_last;
mod arr_map;
mod arr_max;
mod arr_min;
mod arr_pop;
mod arr_product;
mod arr_push;
mod arr_range;
mod arr_reduce;
mod arr_remove;
mod arr_reverse;
mod arr_slice;
mod arr_sort;
mod arr_sort_by;
mod arr_sum;
mod arr_unique;
mod arr_zip;

pub const KEYWORDS: &[&str] = &[
    "arr_push",
    "arr_pop",
    "arr_insert",
    "arr_remove",
    "arr_reverse",
    "arr_concat",
    "arr_first",
    "arr_last",
    "arr_max",
    "arr_min",
    "arr_sum",
    "arr_product",
    "arr_unique",
    "arr_is_empty",
    "arr_count",
    "arr_contains",
    "arr_index_of",
    "arr_sort",
    "arr_slice",
    "arr_flatten",
    "arr_range",
    "arr_fill",
    "arr_map",
    "len",
    "arr_filter",
    "arr_all",
    "arr_any",
    "arr_find",
    "arr_find_index",
    "arr_reduce",
    "arr_sort_by",
    "arr_flat_map",
    "arr_for_each",
    "arr_zip",
];

pub fn module() -> Module {
    Module::new("array")
        .with_function("arr_push", arr_push::std_arr_push)
        .with_function("arr_pop", arr_pop::std_arr_pop)
        .with_function("arr_insert", arr_insert::std_arr_insert)
        .with_function("arr_remove", arr_remove::std_arr_remove)
        .with_function("arr_reverse", arr_reverse::std_arr_reverse)
        .with_function("arr_concat", arr_concat::std_arr_concat)
        .with_function("arr_first", arr_first::std_arr_first)
        .with_function("arr_last", arr_last::std_arr_last)
        .with_function("arr_max", arr_max::std_arr_max)
        .with_function("arr_min", arr_min::std_arr_min)
        .with_function("arr_sum", arr_sum::std_arr_sum)
        .with_function("arr_product", arr_product::std_arr_product)
        .with_function("arr_unique", arr_unique::std_arr_unique)
        .with_function("arr_is_empty", arr_is_empty::std_arr_is_empty)
        .with_function("arr_index_of", arr_index_of::std_arr_index_of)
        .with_function("arr_count", arr_count::std_arr_count)
        .with_function("arr_contains", arr_contains::std_arr_contains)
        .with_function("arr_range", arr_range::std_arr_range)
        .with_function("arr_flatten", arr_flatten::std_arr_flatten)
        .with_function("arr_sort", arr_sort::std_arr_sort)
        .with_function("arr_fill", arr_fill::std_arr_fill)
        .with_function("arr_slice", arr_slice::std_arr_slice)
        .with_function("arr_map", arr_map::std_arr_map)
        .with_function("len", len::std_len)
        .with_function("arr_filter", arr_filter::std_arr_filter)
        .with_function("arr_all", arr_all::std_arr_all)
        .with_function("arr_any", arr_any::std_arr_any)
        .with_function("arr_find", arr_find::std_arr_find)
        .with_function("arr_find_index", arr_find_index::std_arr_find_index)
        .with_function("arr_reduce", arr_reduce::std_arr_reduce)
        .with_function("arr_sort_by", arr_sort_by::std_arr_sort_by)
        .with_function("arr_flat_map", arr_flat_map::std_arr_flat_map)
        .with_function("arr_for_each", arr_for_each::std_arr_for_each)
        .with_raw_function("arr_zip", arr_zip::std_arr_zip)
}
