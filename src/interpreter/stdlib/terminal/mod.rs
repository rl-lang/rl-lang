//! `std::term` - full terminal control via crossterm.

mod enter;
use crate::interpreter::native::Module;
pub const KEYWORDS: &[&str] = &[
    "term_enter",
];

pub fn module() -> Module {
    Module::new("term")
        .with_raw_function("term_enter", enter::func)
}
