use crate::entry::FnEntry;

pub static COS: FnEntry = FnEntry {
    signature: "cos(x)",
    description: "cosine of x in radians",
    example: "get std::math::cos\n\ncos(0.0)",
    expected_output: Some("1.0"),
    returns: "float",
    errors: None,
    see_also: &["sin", "tan"],
    since: Some("v0.1.5"),
};
