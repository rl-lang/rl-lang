use crate::entry::FnEntry;

pub static RAND_CHAR: FnEntry = FnEntry {
    signature: "rand_char()",
    description: "returns a random printable ascii character (32 to 126)",
    example: "get std::random::rand_char\n\nrand_char() // e.g. 'c'",
    expected_output: None,
    returns: "char",
    errors: None,
    see_also: &["rand_string"],
    since: Some("v0.1.5"),
};
