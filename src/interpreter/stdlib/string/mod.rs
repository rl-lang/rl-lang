mod bytes;
mod char_at;
mod chars;
mod concat;
mod contains;
mod count;
mod ends_with;
mod index_of;
mod is_empty;
mod join;
mod pad_left;
mod pad_right;
mod repeat;
mod replace;
mod reverse;
mod slice;
mod split;
mod starts_with;
mod string_to_float;
mod string_to_int;
mod to_lower;
mod to_string;
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
    "contains",
    "starts_with",
    "ends_with",
    "replace",
    "pad_left",
    "pad_right",
    "split",
    "join",
    "count",
    "index_of",
    "string_to_float",
    "string_to_int",
    "to_string",
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
        .with_function("contains", contains::std_cotains)
        .with_function("starts_with", starts_with::std_starts_with)
        .with_function("ends_with", ends_with::std_ends_with)
        .with_function("join", join::std_join)
        .with_function("split", split::std_split)
        .with_function("pad_right", pad_right::std_pad_right)
        .with_function("pad_left", pad_left::std_pad_left)
        .with_function("replace", replace::std_replace)
        .with_function("count", count::std_count)
        .with_function("index_of", index_of::std_index_of)
        .with_function("to_string", to_string::std_to_string)
        .with_function("string_to_float", string_to_float::std_parse_float)
        .with_function("string_to_int", string_to_int::std_parse_int)
}
