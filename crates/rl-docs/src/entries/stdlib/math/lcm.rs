use crate::entry::FnEntry;

pub static LCM: FnEntry = FnEntry {
    signature: "lcm(a, b)",
    description: "least common multiple of a and b",
    example: "get std::math::lcm\n\nlcm(4, 6)",
    expected_output: Some("12"),
    returns: "int",
    errors: None,
    see_also: &["gcd"],
    since: Some("v0.1.5"),
};
