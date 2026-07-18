use crate::entry::FnEntry;

pub static BIT_OR: FnEntry = FnEntry {
    signature: "bit_or(a, b)",
    description: "bitwise OR of two byte or int values",
    example: "get std::bitwise::bit_or\n\nbit_or(5, 3)?",
    expected_output: Some("7"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` or `b` is not a byte or int.\n\nNote: mixing `byte` and `int` is allowed and widens the result to `int`.",
    ),
    see_also: &["bit_and", "bit_xor", "bit_not"],
    since: Some("v0.1.5"),
};
