use crate::docs::entry::FnEntry;

pub static LEX: FnEntry = FnEntry {
    signature: "lex(code)",
    description: "tokenizes a string of rl source code and returns each token as a (kind, lexeme, line) tuple, without parsing or running it",
    example: "get std::rl::lex\n\nlex(\"dec int x = 1\") // [(\"Dec\", \"dec\", 1), (\"Int\", \"int\", 1), (\"Identifier\", \"x\", 1), (\"Equal\", \"=\", 1), (\"Integer\", \"1\", 1)]",
    expected_output: None,
    returns: "Result[arr[(string, string, int)]]",
    errors: Some("returns an error if the source has invalid tokens (e.g. an unterminated string)"),
    see_also: &["check", "eval"],
    since: None,
};
