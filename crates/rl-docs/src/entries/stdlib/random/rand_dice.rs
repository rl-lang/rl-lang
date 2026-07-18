use crate::entry::FnEntry;

pub static RAND_DICE: FnEntry = FnEntry {
    signature: "rand_dice(sides)",
    description: "rolls a single die with the given number of sides and returns the result",
    example: "get std::random::rand_dice\n\nrand_dice(6)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `sides` is 0 or negative"),
    see_also: &["rand_dices"],
    since: Some("v0.1.5"),
};
