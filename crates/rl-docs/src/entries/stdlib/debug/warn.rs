use crate::entry::FnEntry;

pub static WARN: FnEntry = FnEntry {
    signature: "warn(msg)",
    description: "prints a yellow-colored [warn] message to stderr",
    example: r#"get std::debug::warn

warn("disk space is low")"#,
    expected_output: Some("[warn] disk space is low"),
    returns: "null",
    errors: None,
    see_also: &["dbg"],
    since: Some("v2.1.0"),
    deprecated: None,
    updated: Some("v2.1.0"),
};
