use crate::entry::FnEntry;

pub static ARR_FIND: FnEntry = FnEntry {
    signature: "arr_find(arr, fn)",
    description: "returns the first element where the predicate returns true, or null if none match",
    example: "get std::array::arr_find\n\narr_find([1, 2, 3, 4], fn(int x) -> bool { return x > 2 })?",
    expected_output: Some("3"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_find_index", "arr_filter"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
