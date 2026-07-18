use crate::entry::FnEntry;

pub static BIT_NOT: FnEntry = FnEntry {
    signature: "bit_not(a)",
    description: "bitwise NOT (complement) of a byte or int value",
    example: "get std::bitwise::bit_not\n\nbit_not(0)?",
    expected_output: Some("-1"),
    returns: "result[byte] or result[int]",
    errors: Some("Will return error if `a` is not a byte or int"),
    see_also: &["bit_and", "bit_or", "bit_xor"],
    since: Some("v0.1.5"),
};
