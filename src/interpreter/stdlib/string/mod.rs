mod bytes;
mod char_at;
mod chars;
mod concat;
mod is_empty;
mod repeat;
mod reverse;
mod slice;
mod to_lower;
mod to_upper;
mod trim;
mod trim_end;
mod trim_start;

use crate::interpreter::native::Module;

pub const KEYWORDS: &[&str] = &[
    "to_lower",
    "to_upper",
    "trim",
    "trim_end",
    "trim_start",
    "repeat",
    "is_empty",
    "concat",
    "char_at",
    "bytes",
    "chars",
    "slice",
];

pub fn module() -> Module {
    Module::new("str")
        .with_function("to_upper", to_upper::std_to_upper)
        .with_function("to_lower", to_lower::std_to_lower)
        .with_function("trim", trim::std_trim)
        .with_function("trim_end", trim_end::std_trim_end)
        .with_function("trim_start", trim_start::std_trim_start)
        .with_function("repeat", repeat::std_repeat)
        .with_function("is_empty", is_empty::std_is_empty)
        .with_raw_function("concat", concat::std_concat)
        .with_function("char_at", char_at::std_char_at)
        .with_function("bytes", bytes::std_bytes)
        .with_function("chars", chars::std_chars)
        .with_function("reverse", reverse::std_reverse)
        .with_function("slice", slice::std_slice)
}
