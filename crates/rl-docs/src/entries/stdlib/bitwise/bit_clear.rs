use crate::entry::FnEntry;

pub static BIT_CLEAR: FnEntry = FnEntry {
    signature: "bit_clear(x, pos)",
    description: "clears the bit at position pos to 0",
    example: "get std::bitwise::bit_clear\n\nbit_clear(0b1010, 1)?",
    expected_output: Some("0b1000"),
    returns: "byte or int",
    errors: Some("Will return error if `x` is not a byte or int, or if `pos` is out of range."),
    see_also: &["bit_set", "bit_toggle", "bit_is_set"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
