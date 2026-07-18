use crate::entry::FnEntry;

pub static RAND_DICES: FnEntry = FnEntry {
    signature: "rand_dices(count, sides)",
    description: "rolls count dice with the given number of sides and returns the individual results as an array",
    example: "get std::random::rand_dices\n\nrand_dices(3, 6)?",
    expected_output: None,
    returns: "result[arr[int]]",
    errors: Some(
        "Will return error on the following:\n\n- `count` is 0 or negative\n- `sides` is 0 or negative",
    ),
    see_also: &["rand_dice"],
    since: Some("v0.1.5"),
};
