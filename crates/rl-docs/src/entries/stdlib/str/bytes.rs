use crate::entry::FnEntry;

pub static BYTES: FnEntry = FnEntry {
    signature: "bytes(str)",
    description: "returns a byte array of the UTF-8 byte values of each character",
    example: "get std::str::bytes\n\nbytes(\"hi\")",
    expected_output: Some("[104, 105]"),
    returns: "arr[byte]",
    errors: None,
    see_also: &["chars"],
    since: Some("v0.1.5"),
};
