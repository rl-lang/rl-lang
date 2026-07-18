use crate::entry::FnEntry;

pub static RAND_RANGE: FnEntry = FnEntry {
    signature: "rand_range(stop)",
    description: "returns a random int from 0 to stop (exclusive), stop must be greater than zero",
    example: "get std::random::rand_range\n\nrand_range(10)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `stop` is 0 or negative"),
    see_also: &["rand_int_range", "rand_range_step"],
    since: Some("v0.1.5"),
};
