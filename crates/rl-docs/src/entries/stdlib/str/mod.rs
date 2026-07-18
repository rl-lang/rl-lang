use crate::entry::{FnEntry, StdEntry};

pub static STR: StdEntry = StdEntry {
    name: "str",
    description: "functions for string manipulation",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &BYTES,
    &CHAR_AT,
    &CHARS,
    &CONCAT,
    &CONTAINS,
    &COUNT,
    &ENDS_WITH,
    &INDEX_OF,
    &IS_EMPTY,
    &JOIN,
    &PAD_LEFT,
    &PAD_RIGHT,
    &REPEAT,
    &REPLACE,
    &REVERSE,
    &SLICE,
    &SPLIT,
    &STARTS_WITH,
    &TO_LOWER,
    &TO_UPPER,
    &TRIM,
    &TRIM_END,
    &TRIM_START,
    &FORMAT,
];

static BYTES: FnEntry = FnEntry {
    signature: "bytes(str)",
    description: "returns a byte array of the UTF-8 byte values of each character",
    example: "get std::str::bytes\n\nbytes(\"hi\")",
    expected_output: Some("[104, 105]"),
    returns: "arr[byte]",
    errors: None,
    see_also: &["chars"],
    since: Some("v0.1.5"),
};

static CHAR_AT: FnEntry = FnEntry {
    signature: "char_at(str, index)",
    description: "returns the character at the given index",
    example: "get std::str::char_at\n\nchar_at(\"hello\", 1)?",
    expected_output: Some("'e'"),
    returns: "result[char]",
    errors: Some(
        "Will return error on the following:\n\n- `index` is negative\n- `index` is out of bounds for `str`",
    ),
    see_also: &["chars", "slice"],
    since: Some("v0.1.5"),
};

static CHARS: FnEntry = FnEntry {
    signature: "chars(str)",
    description: "returns a char array of each character in the string",
    example: "get std::str::chars\n\nchars(\"hi\")",
    expected_output: Some("['h', 'i']"),
    returns: "arr[char]",
    errors: None,
    see_also: &["bytes", "char_at"],
    since: Some("v0.1.5"),
};

static CONCAT: FnEntry = FnEntry {
    signature: "concat(a, b, ...)",
    description: "concatenates any number of values into a single string",
    example: "get std::str::concat\n\nconcat(\"hello\", \" \", \"world\")",
    expected_output: Some("\"hello world\""),
    returns: "string",
    errors: None,
    see_also: &["join", "format"],
    since: Some("v0.1.5"),
};

static CONTAINS: FnEntry = FnEntry {
    signature: "contains(str, sub)",
    description: "true if str contains the substring sub",
    example: "get std::str::contains\n\ncontains(\"hello\", \"ell\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["starts_with", "ends_with", "index_of"],
    since: Some("v0.1.5"),
};

static COUNT: FnEntry = FnEntry {
    signature: "count(str, sub)",
    description: "returns the number of non-overlapping occurrences of sub in str",
    example: "get std::str::count\n\ncount(\"banana\", \"an\")",
    expected_output: Some("2"),
    returns: "int",
    errors: None,
    see_also: &["contains", "index_of"],
    since: Some("v0.1.5"),
};

static ENDS_WITH: FnEntry = FnEntry {
    signature: "ends_with(str, sub)",
    description: "true if str ends with sub",
    example: "get std::str::ends_with\n\nends_with(\"hello\", \"lo\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["starts_with", "contains"],
    since: Some("v0.1.5"),
};

static INDEX_OF: FnEntry = FnEntry {
    signature: "index_of(str, sub)",
    description: "returns the character index of the first occurrence of sub, or -1 if not found",
    example: "get std::str::index_of\n\nindex_of(\"hello\", \"ll\")",
    expected_output: Some("2"),
    returns: "int",
    errors: None,
    see_also: &["contains", "count"],
    since: Some("v0.1.5"),
};

