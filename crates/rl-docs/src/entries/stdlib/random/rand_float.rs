use crate::entry::FnEntry;

pub static RAND_FLOAT: FnEntry = FnEntry {
    signature: "rand_float()",
    description: "returns a random float in [0.0, 1.0)",
    example: "get std::random::rand_float\n\nrand_float() // e.g. 0.3528",
    expected_output: None,
    returns: "float",
    errors: None,
    see_also: &["rand_float_range"],
    since: Some("v0.1.5"),
};
