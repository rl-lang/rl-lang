use crate::entry::FnEntry;

pub static RESULT_ERR_VALUE: FnEntry = FnEntry {
    signature: "__result_err_value(r)",
    description: "intrinsic: the payload of an err value. call it when you are sure r is err: no static questions beyond the result shape, but ok payloads and non-results abort; RL code builds checked wrappers on top",
    example: r#"get __result_err_value from core

dec string m = __result_err_value(err("boom"))"#,
    expected_output: None,
    returns: "T",
    errors: Some("called on ok or a non-result aborts"),
    see_also: &["__result_ok_value", "__type_of"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
