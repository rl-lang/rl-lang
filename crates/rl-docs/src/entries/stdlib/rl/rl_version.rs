use crate::entry::FnEntry;

pub static RL_VERSION: FnEntry = FnEntry {
    signature: "rl_version()",
    description: "returns the version of the rl-lang interpreter currently running",
    example: r#"get std::rl::rl_version

rl_version()"#,
    expected_output: Some("0.1.5"),
    returns: "string",
    errors: None,
    see_also: &[],
    since: Some("v0.1.5"),
    deprecated: Some("std::rl is deprecated and may be removed in a future version"),
    updated: Some("v0.1.5"),
};
