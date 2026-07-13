use crate::entry::FnEntry;

pub static POW: FnEntry = FnEntry {
    signature: "pow(a, b)",
    description: "raises a to the power of b",
    example: "get std::math::pow\n\npow(2, 2) // 4.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
