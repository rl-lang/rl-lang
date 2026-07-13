use crate::entry::FnEntry;

pub static COS: FnEntry = FnEntry {
    signature: "cos(x)",
    description: "cosine of x in radians",
    example: "get std::math::cos\n\ncos(0.0) // 1.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
