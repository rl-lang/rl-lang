use crate::entry::FnEntry;

pub static ROTATE_LEFT: FnEntry = FnEntry {
    signature: "rotate_left(x, n)",
    description: "rotates the bits of x left by n positions",
    example: "get std::bitwise::rotate_left\n\nrotate_left(0b0001, 2)?",
    expected_output: Some("0b0100"),
    returns: "byte or int",
    errors: Some("Will return error if `x` is not a byte or int, or if `n` is negative."),
    see_also: &["rotate_right", "bit_shift_left", "bit_shift_right"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
