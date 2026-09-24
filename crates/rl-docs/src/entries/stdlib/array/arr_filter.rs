use crate::entry::FnEntry;

pub static ARR_FILTER: FnEntry = FnEntry {
    signature: "arr_filter(arr, fn)",
    description: "returns a new array containing only elements where the predicate returns true",
    example: "get std::array::arr_filter\n\narr_filter([1, 2, 3, 4], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("[3, 4]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map", "arr_find"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
