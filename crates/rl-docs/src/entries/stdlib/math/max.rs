use crate::entry::FnEntry;

pub static MAX: FnEntry = FnEntry {
    signature: "max(a, b)",
    description: "returns the larger of a and b",
    example: "get std::math::max\n\nmax(4, 6) // 6",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
