mod arr_sort;
mod len;

use crate::native::Module;

pub fn module() -> Module {
    Module::new("array")
        .with_function("len", len::std_len)
        .with_function("arr_sort", arr_sort::std_arr_sort)
}
