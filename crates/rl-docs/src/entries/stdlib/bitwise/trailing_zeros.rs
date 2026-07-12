use crate::entry::FnEntry;

pub static TRAILING_ZEROS: FnEntry = FnEntry {
    signature: "trailing_zeros(a)",
    description: "returns the number of trailing zero bits in a byte or int value",
    example: "get std::bitwise::trailing_zeros\n\ntrailing_zeros(8) // 3",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
