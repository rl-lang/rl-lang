use crate::entry::FnEntry;

pub static ARR_SORT_BY: FnEntry = FnEntry {
    signature: "arr_sort_by(arr, fn)",
    description: "sorts the array using a comparator callback that returns -1, 0, or 1",
    example: "get std::array::arr_sort_by\n\narr_sort_by([3, 1, 2], fn(int a, int b) -> int { return a - b })?",
    expected_output: Some("[1, 2, 3]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n- `fn` returns a non-int value\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.\n\nNote: this is an insertion sort, so it's O(n^2) - fine for small arrays,\nbut consider `arr_sort` (which is not comparator-based) for large\nnumeric arrays.",
    ),
    see_also: &["arr_sort"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
