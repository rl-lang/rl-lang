use crate::entry::FnEntry;

pub static BUF_ADDR: FnEntry = FnEntry {
    signature: "__buf_addr(b)",
    description: "intrinsic: address of the buffer data for __syscall6 args, or 0 when empty; valid only until the next mutation",
    example: r#"get __buf_new, __buf_resize, __buf_addr from core

dec b = __buf_new()
__buf_resize(b, 8)
__buf_addr(b) != 0"#,
    expected_output: None,
    returns: "int",
    errors: None,
    see_also: &["__buf_resize", "__syscall6"],
    since: Some("v2.3.0"),
    deprecated: None,
    updated: Some("v2.3.0"),
};
