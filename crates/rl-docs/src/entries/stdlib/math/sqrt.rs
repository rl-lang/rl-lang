use crate::entry::FnEntry;

pub static SQRT: FnEntry = FnEntry {
    signature: "sqrt(x)",
    description: "square root of x",
    example: "get std::math::sqrt\n\nsqrt(4) // 2.0",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
