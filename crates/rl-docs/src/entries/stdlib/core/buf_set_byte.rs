use crate::entry::FnEntry;

pub static BUF_SET_BYTE: FnEntry = FnEntry {
    signature: "__buf_set_byte(b, idx, byte)",
    description: "intrinsic: overwrite the byte at idx in place (emitter fixups); out-of-bounds aborts",
    example: r#"get __buf_new, __buf_push_byte, __buf_set_byte from core

dec b = __buf_new()
__buf_push_byte(b, 72 as byte)
__buf_set_byte(b, 0, 104 as byte)"#,
    expected_output: None,
    returns: "null",
    errors: Some("index out of bounds aborts"),
    see_also: &["__buf_get_byte", "__buf_push_byte"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
