use crate::entry::FnEntry;

pub static EXP: FnEntry = FnEntry {
    signature: "exp(x)",
    description: "e raised to the power x",
    example: "get std::math::exp\n\nexp(1.0) // 2.718...",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
