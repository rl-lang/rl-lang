use crate::docs::entry::{FnEntry, StdEntry};

pub static STR: StdEntry = StdEntry {
    name: "str",
    description: "functions for string manipulation",
    functions: FUNCTIONS,
    since: None,
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
    description: "returns an int array of the UTF-8 byte values of each character",
    example: "get std::str::bytes\n\nbytes(\"hi\") // [104, 105]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static CHAR_AT: FnEntry = FnEntry {
    signature: "char_at(str, index)",
    description: "returns the character at the given index",
    example: "get std::str::char_at\n\nchar_at(\"hello\", 1) // 'e'",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static CHARS: FnEntry = FnEntry {
    signature: "chars(str)",
    description: "returns a char array of each character in the string",
    example: "get std::str::chars\n\nchars(\"hi\") // ['h', 'i']",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static CONCAT: FnEntry = FnEntry {
    signature: "concat(a, b, ...)",
    description: "concatenates any number of values into a single string",
    example: "get std::str::concat\n\nconcat(\"hello\", \" \", \"world\") // hello world",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static CONTAINS: FnEntry = FnEntry {
    signature: "contains(str, sub)",
    description: "true if str contains the substring sub",
    example: "get std::str::contains\n\ncontains(\"hello\", \"ell\") // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static COUNT: FnEntry = FnEntry {
    signature: "count(str, sub)",
    description: "returns the number of non-overlapping occurrences of sub in str",
    example: "get std::str::count\n\ncount(\"banana\", \"an\") // 2",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static ENDS_WITH: FnEntry = FnEntry {
    signature: "ends_with(str, sub)",
    description: "true if str ends with sub",
    example: "get std::str::ends_with\n\nends_with(\"hello\", \"lo\") // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static INDEX_OF: FnEntry = FnEntry {
    signature: "index_of(str, sub)",
    description: "returns the character index of the first occurrence of sub, or -1 if not found",
    example: "get std::str::index_of\n\nindex_of(\"hello\", \"ll\") // 2",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static IS_EMPTY: FnEntry = FnEntry {
    signature: "is_empty(str)",
    description: "true if the string has no characters",
    example: "get std::str::is_empty\n\nis_empty(\"\") // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static JOIN: FnEntry = FnEntry {
    signature: "join(arr, delim)",
    description: "joins an array into a string with delim between each element",
    example: "get std::str::join\n\njoin([\"a\", \"b\", \"c\"], \"-\") // a-b-c",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PAD_LEFT: FnEntry = FnEntry {
    signature: "pad_left(str, width, char)",
    description: "pads str on the left with char until the total length reaches width",
    example: "get std::str::pad_left\n\npad_left(\"5\", 3, '0') // 005",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static PAD_RIGHT: FnEntry = FnEntry {
    signature: "pad_right(str, width, char)",
    description: "pads str on the right with char until the total length reaches width",
    example: "get std::str::pad_right\n\npad_right(\"hi\", 5, '.') // hi...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static REPEAT: FnEntry = FnEntry {
    signature: "repeat(str, count)",
    description: "returns str repeated count times",
    example: "get std::str::repeat\n\nrepeat(\"ab\", 3) // ababab",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static REPLACE: FnEntry = FnEntry {
    signature: "replace(str, from, to)",
    description: "replaces all occurrences of from with to in str",
    example: "get std::str::replace\n\nreplace(\"foo bar foo\", \"foo\", \"baz\") // baz bar baz",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static REVERSE: FnEntry = FnEntry {
    signature: "reverse(str)",
    description: "returns str with characters in reverse order",
    example: "get std::str::reverse\n\nreverse(\"hello\") // olleh",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static SLICE: FnEntry = FnEntry {
    signature: "slice(str, start, end)",
    description: "returns a substring from start to end (exclusive)",
    example: "get std::str::slice\n\nslice(\"hello\", 1, 4) // ell",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static SPLIT: FnEntry = FnEntry {
    signature: "split(str, delim)",
    description: "splits str by delim and returns a string array",
    example: "get std::str::split\n\nsplit(\"a,b,c\", \",\") // [\"a\", \"b\", \"c\"]",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static STARTS_WITH: FnEntry = FnEntry {
    signature: "starts_with(str, sub)",
    description: "true if str starts with sub",
    example: "get std::str::starts_with\n\nstarts_with(\"hello\", \"he\") // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TO_LOWER: FnEntry = FnEntry {
    signature: "to_lower(str)",
    description: "returns str with all characters converted to lowercase",
    example: "get std::str::to_lower\n\nto_lower(\"HELLO\") // hello",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TO_UPPER: FnEntry = FnEntry {
    signature: "to_upper(str)",
    description: "returns str with all characters converted to uppercase",
    example: "get std::str::to_upper\n\nto_upper(\"hello\") // HELLO",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TRIM: FnEntry = FnEntry {
    signature: "trim(str)",
    description: "removes leading and trailing whitespace from str",
    example: "get std::str::trim\n\ntrim(\"  hi  \") // hi",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TRIM_END: FnEntry = FnEntry {
    signature: "trim_end(str)",
    description: "removes trailing whitespace from str",
    example: "get std::str::trim_end\n\ntrim_end(\"hi  \") // hi",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static TRIM_START: FnEntry = FnEntry {
    signature: "trim_start(str)",
    description: "removes leading whitespace from str",
    example: "get std::str::trim_start\n\ntrim_start(\"  hi\") // hi",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static FORMAT: FnEntry = FnEntry {
    signature: "format(template, ...)",
    description: "replaces each \"{}\" in template with the corresponding argument, in order",
    example: "get std::str::format\n\nformat(\"{} is {}\", \"age\", 30) // age is 30",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
