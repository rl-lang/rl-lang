use crate::entry::FnEntry;

pub static FIBONACCI: FnEntry = FnEntry {
    signature: "fibonacci(x)",
    description: "xth fibonacci number",
    example: "get std::math::fibonacci\n\nfibonacci(7) // 13",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
};
