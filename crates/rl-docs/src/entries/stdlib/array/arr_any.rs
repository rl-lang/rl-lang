use crate::entry::FnEntry;

pub static ARR_ANY: FnEntry = FnEntry {
    signature: "arr_any(arr, fn)",
    description: "true if at least one element satisfies the predicate",
    example: "get std::array::arr_any\n\narr_any([1, 2, 3], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_all", "arr_find"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
