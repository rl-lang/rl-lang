use crate::interpreter::native::Module;

mod arr_concat;
mod arr_first;
mod arr_last;
mod arr_reverse;
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
}
