use crate::entry::FnEntry;

pub static CHECK: FnEntry = FnEntry {
    signature: "check(code)",
    description: "parses and type-checks a string of rl source code without running it; useful for validating snippets before evaluating them",
    example: "get std::rl::check\n\ncheck(\"dec int x = 1\") // ok(null)\ncheck(\"dec int x = \\\"a\\\"\") // err([\"...type mismatch...\"])",
    expected_output: None,
    returns: "Result[null]",
    errors: Some(
        "returns an array of type-checker error messages if the code fails to parse or type-check",
    ),
    see_also: &["lex", "eval"],
    since: None,
};
