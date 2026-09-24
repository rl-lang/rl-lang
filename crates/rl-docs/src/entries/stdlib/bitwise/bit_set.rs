use crate::entry::FnEntry;

pub static BIT_SET: FnEntry = FnEntry {
    signature: "bit_set(x, pos)",
    description: "sets the bit at position pos to 1",
    example: "get std::bitwise::bit_set\n\nbit_set(0b1000, 1)?",
    expected_output: Some("0b1010"),
    returns: "byte or int",
    errors: Some("Will return error if `x` is not a byte or int, or if `pos` is out of range."),
    see_also: &["bit_clear", "bit_toggle", "bit_is_set"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
