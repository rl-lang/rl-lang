use crate::entry::FnEntry;

pub static RAND_STRING: FnEntry = FnEntry {
    signature: "rand_string(count)",
    description: "returns a random printable ascii string of the given length",
    example: "get std::random::rand_string\n\nrand_string(8)?",
    expected_output: None,
    returns: "result[string]",
    errors: Some(
        "Will return error if `count` is exactly 0.\n\nNote: same as `rand_bytes`, a negative `count` is not caught by the\ncurrent validation and silently returns an empty string instead of an\nerror.",
    ),
    see_also: &["rand_char", "rand_bytes"],
    since: Some("v0.1.5"),
};
