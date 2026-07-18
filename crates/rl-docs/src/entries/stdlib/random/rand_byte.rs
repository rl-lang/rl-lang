use crate::entry::FnEntry;

pub static RAND_BYTE: FnEntry = FnEntry {
    signature: "rand_byte()",
    description: "returns a random value in 0 to 255",
    example: "get std::random::rand_byte\n\nrand_byte() // e.g. 110",
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["rand_bytes"],
    since: Some("v0.1.5"),
};