static IS_EMPTY: FnEntry = FnEntry {
    signature: "is_empty(str)",
    description: "true if the string has no characters",
    example: "get std::str::is_empty\n\nis_empty(\"\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static JOIN: FnEntry = FnEntry {
    signature: "join(arr, delim)",
    description: "joins an array into a string with delim between each element",
    example: "get std::str::join\n\njoin([\"a\", \"b\", \"c\"], \"-\")?",
    expected_output: Some("\"a-b-c\""),
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` contains a function, lambda, or enclosure value\n\nNote: array elements that are themselves arrays, records, or tags are\nsilently dropped from the joined output rather than erroring or being\nstringified.",
    ),
    see_also: &["split", "concat"],
    since: Some("v0.1.5"),
};

static PAD_LEFT: FnEntry = FnEntry {
    signature: "pad_left(str, width, char)",
    description: "pads str on the left with char until the total length reaches width",
    example: "get std::str::pad_left\n\npad_left(\"5\", 3, '0')",
    expected_output: Some("\"005\""),
    returns: "string",
    errors: None,
    see_also: &["pad_right"],
    since: Some("v0.1.5"),
};

static PAD_RIGHT: FnEntry = FnEntry {
    signature: "pad_right(str, width, char)",
    description: "pads str on the right with char until the total length reaches width",
    example: "get std::str::pad_right\n\npad_right(\"hi\", 5, '.')",
    expected_output: Some("\"hi...\""),
    returns: "string",
    errors: None,
    see_also: &["pad_left"],
    since: Some("v0.1.5"),
};

static REPEAT: FnEntry = FnEntry {
    signature: "repeat(str, count)",
    description: "returns str repeated count times",
    example: "get std::str::repeat\n\nrepeat(\"ab\", 3)",
    expected_output: Some("\"ababab\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static REPLACE: FnEntry = FnEntry {
    signature: "replace(str, from, to)",
    description: "replaces all occurrences of from with to in str",
    example: "get std::str::replace\n\nreplace(\"foo bar foo\", \"foo\", \"baz\")",
    expected_output: Some("\"baz bar baz\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static REVERSE: FnEntry = FnEntry {
    signature: "reverse(str)",
    description: "returns str with characters in reverse order",
    example: "get std::str::reverse\n\nreverse(\"hello\")",
    expected_output: Some("\"olleh\""),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};

static SLICE: FnEntry = FnEntry {
    signature: "slice(str, start, end)",
    description: "returns a substring from start to end (exclusive)",
    example: "get std::str::slice\n\nslice(\"hello\", 1, 4)?",
    expected_output: Some("\"ell\""),
    returns: "result[string]",
    errors: Some(
        "Will return error on the following:\n\n- `start` or `end` is out of bounds for `str`\n\nNote: passing `end` < `start` is not validated as an error and will panic\nat runtime (integer underflow) rather than returning a `result[string]` err.",
    ),
    see_also: &["char_at"],
    since: Some("v0.1.5"),
};

static SPLIT: FnEntry = FnEntry {
    signature: "split(str, delim)",
    description: "splits str by delim and returns a string array",
    example: "get std::str::split\n\nsplit(\"a,b,c\", \",\")",
    expected_output: Some("[\"a\", \"b\", \"c\"]"),
    returns: "arr[string]",
    errors: None,
    see_also: &["join"],
    since: Some("v0.1.5"),
};

static STARTS_WITH: FnEntry = FnEntry {
    signature: "starts_with(str, sub)",
    description: "true if str starts with sub",
    example: "get std::str::starts_with\n\nstarts_with(\"hello\", \"he\")",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["ends_with", "contains"],
    since: Some("v0.1.5"),
};

static TO_LOWER: FnEntry = FnEntry {
    signature: "to_lower(str)",
    description: "returns str with all characters converted to lowercase",
    example: "get std::str::to_lower\n\nto_lower(\"HELLO\")",
    expected_output: Some("\"hello\""),
    returns: "string",
    errors: None,
    see_also: &["to_upper"],
    since: Some("v0.1.5"),
};

static TO_UPPER: FnEntry = FnEntry {
    signature: "to_upper(str)",
    description: "returns str with all characters converted to uppercase",
    example: "get std::str::to_upper\n\nto_upper(\"hello\")",
    expected_output: Some("\"HELLO\""),
    returns: "string",
    errors: None,
    see_also: &["to_lower"],
    since: Some("v0.1.5"),
};

static TRIM: FnEntry = FnEntry {
    signature: "trim(str)",
    description: "removes leading and trailing whitespace from str",
    example: "get std::str::trim\n\ntrim(\"  hi  \")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim_start", "trim_end"],
    since: Some("v0.1.5"),
};

static TRIM_END: FnEntry = FnEntry {
    signature: "trim_end(str)",
    description: "removes trailing whitespace from str",
    example: "get std::str::trim_end\n\ntrim_end(\"hi  \")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim", "trim_start"],
    since: Some("v0.1.5"),
};

static TRIM_START: FnEntry = FnEntry {
    signature: "trim_start(str)",
    description: "removes leading whitespace from str",
    example: "get std::str::trim_start\n\ntrim_start(\"  hi\")",
    expected_output: Some("\"hi\""),
    returns: "string",
    errors: None,
    see_also: &["trim", "trim_end"],
    since: Some("v0.1.5"),
};

static FORMAT: FnEntry = FnEntry {
    signature: "format(template, ...)",
    description: "replaces each \"{}\" in template with the corresponding argument, in order",
    example: "get std::str::format\n\nformat(\"{} is {}\", \"age\", 30)",
    expected_output: Some("\"age is 30\""),
    returns: "string",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `template` has more \"{}\" placeholders than arguments given\n- more arguments are given than \"{}\" placeholders used",
    ),
    see_also: &["concat"],
    since: Some("v0.1.5"),
};
