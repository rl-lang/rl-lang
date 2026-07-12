//! `std::debug` - assertions, panics, and debug utilities.

mod assert;
mod assert_approx_eq;
mod assert_eq;
mod assert_ge;
mod assert_gt;
mod assert_le;
mod assert_lt;
mod assert_ne;
mod bench;
pub mod common;
mod dbg;
mod panic;
mod todo;
mod type_of;
mod unreachable;

pub const KEYWORDS: &[&str] = &[
    "assert",
    "assert_eq",
    "assert_ne",
    "assert_lt",
    "assert_le",
    "assert_gt",
    "assert_ge",
    "assert_approx_eq",
    "panic",
    "unreachable",
    "todo",
    "dbg",
    "type_of",
    "bench",
];

pub fn module() -> crate::interpreter::native::Module {
    use crate::interpreter::native::Module;
    Module::new("debug")
        .with_raw_function("assert", assert::func)
        .with_raw_function("assert_eq", assert_eq::func)
        .with_raw_function("assert_ne", assert_ne::func)
        .with_raw_function("assert_lt", assert_lt::func)
        .with_raw_function("assert_le", assert_le::func)
        .with_raw_function("assert_gt", assert_gt::func)
        .with_raw_function("assert_ge", assert_ge::func)
        .with_raw_function("assert_approx_eq", assert_approx_eq::func)
        .with_raw_function("panic", panic::func)
        .with_raw_function("unreachable", unreachable::func)
        .with_raw_function("todo", todo::func)
        .with_function("dbg", dbg::func)
        .with_function("type_of", type_of::func)
        .with_function("bench", bench::func)
}
