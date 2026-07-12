use crate::entry::FnEntry;

pub static EVAL: FnEntry = FnEntry {
    signature: "eval(code)",
    description: "parses, resolves, and runs a string of rl source code in the current scope, so it can read and declare variables alongside the calling program; returns everything the code printed",
    example: "get std::rl::eval\n\ndec int x = 5\neval(\"println(x + 1)\") // ok(\"6\\n\")",
    expected_output: None,
    returns: "Result[string]",
    errors: Some("returns an error message if the code fails to parse, resolve, or run"),
    see_also: &["eval_isolated", "check"],
    since: None,
};
