use crate::entry::FnEntry;

pub static RESULT_OK_VALUE: FnEntry = FnEntry {
    signature: "__result_ok_value(r)",
    description: "intrinsic: the payload of an ok value. call it when you are sure r is ok: no static questions beyond the result shape, but err payloads and non-results abort; RL code builds checked wrappers on top",
    example: r#"get __result_ok_value from core

dec string s = __result_ok_value(ok("payload"))"#,
    expected_output: None,
    returns: "T",
    errors: Some("called on err or a non-result aborts"),
    see_also: &["__result_err_value", "__type_of"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
