use crate::entry::FnEntry;

pub static CHECK: FnEntry = FnEntry {
    signature: "check(code)",
    description: "parses and type-checks a string of rl source code without running it; useful for validating snippets before evaluating them",
    example: r#"get std::rl::check

check("dec int x = 1")?
check("dec int x = \"a\"")?"#,
    expected_output: Some(
        r#"
null
err(" ... type mismatch ... ")
        "#,
    ),
    returns: "result[null]",
    errors: Some(
        "returns an array of type-checker error messages if the code fails to parse or type-check",
    ),
    see_also: &["lex", "eval"],
    since: Some("v0.1.5"),
};
