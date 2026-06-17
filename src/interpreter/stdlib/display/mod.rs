pub mod len;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &["len"];

pub fn module() -> Module {
    Module::new("display").with_function("len", len::std_len)
}
