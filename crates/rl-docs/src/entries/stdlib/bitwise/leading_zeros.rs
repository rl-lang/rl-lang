use crate::entry::FnEntry;

pub static LEADING_ZEROS: FnEntry = FnEntry {
    signature: "leading_zeros(a)",
    description: "returns the number of leading zero bits in a byte or int value",
    example: "get std::bitwise::leading_zeros\n\nleading_zeros(8)?",
    expected_output: Some("60"),
    returns: "result[byte] or result[int]",
    errors: Some(
        "Will return error if `a` is not a byte or int.\n\nNote: an `int` is 64 bits wide and a `byte` is 8 bits wide, so the same\nvalue produces very different counts depending on the argument's type\n(e.g. `leading_zeros(8)` is 60 for an int but 4 for a byte).",
    ),
    see_also: &["trailing_zeros", "count_bits"],
    since: Some("v0.1.5"),
};
