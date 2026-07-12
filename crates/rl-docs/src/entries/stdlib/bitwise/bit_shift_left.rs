use crate::entry::FnEntry;

pub static BIT_SHIFT_LEFT: FnEntry = FnEntry {
    signature: "bit_shift_left(a, n)",
    description: "shifts a byte or int value left by n bits",
    example: "get std::bitwise::bit_shift_left\n\nbit_shift_left(5, 1) // 10",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
