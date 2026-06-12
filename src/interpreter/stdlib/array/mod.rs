use crate::interpreter::native::Module;

mod arr_concat;
mod arr_contains;
mod arr_count;
mod arr_fill;
mod arr_first;
mod arr_flatten;
mod arr_index_of;
mod arr_insert;
mod arr_is_empty;
mod arr_last;
mod arr_max;
mod arr_min;
mod arr_pop;
mod arr_product;
mod arr_push;
mod arr_range;
mod arr_remove;
mod arr_reverse;
mod arr_slice;
mod arr_sort;
mod arr_sum;
mod arr_unique;

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
}
