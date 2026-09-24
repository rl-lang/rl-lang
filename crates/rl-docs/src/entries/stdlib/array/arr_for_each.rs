use crate::entry::FnEntry;

pub static ARR_FOR_EACH: FnEntry = FnEntry {
    signature: "arr_for_each(arr, fn)",
    description: "calls the callback on every element for side effects, returns null",
    example: "get std::array::arr_for_each\n\narr_for_each([1, 2, 3], fn(int x) { println(x) })?",
    expected_output: None,
    returns: "result[null]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `fn` is not a function/lambda, or declares a non-null return type\n\nAn error raised inside `fn` while it runs propagates as an\ninterpreter-level runtime error, not a catchable `result[..]` err.",
    ),
    see_also: &["arr_map"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
