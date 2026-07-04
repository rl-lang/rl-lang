use crate::docs::entry::{FnEntry, StdEntry};

pub static RES: StdEntry = StdEntry {
    name: "res",
    description: "functions for working with result[T] values (ok / err)",
    functions: FUNCTIONS,
    since: None,
    unstable: false,
};

static FUNCTIONS: &[&FnEntry] = &[
    &IS_OK,
    &IS_ERR,
    &RESULT_UNWRAP,
    &RESULT_UNWRAP_ERR,
    &RESULT_UNWRAP_OR,
    &RESULT_MAP,
    &RESULT_MAP_ERR,
];

static IS_OK: FnEntry = FnEntry {
    signature: "is_ok(r)",
    description: "true if r is an ok value",
    example: "get std::res::is_ok\n\ndec result[int] r = ok(42)\nprintln(is_ok(r))  // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static IS_ERR: FnEntry = FnEntry {
    signature: "is_err(r)",
    description: "true if r is an err value",
    example: "get std::res::is_err\n\ndec result[int] r = err(\"not found\")\nprintln(is_err(r))  // true",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static RESULT_UNWRAP: FnEntry = FnEntry {
    signature: "result_unwrap(r)",
    description: "returns the inner value of an ok result; panics at runtime if r is err",
    example: "get std::res::result_unwrap\n\ndec result[int] r = ok(10)\nprintln(result_unwrap(r))  // 10",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static RESULT_UNWRAP_ERR: FnEntry = FnEntry {
    signature: "result_unwrap_err(r)",
    description: "returns the inner value of an err result; panics at runtime if r is ok",
    example: "get std::res::result_unwrap_err\n\ndec result[int] r = err(\"oops\")\nprintln(result_unwrap_err(r))  // oops",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static RESULT_UNWRAP_OR: FnEntry = FnEntry {
    signature: "result_unwrap_or(r, default)",
    description: "returns the inner ok value, or default if r is err",
    example: "get std::res::result_unwrap_or\n\ndec result[int] r = err(\"fail\")\nprintln(result_unwrap_or(r, 0))  // 0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static RESULT_MAP: FnEntry = FnEntry {
    signature: "result_map(r, fn)",
    description: "applies a function to the ok value and returns a new ok; passes err through unchanged",
    example: "get std::res::result_map\n\ndec result[int] r = ok(5)\ndec result[int] doubled = result_map(r, fn(int x) -> int { return x * 2 })\nprintln(result_unwrap(doubled))  // 10",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};

static RESULT_MAP_ERR: FnEntry = FnEntry {
    signature: "result_map_err(r, fn)",
    description: "applies a function to the err value and returns a new err; passes ok through unchanged",
    example: "get std::res::result_map_err\n\ndec result[int] r = err(\"oops\")\ndec result[int] r2 = result_map_err(r, fn(string s) -> string { return concat(s, \"!\") })\nprintln(result_unwrap_err(r2))  // oops!",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
