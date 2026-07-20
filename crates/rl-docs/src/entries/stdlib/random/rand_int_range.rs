use crate::entry::FnEntry;

pub static RAND_INT_RANGE: FnEntry = FnEntry {
    signature: "rand_int_range(min, max)",
    description: "returns a random int between min and max (inclusive)",
    example: "get std::random::rand_int_range\n\nrand_int_range(1, 6)?",
    expected_output: None,
    returns: "result[int]",
    errors: Some("Will return error if `min` is greater than or equal to `max`"),
    see_also: &["rand_int", "rand_range"],
    since: Some("v0.1.5"),
};
