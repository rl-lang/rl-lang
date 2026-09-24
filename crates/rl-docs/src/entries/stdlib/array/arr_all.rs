use crate::entry::FnEntry;

pub static ARR_ALL: FnEntry = FnEntry {
    signature: "arr_all(arr, fn)",
    description: "true if every element satisfies the predicate",
    example: "get std::array::arr_all\n\narr_all([2, 4, 6], fn(int x) -> bool { return mod(x, 2) == 0 })?",
    expected_output: Some("true"),
    returns: "result[bool]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_any", "arr_filter"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
