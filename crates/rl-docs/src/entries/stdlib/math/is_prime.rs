use crate::entry::FnEntry;

pub static IS_PRIME: FnEntry = FnEntry {
    signature: "is_prime(x)",
    description: "true if x is a prime number",
    example: "get std::math::is_prime\n\nis_prime(7)",
    expected_output: Some("true"),
    returns: "bool",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
