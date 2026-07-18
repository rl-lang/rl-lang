use crate::entry::FnEntry;

pub static BIT_SHIFT_RIGHT: FnEntry = FnEntry {
    signature: "bit_shift_right(a, n)",
    description: "shifts a byte or int value right by n bits",
    example: "get std::bitwise::bit_shift_right\n\nbit_shift_right(10, 1)?",
    expected_output: Some("5"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` or `n` is not a byte or int.\n\nNote: the result type follows `a`'s type only, same as `bit_shift_left`.",
    ),
    see_also: &["bit_shift_left"],
    since: Some("v0.1.5"),
};
