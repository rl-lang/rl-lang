use crate::entry::FnEntry;

pub static ACOS: FnEntry = FnEntry {
    signature: "acos(x)",
    description: "arc cosine of x in radians",
    example: "get std::math::acos\n\nacos(1.0) // 0.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
