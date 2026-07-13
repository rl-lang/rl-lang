use crate::entry::FnEntry;

pub static RADIANS: FnEntry = FnEntry {
    signature: "radians(x)",
    description: "convert degrees to radians",
    example: "get std::math::radians\n\nradians(180.0) // 3.14159...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
