use crate::entry::FnEntry;

pub static RAND_CHOICE: FnEntry = FnEntry {
    signature: "rand_choice(arr)",
    description: "returns a random element from the array",
    example: "get std::random::rand_choice\n\nrand_choice([1, 2, 3])?",
    expected_output: None,
    returns: "result[T]",
    errors: Some(
        "Will return error on the following:\n\n- `arr` is not an array\n- `arr` is empty",
    ),
    see_also: &["rand_choices", "rand_sample"],
    since: Some("v0.1.5"),
};
