use crate::entry::FnEntry;

pub static RAND_BOOL_WEIGHTED: FnEntry = FnEntry {
    signature: "rand_bool_weighted(probability)",
    description: "returns a random bool that is true with the given probability; values are clamped to [0.0, 1.0] rather than erroring",
    example: "get std::random::rand_bool_weighted\n\nrand_bool_weighted(0.8) // e.g. true",
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["rand_bool"],
    since: Some("v0.1.5"),
};
