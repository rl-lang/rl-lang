use crate::docs::entry::FnEntry;

pub static BIT_AND: FnEntry = FnEntry {
    signature: "bit_and(a, b)",
    description: "bitwise AND of two byte or int values",
    example: "get std::bitwise::bit_and\n\nbit_and(5, 3) // 1",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
