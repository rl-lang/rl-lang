use crate::entry::FnEntry;

pub static ROTATE_RIGHT: FnEntry = FnEntry {
    signature: "rotate_right(x, n)",
    description: "rotates the bits of x right by n positions",
    example: "get std::bitwise::rotate_right\n\nrotate_right(0b0100, 2)?",
    expected_output: Some("0b0001"),
    returns: "byte or int",
    errors: Some("Will return error if `x` is not a byte or int, or if `n` is negative."),
    see_also: &["rotate_left", "bit_shift_left", "bit_shift_right"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
