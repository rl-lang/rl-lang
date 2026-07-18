use crate::entry::FnEntry;

pub static EXP: FnEntry = FnEntry {
    signature: "exp(x)",
    description: "e raised to the power x",
    example: "get std::math::exp\n\nexp(1.0)",
    expected_output: Some("2.718281828459045"),
    returns: "float",
    errors: None,
    see_also: &["pow", "log"],
    since: Some("v0.1.5"),
};
