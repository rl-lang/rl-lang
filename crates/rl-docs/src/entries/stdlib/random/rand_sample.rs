use crate::entry::FnEntry;

pub static RAND_SAMPLE: FnEntry = FnEntry {
    signature: "rand_sample(arr, count)",
    description: "returns an array of count random elements from arr, without replacement (count must not exceed arr's length)",
    example: "get std::random::rand_sample\n\nrand_sample([1, 2, 3, 4], 2)?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `count` is 0 or negative\n- `count` is greater than the length of `arr`",
    ),
    see_also: &["rand_choices", "rand_shuffle"],
    since: Some("v0.1.5"),
};
