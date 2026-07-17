use crate::entry::FnEntry;

pub static EVAL: FnEntry = FnEntry {
    signature: "eval(code)",
    description: "parses, resolves, and runs a string of rl source code in the current scope, so it can read and declare variables alongside the calling program; returns everything the code printed",
    example: r#"get std::rl::eval

dec int x = 5
eval("println(x + 1)")?"#,
    expected_output: Some("\"6\\n\""),
    returns: "result[string]",
    errors: Some("returns an error message if the code fails to parse, resolve, or run"),
    see_also: &["eval_isolated", "check"],
    since: Some("v0.1.5"),
};
