use crate::entry::FnEntry;

pub static BIT_IS_SET: FnEntry = FnEntry {
    signature: "bit_is_set(x, pos)",
    description: "returns true if the bit at position pos is set",
    example: "get std::bitwise::bit_is_set\n\nbit_is_set(0b1010, 3)?",
    expected_output: Some("true"),
    returns: "bool",
    errors: Some("Will return error if `x` is not a byte or int, or if `pos` is out of range."),
    see_also: &["bit_set", "bit_clear", "bit_toggle"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
