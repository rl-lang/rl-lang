use crate::entry::FnEntry;

pub static ARR_REDUCE: FnEntry = FnEntry {
    signature: "arr_reduce(arr, fn, initial)",
    description: "folds the array into a single value using the callback and a starting accumulator",
    example: "get std::array::arr_reduce\n\narr_reduce([1, 2, 3, 4], fn(int acc, int x) -> int { return acc + x }, 0)?",
    expected_output: Some("10"),
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map", "arr_sum"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
