use crate::entry::FnEntry;

pub static BUF_GET_BYTE: FnEntry = FnEntry {
    signature: "__buf_get_byte(b, idx)",
    description: "intrinsic: the byte at idx as a byte value; out-of-bounds aborts",
    example: r#"get __buf_new, __buf_push_byte, __buf_get_byte from core

dec b = __buf_new()
__buf_push_byte(b, 72 as byte)
dec byte x = __buf_get_byte(b, 0)"#,
    expected_output: None,
    returns: "byte",
    errors: Some("index out of bounds aborts"),
    see_also: &["__buf_set_byte", "__buf_len"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
