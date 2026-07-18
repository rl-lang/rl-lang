use crate::entry::FnEntry;

pub static FIBONACCI: FnEntry = FnEntry {
    signature: "fibonacci(x)",
    description: "xth fibonacci number",
    example: "get std::math::fibonacci\n\nfibonacci(7)",
    expected_output: Some("13"),
    returns: "int",
    errors: None,
    see_also: &["factorial"],
    since: Some("v0.1.5"),
};
