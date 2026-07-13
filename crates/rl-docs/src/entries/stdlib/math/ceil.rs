use crate::entry::FnEntry;

pub static CEIL: FnEntry = FnEntry {
    signature: "ceil(x)",
    description: "smallest integer greater than or equal to x",
    example: "get std::math::ceil\n\nceil(2.12) // 3.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
