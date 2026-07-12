use crate::docs::entry::FnEntry;

pub static BIT_SHIFT_RIGHT: FnEntry = FnEntry {
    signature: "bit_shift_right(a, n)",
    description: "shifts a byte or int value right by n bits",
    example: "get std::bitwise::bit_shift_right\n\nbit_shift_right(10, 1) // 5",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
