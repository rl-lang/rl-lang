use crate::entry::FnEntry;

pub static HYPOT: FnEntry = FnEntry {
    signature: "hypot(a, b)",
    description: "length of the hypotenuse given two sides: √(a² + b²)",
    example: "get std::math::hypot\n\nhypot(3.0, 4.0)",
    expected_output: Some("5.0"),
    returns: "float",
    errors: None,
    see_also: &["sqrt"],
    since: Some("v0.1.5"),
};
