use crate::entry::FnEntry;

pub static TYPE_OF: FnEntry = FnEntry {
    signature: "__type_of(v)",
    description: "intrinsic: the runtime type name of a value as a string. what generic RL code branches on",
    example: r#"get __type_of from core

dec string t = __type_of(42)"#,
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &["__abort"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
