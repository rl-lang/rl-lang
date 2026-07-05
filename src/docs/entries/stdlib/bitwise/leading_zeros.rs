use crate::docs::entry::FnEntry;

pub static LEADING_ZEROS: FnEntry = FnEntry {
    signature: "leading_zeros(a)",
    description: "returns the number of leading zero bits in a byte or int value",
    example: "get std::bitwise::leading_zeros\n\nleading_zeros(8) // 60",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
