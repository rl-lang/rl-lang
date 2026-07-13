use crate::entry::FnEntry;

pub static LCM: FnEntry = FnEntry {
    signature: "lcm(a, b)",
    description: "least common multiple of a and b",
    example: "get std::math::lcm\n\nlcm(4, 6) // 12",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
