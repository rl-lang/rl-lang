use crate::entry::FnEntry;

pub static BIT_OR: FnEntry = FnEntry {
    signature: "bit_or(a, b)",
    description: "bitwise OR of two byte or int values",
    example: "get std::bitwise::bit_or\n\nbit_or(5, 3) // 7",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
