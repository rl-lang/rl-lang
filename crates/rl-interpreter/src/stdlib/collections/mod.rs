use crate::native::Module;

pub use rl_commons::keywords::collections::KEYWORDS;

mod map_clear;
mod map_contains;
mod map_get;
mod map_is_empty;
mod map_keys;
mod map_len;
mod map_merge;
mod map_remove;
mod map_to_array;
mod map_values;
mod set_add;
mod set_contains;
mod set_is_empty;
mod set_len;
mod set_remove;
mod set_to_array;

pub fn module() -> Module {
    Module::new("collections")
        .with_function("set_add", set_add::std_set_add)
        .with_function("set_remove", set_remove::std_set_remove)
        .with_function("set_contains", set_contains::std_set_contains)
        .with_function("set_len", set_len::std_set_len)
        .with_function("set_is_empty", set_is_empty::std_set_is_empty)
        .with_function("set_to_array", set_to_array::std_set_to_array)
        .with_function("map_contains", map_contains::std_map_contains)
        .with_function("map_remove", map_remove::std_map_remove)
        .with_function("map_len", map_len::std_map_len)
        .with_function("map_is_empty", map_is_empty::std_map_is_empty)
        .with_function("map_to_array", map_to_array::std_map_to_array)
        .with_function("map_get", map_get::std_map_get)
        .with_function("map_keys", map_keys::std_map_keys)
        .with_function("map_values", map_values::std_map_values)
        .with_function("map_clear", map_clear::std_map_clear)
        .with_function("map_merge", map_merge::std_map_merge)
}
