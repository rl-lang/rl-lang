use crate::entry::{FnEntry, StdEntry};

mod bytes;
mod char_at;
mod chars;
mod concat;
mod contains;
mod count;
mod dedent;
mod diff_lines;
mod ends_with;
mod format;
mod index_of;
mod indent;
mod is_alpha;
mod is_empty;
mod is_numeric;
mod is_whitespace;
mod join;
mod last_index_of;
mod lines;
mod pad_left;
mod pad_right;
mod repeat;
mod replace;
mod reverse;
mod slice;
mod split;
mod split_once;
mod starts_with;
mod strip_prefix;
mod strip_suffix;
mod to_lower;
mod to_upper;
mod trim;
mod trim_end;
mod trim_start;
mod unicode_category;
mod wrap;

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
    &dedent::DEDENT,
    &diff_lines::DIFF_LINES,
    &ends_with::ENDS_WITH,
    &format::FORMAT,
    &index_of::INDEX_OF,
    &indent::INDENT,
    &is_alpha::IS_ALPHA,
    &is_empty::IS_EMPTY,
    &is_numeric::IS_NUMERIC,
    &is_whitespace::IS_WHITESPACE,
    &join::JOIN,
    &last_index_of::LAST_INDEX_OF,
    &lines::LINES,
    &pad_left::PAD_LEFT,
    &pad_right::PAD_RIGHT,
    &repeat::REPEAT,
    &replace::REPLACE,
    &reverse::REVERSE,
    &slice::SLICE,
    &split::SPLIT,
    &split_once::SPLIT_ONCE,
    &starts_with::STARTS_WITH,
    &strip_prefix::STRIP_PREFIX,
    &strip_suffix::STRIP_SUFFIX,
    &to_lower::TO_LOWER,
    &to_upper::TO_UPPER,
    &trim::TRIM,
    &trim_end::TRIM_END,
    &trim_start::TRIM_START,
    &unicode_category::UNICODE_CATEGORY,
    &wrap::WRAP,
];
