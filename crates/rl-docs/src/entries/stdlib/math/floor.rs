use crate::entry::FnEntry;

pub static FLOOR: FnEntry = FnEntry {
    signature: "floor(x)",
    description: "largest integer less than or equal to x",
    example: "get std::math::floor\n\nfloor(1.23) // 1.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
