use crate::entry::FnEntry;

pub static ROUND: FnEntry = FnEntry {
    signature: "round(x)",
    description: "rounds x to the nearest integer",
    example: "get std::math::round\n\nround(2.2) // 2.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
