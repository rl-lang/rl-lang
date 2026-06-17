pub mod input;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["read", "read_int", "read_float"];

pub fn module() -> Module {
    Module::new("io")
        .with_raw_function("read", input::std_read)
        .with_raw_function("read_int", input::std_read_int)
        .with_raw_function("read_float", input::std_read_float)
}
