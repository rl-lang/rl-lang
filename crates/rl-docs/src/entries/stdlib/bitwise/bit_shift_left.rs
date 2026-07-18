use crate::entry::FnEntry;

pub static BIT_SHIFT_LEFT: FnEntry = FnEntry {
    signature: "bit_shift_left(a, n)",
    description: "shifts a byte or int value left by n bits",
    example: "get std::bitwise::bit_shift_left\n\nbit_shift_left(5, 1)?",
    expected_output: Some("10"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` or `n` is not a byte or int.\n\nNote: the result type follows `a`'s type only - `n` can be a byte or int\nregardless of `a`'s type, and does not widen the result the way\n`bit_and`/`bit_or` do.",
    ),
    see_also: &["bit_shift_right"],
    since: Some("v0.1.5"),
};
