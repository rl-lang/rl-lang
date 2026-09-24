use crate::entry::FnEntry;

pub static ARR_FLAT_MAP: FnEntry = FnEntry {
    signature: "arr_flat_map(arr, fn)",
    description: "maps each element to an array via the callback then flattens the results one level",
    example: "get std::array::arr_flat_map\n\narr_flat_map([1, 2, 3], fn(int x) -> arr[int] { return [x, x * 10] })?",
    expected_output: Some("[1, 10, 2, 20, 3, 30]"),
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or does not declare an array return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err. Also,\nif `fn` returns a non-array value for some element (which the return-type\ncheck should normally prevent), that element is silently dropped rather\nthan erroring.",
    ),
    see_also: &["arr_map", "arr_flatten"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
