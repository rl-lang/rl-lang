use crate::entry::FnEntry;

pub static STR_GET_BYTE: FnEntry = FnEntry {
    signature: "__str_get_byte(s, idx)",
    description: "intrinsic: the byte at idx as a byte value. what hashing, encodings and binary protocols index; out-of-bounds aborts",
    example: r#"get __str_get_byte from core

dec byte b = __str_get_byte("hi", 0)"#,
    expected_output: None,
    returns: "byte",
    errors: Some("index out of bounds aborts"),
    see_also: &["__str_len", "__str_slice"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
