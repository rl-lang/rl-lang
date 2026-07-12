use crate::docs::entry::FnEntry;

pub static BIT_NOT: FnEntry = FnEntry {
    signature: "bit_not(a)",
    description: "bitwise NOT (complement) of a byte or int value",
    example: "get std::bitwise::bit_not\n\nbit_not(0) // -1",
    expected_output: None,
    returns: "",
    errors: None,
    see_also: &[],
    since: None,
};
