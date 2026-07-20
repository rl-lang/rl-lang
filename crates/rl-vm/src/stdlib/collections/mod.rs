mod set_add;
mod set_contains;
mod set_is_empty;
mod set_len;
mod set_remove;
mod set_to_array;

use crate::native::Module;

pub fn module() -> Module {
    Module::new("collections")
        .with_function("set_add", set_add::std_set_add)
        .with_function("set_remove", set_remove::std_set_remove)
        .with_function("set_contains", set_contains::std_set_contains)
        .with_function("set_len", set_len::std_set_len)
        .with_function("set_is_empty", set_is_empty::std_set_is_empty)
        .with_function("set_to_array", set_to_array::std_set_to_array)
}
