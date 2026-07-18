use crate::entry::FnEntry;

pub static BIT_AND: FnEntry = FnEntry {
    signature: "bit_and(a, b)",
    description: "bitwise AND of two byte or int values",
    example: "get std::bitwise::bit_and\n\nbit_and(5, 3)?",
    expected_output: Some("1"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` or `b` is not a byte or int.\n\nNote: mixing `byte` and `int` is allowed and widens the result to `int`.",
    ),
    see_also: &["bit_or", "bit_xor", "bit_not"],
    since: Some("v0.1.5"),
};
