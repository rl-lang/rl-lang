use crate::entry::FnEntry;

pub static MIN: FnEntry = FnEntry {
    signature: "min(a, b)",
    description: "returns the smaller of a and b",
    example: "get std::math::min\n\nmin(4, 6) // 4",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
