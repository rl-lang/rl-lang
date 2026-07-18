use crate::entry::FnEntry;

pub static COUNT_BITS: FnEntry = FnEntry {
    signature: "count_bits(a)",
    description: "returns the number of set (1) bits in a byte or int value",
    example: "get std::bitwise::count_bits\n\ncount_bits(7)?",
    expected_output: Some("3"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` is not a byte or int.\n\nNote: the count is returned in `a`'s own type - a `byte` input gives a\n`result[byte]` count (max 8), an `int` input gives a `result[int]` count\n(max 64).",
    ),
    see_also: &["leading_zeros", "trailing_zeros"],
    since: Some("v0.1.5"),
};
