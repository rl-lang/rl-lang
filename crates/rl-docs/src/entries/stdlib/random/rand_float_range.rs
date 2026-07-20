use crate::entry::FnEntry;

pub static RAND_FLOAT_RANGE: FnEntry = FnEntry {
    signature: "rand_float_range(min, max)",
    description: "returns a random float in [min, max)",
    example: "get std::random::rand_float_range\n\nrand_float_range(1.0, 2.0)?",
    expected_output: None,
    returns: "result[float]",
    errors: Some("Will return error if `min` is greater than or equal to `max`"),
    see_also: &["rand_float"],
    since: Some("v0.1.5"),
};
