use crate::entry::FnEntry;

pub static RAND_BOOL: FnEntry = FnEntry {
    signature: "rand_bool()",
    description: "returns a random bool, using an internally randomized probability",
    example: "get std::random::rand_bool\n\nrand_bool() // e.g. true",
    expected_output: None,
    returns: "bool",
    errors: None,
    see_also: &["rand_bool_weighted"],
    since: Some("v0.1.5"),
};
