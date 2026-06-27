//! `std::term` - full terminal control via crossterm.

mod clear;
mod clear_line;
mod enter;
mod leave;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "term_enter",
    "term_leave",
    "term_clear",
    "term_clear_line",
];

pub fn module() -> Module {
    Module::new("term")
        // enter / leave
        .with_raw_function("term_enter", enter::func)
        .with_raw_function("term_leave", leave::func)
        // clear
        .with_raw_function("term_clear", clear::func)
        .with_raw_function("term_clear_line", clear_line::func)
}
