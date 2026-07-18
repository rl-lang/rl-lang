use crate::entry::FnEntry;

pub static ACOS: FnEntry = FnEntry {
    signature: "acos(x)",
    description: "arc cosine of x in radians",
    example: "get std::math::acos\n\nacos(1.0)",
    expected_output: Some("0.0"),
    returns: "float",
    errors: None,
    see_also: &["asin", "atan"],
    since: Some("v0.1.5"),
};
