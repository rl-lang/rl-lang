use crate::entry::FnEntry;

pub static ARR_FIND_INDEX: FnEntry = FnEntry {
    signature: "arr_find_index(arr, fn)",
    description: "returns the index of the first element where the predicate returns true, or -1 if none match",
    example: "get std::array::arr_find_index\n\narr_find_index([10, 20, 30], fn(int x) -> bool { return x == 20 })?",
    expected_output: Some("1"),
    returns: "result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare a `bool` return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_find", "arr_index_of"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
