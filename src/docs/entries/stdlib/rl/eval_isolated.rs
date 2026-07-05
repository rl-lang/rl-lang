use crate::docs::entry::FnEntry;

pub static EVAL_ISOLATED: FnEntry = FnEntry {
    signature: "eval_isolated(code)",
    description: "parses and runs a string of rl source code in a brand new interpreter with its own scope and stdlib, with no access to the calling program's variables; returns everything the code printed",
    example: "get std::rl::eval_isolated\n\neval_isolated(\"println(1 + 1)\") // ok(\"2\\n\")",
    expected_output: None,
    returns: "Result[string]",
    errors: Some("returns an error message if the code fails to parse or run"),
    see_also: &["eval", "check"],
    since: None,
};
