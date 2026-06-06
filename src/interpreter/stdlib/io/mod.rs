pub mod input;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["input"];

pub fn module() -> Module {
    Module::new("io").with_raw_function("input", input::std_input)
}
