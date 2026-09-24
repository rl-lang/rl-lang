use crate::entry::FnEntry;

pub static ARR_MAP: FnEntry = FnEntry {
    signature: "arr_map(arr, fn)",
    description: "returns a new array with each element transformed by the callback",
    example: "get std::array::arr_map\n\narr_map([1, 2, 3], fn(int x) -> int { return x * 2 })?",
    expected_output: Some("[2, 4, 6]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.\n\nNote: the resulting array's element type is inferred from the first\nmapped element - `arr_map` does not check `fn`'s declared return type\nthe way `arr_filter`/`arr_flat_map`/`arr_for_each` do.",
    ),
    see_also: &["arr_filter", "arr_flat_map", "arr_reduce"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
