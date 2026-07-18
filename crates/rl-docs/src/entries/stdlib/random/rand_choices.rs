use crate::entry::FnEntry;

pub static RAND_CHOICES: FnEntry = FnEntry {
    signature: "rand_choices(arr, count)",
    description: "returns an array of count random elements from arr, with replacement",
    example: "get std::random::rand_choices\n\nrand_choices([1, 2, 3], 5)?",
    expected_output: None,
    returns: "result[arr[T]]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `count` is 0 or negative\n\nNote: unlike `rand_choice`, an empty `arr` is not validated here and\nwill panic at runtime rather than returning an error.",
    ),
    see_also: &["rand_choice", "rand_sample"],
    since: Some("v0.1.5"),
};
