use crate::entry::FnEntry;

pub static BIT_XOR: FnEntry = FnEntry {
    signature: "bit_xor(a, b)",
    description: "bitwise XOR of two byte or int values; both arguments must be the same type",
    example: "get std::bitwise::bit_xor\n\nbit_xor(5, 3)?",
    expected_output: Some("6"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error on the following:\n\n- `a` or `b` is not a byte or int\n- `a` and `b` are different types (unlike `bit_and`/`bit_or`, mixing\n  `byte` and `int` is not allowed here)",
    ),
    see_also: &["bit_and", "bit_or"],
    since: Some("v0.1.5"),
};
