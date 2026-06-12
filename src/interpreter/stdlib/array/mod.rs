use crate::interpreter::native::Module;

mod arr_concat;
mod arr_first;
mod arr_is_empty;
mod arr_last;
mod arr_max;
mod arr_min;
mod arr_product;
mod arr_reverse;
mod arr_sum;
mod arr_unique;
mod insert;
mod pop;
mod push;
mod remove;

pub const KEYWORDS: &[&str] = &[
    "push",
    "pop",
    "insert",
    "remove",
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
];

pub fn module() -> Module {
    Module::new("array")
        .with_function("push", push::std_push)
        .with_function("pop", pop::std_pop)
        .with_function("insert", insert::std_insert)
        .with_function("remove", remove::std_remove)
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
}
