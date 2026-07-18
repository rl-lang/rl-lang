use crate::entry::{FnEntry, StdEntry};

mod bytes;
mod char_at;
mod chars;
mod concat;
mod contains;
mod count;
mod ends_with;
mod format;
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
mod to_lower;
mod to_upper;
mod trim;
mod trim_end;
mod trim_start;

pub static STR: StdEntry = StdEntry {
    name: "str",
    description: "functions for string manipulation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &bytes::BYTES,
    &char_at::CHAR_AT,
    &chars::CHARS,
    &concat::CONCAT,
    &contains::CONTAINS,
    &count::COUNT,
    &ends_with::ENDS_WITH,
    &format::FORMAT,
    &index_of::INDEX_OF,
    &is_empty::IS_EMPTY,
    &join::JOIN,
    &pad_left::PAD_LEFT,
    &pad_right::PAD_RIGHT,
    &repeat::REPEAT,
    &replace::REPLACE,
    &reverse::REVERSE,
    &slice::SLICE,
    &split::SPLIT,
    &starts_with::STARTS_WITH,
    &to_lower::TO_LOWER,
    &to_upper::TO_UPPER,
    &trim::TRIM,
    &trim_end::TRIM_END,
    &trim_start::TRIM_START,
];
