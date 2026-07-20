use crate::entry::FnEntry;

pub static GCD: FnEntry = FnEntry {
    signature: "gcd(a, b)",
    description: "greatest common divisor of a and b",
    example: "get std::math::gcd\n\ngcd(12, 8)",
    expected_output: Some("4"),
    returns: "int",
    errors: None,
    see_also: &["lcm"],
    since: Some("v0.1.5"),
};
