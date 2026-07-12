use crate::entry::FnEntry;

pub static RL_VERSION: FnEntry = FnEntry {
    signature: "rl_version()",
    description: "returns the version of the rl-lang interpreter currently running",
    example: "get std::rl::rl_version\n\nrl_version() // \"0.1.4\"",
    expected_output: None,
    returns: "string",
    errors: None,
    see_also: &[],
    since: None,
};
