use crate::entry::FnEntry;

pub static ARR_PRODUCT: FnEntry = FnEntry {
    signature: "arr_product(arr)",
    description: "returns the product of all elements in an int or float array",
    example: "get std::array::arr_product\n\narr_product([1, 2, 3, 4])?",
    expected_output: Some("24"),
    returns: "result[int] or result[float]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is not an int or float array\n\nUnlike `arr_max`/`arr_min`, an empty array is not rejected - it returns\n`1` (int) or `1.0` (float), the empty product.",
    ),
    see_also: &["arr_sum"],
    since: Some("v0.1.5"),
    deprecated: None,
    updated: Some("v0.1.5"),
};
