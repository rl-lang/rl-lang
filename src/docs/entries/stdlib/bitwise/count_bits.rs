use crate::docs::entry::FnEntry;

pub static COUNT_BITS: FnEntry = FnEntry {
    signature: "count_bits(a)",
    description: "returns the number of set (1) bits in a byte or int value",
    example: "get std::bitwise::count_bits\n\ncount_bits(7) // 3",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
