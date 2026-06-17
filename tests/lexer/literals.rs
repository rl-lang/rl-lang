//! Tests for lexer token recognition
//! Covers literals, arithmetic operators compound assignment operators and punctuation

use super::super::common;
use rl_lang::lexer::tokentypes::TokenType;

/// Integer literal produces `NumberLiteral` with correct value and lexeme
#[test]
fn integer_literal() {
    let tokens = common::lex("42");
    assert_eq!(tokens[0].token, TokenType::NumberLiteral(42));
    assert_eq!(tokens[0].lexeme, "42");
}

/// Character literal produces `CharacterLiteral` with correct value and lexeme
#[test]
fn character_literal() {
    let tokens = common::lex("'x'");
    assert_eq!(tokens[0].token, TokenType::CharacterLiteral('x'));
    assert_eq!(tokens[0].lexeme, "'x'");
}

/// `+` produces `Plus` with correct lexeme
#[test]
fn plus_literal() {
    let tokens = common::lex("+");
    assert_eq!(tokens[0].token, TokenType::Plus);
    assert_eq!(tokens[0].lexeme, "+");
}

/// `/` produces `Slash` with correct lexeme
#[test]
fn slash_literal() {
    let tokens = common::lex("/");
    assert_eq!(tokens[0].token, TokenType::Slash);
    assert_eq!(tokens[0].lexeme, "/");
}

/// `*` produces `Star` with correct lexeme
#[test]
fn star_literal() {
    let tokens = common::lex("*");
    assert_eq!(tokens[0].token, TokenType::Star);
    assert_eq!(tokens[0].lexeme, "*");
}

/// `-` produces `Minus` with correct lexeme
#[test]
fn minus_literal() {
    let tokens = common::lex("-");
    assert_eq!(tokens[0].token, TokenType::Minus);
    assert_eq!(tokens[0].lexeme, "-");
}

/// `+=` produces `PlusEqual` not two separate tokens
#[test]
fn plus_equal_literal() {
    let tokens = common::lex("+=");
    assert_eq!(tokens[0].token, TokenType::PlusEqual);
    assert_eq!(tokens[0].lexeme, "+=");
}

/// `/=` produces `SlashEqual` not two separate tokens
#[test]
fn slash_equal_literal() {
    let tokens = common::lex("/=");
    assert_eq!(tokens[0].token, TokenType::SlashEqual);
    assert_eq!(tokens[0].lexeme, "/=");
}

/// `*=` produces `StarEqual` not two separate tokens
#[test]
fn star_equal_literal() {
    let tokens = common::lex("*=");
    assert_eq!(tokens[0].token, TokenType::StarEqual);
    assert_eq!(tokens[0].lexeme, "*=");
}

/// `-=` produces `MinusEqual` not two separate tokens
#[test]
fn minus_equal_literal() {
    let tokens = common::lex("-=");
    assert_eq!(tokens[0].token, TokenType::MinusEqual);
    assert_eq!(tokens[0].lexeme, "-=");
}

/// Float literal produces `FloatLiteral` with correct value and lexeme
#[test]
fn float_literal() {
    let tokens = common::lex("3.84");
    assert_eq!(tokens[0].token, TokenType::FloatLiteral(3.84));
    assert_eq!(tokens[0].lexeme, "3.84");
}

/// String literal strips quotes and produces `StringLiteral` with inner content
#[test]
fn string_literal() {
    let tokens = common::lex("\"hello\"");
    assert_eq!(
        tokens[0].token,
        TokenType::StringLiteral("hello".to_string())
    );
    assert_eq!(tokens[0].lexeme, "\"hello\"");
}

/// `true` keyword produces `BoolLiteral(true)`
#[test]
fn bool_literal_true() {
    let tokens = common::lex("true");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[0].lexeme, "true");
}

/// `false` keyword produces `BoolLiteral(false)`
#[test]
fn bool_literal_false() {
    let tokens = common::lex("false");
    assert_eq!(tokens[0].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[0].lexeme, "false");
}

/// `(` produces `LeftParen` with correct lexeme
#[test]
fn left_paren_literal() {
    let tokens = common::lex("(");
    assert_eq!(tokens[0].token, TokenType::LeftParen);
    assert_eq!(tokens[0].lexeme, "(");
}

/// `)` produces `RightParen` with correct lexeme
#[test]
fn right_paren_literal() {
    let tokens = common::lex(")");
    assert_eq!(tokens[0].token, TokenType::RightParen);
    assert_eq!(tokens[0].lexeme, ")");
}

/// `[` produces `LeftBracket` with correct lexeme
#[test]
fn left_bracket_literal() {
    let tokens = common::lex("[");
    assert_eq!(tokens[0].token, TokenType::LeftBracket);
    assert_eq!(tokens[0].lexeme, "[");
}

/// `]` produces `RightBracket` with correct lexeme
#[test]
fn right_bracket_literal() {
    let tokens = common::lex("]");
    assert_eq!(tokens[0].token, TokenType::RightBracket);
    assert_eq!(tokens[0].lexeme, "]");
}

/// `{` produces `LeftBrace` with correct lexeme
#[test]
fn left_brace_literal() {
    let tokens = common::lex("{");
    assert_eq!(tokens[0].token, TokenType::LeftBrace);
    assert_eq!(tokens[0].lexeme, "{");
}

