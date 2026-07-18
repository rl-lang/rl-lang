use crate::entry::{FnEntry, StdEntry};

pub static RES: StdEntry = StdEntry {
    name: "res",
    description: "functions for working with result[T] values (ok / err)",
    functions: FUNCTIONS,
    since: Some("v0.1.5"),
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
    example: "get std::res::is_ok\n\ndec result[int] r = ok(42)\nis_ok(r)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_err"],
    since: Some("v0.1.5"),
};

static IS_ERR: FnEntry = FnEntry {
    signature: "is_err(r)",
    description: "true if r is an err value",
    example: "get std::res::is_err\n\ndec result[int] r = err(\"not found\")\nis_err(r)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &["is_ok"],
    since: Some("v0.1.5"),
};

static RESULT_UNWRAP: FnEntry = FnEntry {
    signature: "result_unwrap(r)",
    description: "returns the inner value of an ok result; panics at runtime if r is err",
    example: "get std::res::result_unwrap\n\ndec result[int] r = ok(10)\nresult_unwrap(r)",
    expected_output: Some("10"),
    returns: "T",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `r` is `err(..)`\n- `r` is not a result at all",
    ),
    see_also: &["result_unwrap_err", "result_unwrap_or", "is_ok"],
    since: Some("v0.1.5"),
};

static RESULT_UNWRAP_ERR: FnEntry = FnEntry {
    signature: "result_unwrap_err(r)",
    description: "returns the inner value of an err result; panics at runtime if r is ok",
    example: "get std::res::result_unwrap_err\n\ndec result[int] r = err(\"oops\")\nresult_unwrap_err(r)",
    expected_output: Some("\"oops\""),
    returns: "T",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) on the following:\n\n- `r` is `ok(..)`\n- `r` is not a result at all",
    ),
    see_also: &["result_unwrap", "is_err"],
    since: Some("v0.1.5"),
};

static RESULT_UNWRAP_OR: FnEntry = FnEntry {
    signature: "result_unwrap_or(r, default)",
    description: "returns the inner ok value, or default if r is err",
    example: "get std::res::result_unwrap_or\n\ndec result[int] r = err(\"fail\")\nresult_unwrap_or(r, 0)",
    expected_output: Some("0"),
    returns: "T",
    errors: Some(
        "Will panic at runtime (not a catchable `result[..]` err) if `r` is not a\nresult at all. Unlike `result_unwrap`, an `err(..)` value does not panic -\nit returns `default` instead, since that's the documented purpose of\nthis function.",
    ),
    see_also: &["result_unwrap"],
    since: Some("v0.1.5"),
};

static RESULT_MAP: FnEntry = FnEntry {
    signature: "result_map(r, fn)",
    description: "applies a function to the ok value and returns a new ok; passes err through unchanged",
    example: "get std::res::result_map\n\ndec result[int] r = ok(5)\ndec result[int] doubled = result_map(r, fn(int x) -> int { return x * 2 })\nresult_unwrap(doubled)",
    expected_output: Some("10"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `r` is not a result\n- calling `fn` fails (the failure message is wrapped as the err value,\n  rather than propagating as an interpreter error)",
    ),
    see_also: &["result_map_err", "result_unwrap"],
    since: Some("v0.1.5"),
};

static RESULT_MAP_ERR: FnEntry = FnEntry {
    signature: "result_map_err(r, fn)",
    description: "applies a function to the err value and returns a new err; passes ok through unchanged",
    example: "get std::res::result_map_err\n\ndec result[int] r = err(\"oops\")\ndec result[int] r2 = result_map_err(r, fn(string s) -> string { return concat(\"ERR: \", s) })\nresult_unwrap_err(r2)",
    expected_output: Some("\"ERR: oops\""),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `r` is not a result\n- calling `fn` fails (the failure message is wrapped as the err value,\n  rather than propagating as an interpreter error)",
    ),
    see_also: &["result_map", "result_unwrap_err"],
    since: Some("v0.1.5"),
};
