use crate::entry::FnEntry;

pub static RESULT_UNWRAP_OR: FnEntry = FnEntry {
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
