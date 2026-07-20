use crate::entry::FnEntry;

pub static RAND_INT: FnEntry = FnEntry {
    signature: "rand_int()",
    description: "returns a random int across the full int range",
    example: "get std::random::rand_int\n\nrand_int() // e.g. 4650267523947147985",
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["rand_int_range"],
    since: Some("v0.1.5"),
};
