mod format_time;
mod now;
mod time_arith;
mod time_parts;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "time_now",
    "time_now_ms",
    "format_time",
    "format_date_str",
    "format_time_str",
    "time_add",
    "time_diff",
    "time_parts",
];

pub fn module() -> Module {
    Module::new("time")
        .with_function("time_now", now::now)
        .with_function("time_now_ms", now::now_ms)
        .with_function("format_time", format_time::format_time)
        .with_function("format_date_str", format_time::date_str)
        .with_function("format_time_str", format_time::time_str)
        .with_function("time_add", time_arith::time_add)
        .with_function("time_diff", time_arith::time_diff)
        .with_function("time_parts", time_parts::time_parts)
}