/// `}` produces `RightBrace` with correct lexeme
#[test]
fn right_brace_literal() {
    let tokens = common::lex("}");
    assert_eq!(tokens[0].token, TokenType::RightBrace);
    assert_eq!(tokens[0].lexeme, "}");
}

/// `.` produces `Dot` not the start of a float
#[test]
fn dot_literal() {
    let tokens = common::lex(".");
    assert_eq!(tokens[0].token, TokenType::Dot);
    assert_eq!(tokens[0].lexeme, ".");
}

/// `..` produces a single `DotDot` not two `Dot` tokens
#[test]
fn dot_dot_literal() {
    let tokens = common::lex("..");
    assert_eq!(tokens[0].token, TokenType::DotDot);
    assert_eq!(tokens[0].lexeme, "..");
}

/// `:` produces `Colon` with correct lexeme
#[test]
fn colon_literal() {
    let tokens = common::lex(":");
    assert_eq!(tokens[0].token, TokenType::Colon);
    assert_eq!(tokens[0].lexeme, ":");
}

/// `::` produces a single `ColonColon` not two `Colon` tokens
#[test]
fn colon_colon_literal() {
    let tokens = common::lex("::");
    assert_eq!(tokens[0].token, TokenType::ColonColon);
    assert_eq!(tokens[0].lexeme, "::");
}

/// `;` produces `Semicolon` with correct lexeme
#[test]
fn semicolon_literal() {
    let tokens = common::lex(";");
    assert_eq!(tokens[0].token, TokenType::Semicolon);
    assert_eq!(tokens[0].lexeme, ";");
}

/// `,` produces `Comma` with correct lexeme
#[test]
fn comma_literal() {
    let tokens = common::lex(",");
    assert_eq!(tokens[0].token, TokenType::Comma);
    assert_eq!(tokens[0].lexeme, ",");
}

/// `!` produces `Bang` with correct lexeme
#[test]
fn bang_literal() {
    let tokens = common::lex("!");
    assert_eq!(tokens[0].token, TokenType::Bang);
    assert_eq!(tokens[0].lexeme, "!");
}
/// `=` produces `Assign` with correct lexeme
#[test]
fn assign_literal() {
    let tokens = common::lex("=");
    assert_eq!(tokens[0].token, TokenType::Assign);
    assert_eq!(tokens[0].lexeme, "=");
}

/// `>` produces `Greater` with correct lexeme
#[test]
fn greater_literal() {
    let tokens = common::lex(">");
    assert_eq!(tokens[0].token, TokenType::Greater);
    assert_eq!(tokens[0].lexeme, ">");
}

/// `<` produces `Less` with correct lexeme
#[test]
fn less_literal() {
    let tokens = common::lex("<");
    assert_eq!(tokens[0].token, TokenType::Less);
    assert_eq!(tokens[0].lexeme, "<");
}

/// `>=` produces `GreaterEqual` not two separate tokens
#[test]
fn greater_equal_literal() {
    let tokens = common::lex(">=");
    assert_eq!(tokens[0].token, TokenType::GreaterEqual);
    assert_eq!(tokens[0].lexeme, ">=");
}

/// `<=` produces `LessEqual` not two separate tokens
#[test]
fn less_equal_literal() {
    let tokens = common::lex("<=");
    assert_eq!(tokens[0].token, TokenType::LessEqual);
    assert_eq!(tokens[0].lexeme, "<=");
}

/// `==` produces `EqualEqual` not two separate tokens
#[test]
fn compare_literal() {
    let tokens = common::lex("==");
    assert_eq!(tokens[0].token, TokenType::Compare);
    assert_eq!(tokens[0].lexeme, "==");
}

/// `!=` produces `BangEqual` with correct lexeme
#[test]
fn bang_equal_literal() {
    let tokens = common::lex("!=");
    assert_eq!(tokens[0].token, TokenType::BangEqual);
    assert_eq!(tokens[0].lexeme, "!=");
}

/// `#` produces `Hash` with correct lexeme
#[test]
fn hash_literal() {
    let tokens = common::lex("#");
    assert_eq!(tokens[0].token, TokenType::Hash);
    assert_eq!(tokens[0].lexeme, "#");
}

/// `!#` produces `BangHash` not two separate tokens
#[test]
fn bang_hash_literal() {
    let tokens = common::lex("!#");
    assert_eq!(tokens[0].token, TokenType::BangHash);
    assert_eq!(tokens[0].lexeme, "!#");
}

/// `null` keyword produces `Null` with correct lexeme
#[test]
fn null_literal() {
    let tokens = common::lex("null");
    assert_eq!(tokens[0].token, TokenType::Null);
    assert_eq!(tokens[0].lexeme, "null");
}

/// Identifier produces `Identifier` with correct name as lexeme
#[test]
fn identifier_literal() {
    let tokens = common::lex("my_var");
    assert_eq!(tokens[0].token, TokenType::Identifier("my_var".to_string()));
    assert_eq!(tokens[0].lexeme, "my_var");
}

/// `Eof` is always the last token produced
#[test]
fn eof_is_last_token() {
    let tokens = common::lex("42");
    assert_eq!(tokens.last().unwrap().token, TokenType::Eof);
}

/// Newline produces `Newline` token
#[test]
fn newline_literal() {
    let tokens = common::lex("\n");
    assert_eq!(tokens[0].token, TokenType::Newline);
    assert_eq!(tokens[0].lexeme, "\n");
}
