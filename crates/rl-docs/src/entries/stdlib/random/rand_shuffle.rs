use crate::entry::FnEntry;

pub static RAND_SHUFFLE: FnEntry = FnEntry {
    signature: "rand_shuffle(arr)",
    description: "returns the array with its elements in random order",
    example: "get std::random::rand_shuffle\n\nrand_shuffle([1, 2, 3, 4, 5])?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["rand_sample"],
    since: Some("v0.1.5"),
};
