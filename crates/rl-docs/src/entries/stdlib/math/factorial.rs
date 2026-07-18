use crate::entry::FnEntry;

pub static FACTORIAL: FnEntry = FnEntry {
    signature: "factorial(x)",
    description: "product of all integers from 1 to x",
    example: "get std::math::factorial\n\nfactorial(5)",
    expected_output: Some("120"),
    returns: "int",
    errors: None,
    see_also: &["fibonacci"],
    since: Some("v0.1.5"),
};
