use crate::entry::FnEntry;

pub static EVAL_ISOLATED: FnEntry = FnEntry {
    signature: "eval_isolated(code)",
    description: "parses and runs a string of rl source code in a brand new interpreter with its own scope and stdlib, with no access to the calling program's variables; returns everything the code printed",
    example: r#"get std::rl::eval_isolated

eval_isolated("println(1 + 1)")"#,
    expected_output: Some("\"2\\n\""),
    returns: "result[string]",
    errors: Some("returns an error message if the code fails to parse or run"),
    see_also: &["eval", "check"],
    since: Some("v0.1.5"),
};
