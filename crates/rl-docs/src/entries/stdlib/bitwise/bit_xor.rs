use crate::entry::FnEntry;

pub static BIT_XOR: FnEntry = FnEntry {
    signature: "bit_xor(a, b)",
    description: "bitwise XOR of two byte or int values; both arguments must be the same type",
    example: "get std::bitwise::bit_xor\n\nbit_xor(5, 3) // 6",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
