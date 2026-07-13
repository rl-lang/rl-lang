use crate::entry::FnEntry;

pub static GCD: FnEntry = FnEntry {
    signature: "gcd(a, b)",
    description: "greatest common divisor of a and b",
    example: "get std::math::gcd\n\ngcd(12, 8) // 4",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
