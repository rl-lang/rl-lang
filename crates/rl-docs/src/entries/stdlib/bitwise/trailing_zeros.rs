use crate::entry::FnEntry;

pub static TRAILING_ZEROS: FnEntry = FnEntry {
    signature: "trailing_zeros(a)",
    description: "returns the number of trailing zero bits in a byte or int value",
    example: "get std::bitwise::trailing_zeros\n\ntrailing_zeros(8)?",
    expected_output: Some("3"),
    returns: "result[byte] or result[int]",
    errors: Some("Will return error if `a` is not a byte or int"),
    see_also: &["leading_zeros", "count_bits"],
    since: Some("v0.1.5"),
};
