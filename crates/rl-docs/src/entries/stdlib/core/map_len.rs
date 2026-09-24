use crate::entry::FnEntry;

pub static MAP_LEN: FnEntry = FnEntry {
    signature: "__map_len(map)",
    description: "intrinsic: entry count of a map",
    example: r#"get __map_len from core

dec int n = __map_len({"a": 1})"#,
    expected_output: None,
    returns: "int",
    errors: Some("non-map aborts"),
    see_also: &["__map_keys", "__map_get"],
    since: Some("v2.2.0"),
    deprecated: None,
    updated: Some("v2.2.0"),
};
