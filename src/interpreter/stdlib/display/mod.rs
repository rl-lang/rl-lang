pub mod len;
pub mod print;
pub mod println;

use crate::interpreter::native::Module;

pub fn module() -> Module {
    Module::new("display")
        .with_raw_function("print", print::std_print)
        .with_raw_function("println", println::std_println)
        .with_function("len", len::std_len)
}
