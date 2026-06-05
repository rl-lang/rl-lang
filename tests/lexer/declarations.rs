//! Tests for lexer declaration recognition.
//! Covers all variable and constant declaration forms, checking every token in each declaration.

use super::super::common;
use rl_lang::lexer::tokentypes::TokenType;

// ------------------------------------------------------------
// dec declarations
// ------------------------------------------------------------

/// `dec int x = 0` — checks every token in order
#[test]
fn dec_int_declaration() {
    let tokens = common::lex("dec int x = 0");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Int);
    assert_eq!(tokens[1].lexeme, "int");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::NumberLiteral(0));
    assert_eq!(tokens[4].lexeme, "0");
}

/// `dec float x = 0.0` — checks every token in order
#[test]
fn dec_float_declaration() {
    let tokens = common::lex("dec float x = 0.0");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Float);
    assert_eq!(tokens[1].lexeme, "float");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::FloatLiteral(0.0));
    assert_eq!(tokens[4].lexeme, "0.0");
}

/// `dec bool x = false` — checks every token in order
#[test]
fn dec_bool_declaration() {
    let tokens = common::lex("dec bool x = false");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Bool);
    assert_eq!(tokens[1].lexeme, "bool");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[4].lexeme, "false");
}

/// `dec string x = "hello"` — checks every token in order
#[test]
fn dec_string_declaration() {
    let tokens = common::lex("dec string x = \"hello\"");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::String);
    assert_eq!(tokens[1].lexeme, "string");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(
        tokens[4].token,
        TokenType::StringLiteral("hello".to_string())
    );
    assert_eq!(tokens[4].lexeme, "\"hello\"");
}

/// `dec char x = 'a'` — checks every token in order
#[test]
fn dec_char_declaration() {
    let tokens = common::lex("dec char x = 'a'");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Char);
    assert_eq!(tokens[1].lexeme, "char");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::CharacterLiteral('a'));
    assert_eq!(tokens[4].lexeme, "'a'");
}

/// `dec arr[int] x = [1, 2, 3]` — checks every token in order
#[test]
fn dec_arr_int_declaration() {
    let tokens = common::lex("dec arr[int] x = [1, 2, 3]");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Int);
    assert_eq!(tokens[3].lexeme, "int");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::NumberLiteral(1));
    assert_eq!(tokens[8].lexeme, "1");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::NumberLiteral(2));
    assert_eq!(tokens[10].lexeme, "2");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::NumberLiteral(3));
    assert_eq!(tokens[12].lexeme, "3");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `dec arr[float] x = [1.0, 2.0, 3.0]` — checks every token in order
#[test]
fn dec_arr_float_declaration() {
    let tokens = common::lex("dec arr[float] x = [1.0, 2.0, 3.0]");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Float);
    assert_eq!(tokens[3].lexeme, "float");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::FloatLiteral(1.0));
    assert_eq!(tokens[8].lexeme, "1.0");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::FloatLiteral(2.0));
    assert_eq!(tokens[10].lexeme, "2.0");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::FloatLiteral(3.0));
    assert_eq!(tokens[12].lexeme, "3.0");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `dec arr[bool] x = [true, false, true]` — checks every token in order
#[test]
fn dec_arr_bool_declaration() {
    let tokens = common::lex("dec arr[bool] x = [true, false, true]");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Bool);
    assert_eq!(tokens[3].lexeme, "bool");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[8].lexeme, "true");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[10].lexeme, "false");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[12].lexeme, "true");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `dec arr[string] x = ["one", "two", "three"]` — checks every token in order
#[test]
fn dec_arr_string_declaration() {
    let tokens = common::lex("dec arr[string] x = [\"one\", \"two\", \"three\"]");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::String);
    assert_eq!(tokens[3].lexeme, "string");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::StringLiteral("one".to_string()));
    assert_eq!(tokens[8].lexeme, "\"one\"");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(
        tokens[10].token,
        TokenType::StringLiteral("two".to_string())
    );
    assert_eq!(tokens[10].lexeme, "\"two\"");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(
        tokens[12].token,
        TokenType::StringLiteral("three".to_string())
    );
    assert_eq!(tokens[12].lexeme, "\"three\"");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `dec arr[char] x = ['a', 'b', 'c']` — checks every token in order
#[test]
fn dec_arr_char_declaration() {
    let tokens = common::lex("dec arr[char] x = ['a', 'b', 'c']");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Char);
    assert_eq!(tokens[3].lexeme, "char");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::CharacterLiteral('a'));
    assert_eq!(tokens[8].lexeme, "'a'");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::CharacterLiteral('b'));
    assert_eq!(tokens[10].lexeme, "'b'");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::CharacterLiteral('c'));
    assert_eq!(tokens[12].lexeme, "'c'");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

// ------------------------------------------------------------
// CONST declarations
// ------------------------------------------------------------

/// `CONST int x = 1` — checks every token in order
#[test]
fn const_int_declaration() {
    let tokens = common::lex("CONST int x = 1");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Int);
    assert_eq!(tokens[1].lexeme, "int");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::NumberLiteral(1));
    assert_eq!(tokens[4].lexeme, "1");
}

/// `CONST float x = 0.0` — checks every token in order
#[test]
fn const_float_declaration() {
    let tokens = common::lex("CONST float x = 0.0");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Float);
    assert_eq!(tokens[1].lexeme, "float");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::FloatLiteral(0.0));
    assert_eq!(tokens[4].lexeme, "0.0");
}

/// `CONST bool x = false` — checks every token in order
#[test]
fn const_bool_declaration() {
    let tokens = common::lex("CONST bool x = false");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Bool);
    assert_eq!(tokens[1].lexeme, "bool");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[4].lexeme, "false");
}

/// `CONST string x = "hello"` — checks every token in order
#[test]
fn const_string_declaration() {
    let tokens = common::lex("CONST string x = \"hello\"");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::String);
    assert_eq!(tokens[1].lexeme, "string");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(
        tokens[4].token,
        TokenType::StringLiteral("hello".to_string())
    );
    assert_eq!(tokens[4].lexeme, "\"hello\"");
}

/// `CONST char x = 'a'` — checks every token in order
#[test]
fn const_char_declaration() {
    let tokens = common::lex("CONST char x = 'a'");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Char);
    assert_eq!(tokens[1].lexeme, "char");
    assert_eq!(tokens[2].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[2].lexeme, "x");
    assert_eq!(tokens[3].token, TokenType::Assign);
    assert_eq!(tokens[3].lexeme, "=");
    assert_eq!(tokens[4].token, TokenType::CharacterLiteral('a'));
    assert_eq!(tokens[4].lexeme, "'a'");
}

/// `CONST arr[int] x = [1, 2, 3]` — checks every token in order
#[test]
fn const_arr_int_declaration() {
    let tokens = common::lex("CONST arr[int] x = [1, 2, 3]");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Int);
    assert_eq!(tokens[3].lexeme, "int");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::NumberLiteral(1));
    assert_eq!(tokens[8].lexeme, "1");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::NumberLiteral(2));
    assert_eq!(tokens[10].lexeme, "2");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::NumberLiteral(3));
    assert_eq!(tokens[12].lexeme, "3");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `CONST arr[float] x = [1.0, 2.0, 3.0]` — checks every token in order
#[test]
fn const_arr_float_declaration() {
    let tokens = common::lex("CONST arr[float] x = [1.0, 2.0, 3.0]");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Float);
    assert_eq!(tokens[3].lexeme, "float");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::FloatLiteral(1.0));
    assert_eq!(tokens[8].lexeme, "1.0");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::FloatLiteral(2.0));
    assert_eq!(tokens[10].lexeme, "2.0");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::FloatLiteral(3.0));
    assert_eq!(tokens[12].lexeme, "3.0");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `CONST arr[bool] x = [true, false, true]` — checks every token in order
#[test]
fn const_arr_bool_declaration() {
    let tokens = common::lex("CONST arr[bool] x = [true, false, true]");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Bool);
    assert_eq!(tokens[3].lexeme, "bool");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[8].lexeme, "true");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::BoolLiteral(false));
    assert_eq!(tokens[10].lexeme, "false");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::BoolLiteral(true));
    assert_eq!(tokens[12].lexeme, "true");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `CONST arr[string] x = ["one", "two", "three"]` — checks every token in order
#[test]
fn const_arr_string_declaration() {
    let tokens = common::lex("CONST arr[string] x = [\"one\", \"two\", \"three\"]");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::String);
    assert_eq!(tokens[3].lexeme, "string");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::StringLiteral("one".to_string()));
    assert_eq!(tokens[8].lexeme, "\"one\"");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(
        tokens[10].token,
        TokenType::StringLiteral("two".to_string())
    );
    assert_eq!(tokens[10].lexeme, "\"two\"");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(
        tokens[12].token,
        TokenType::StringLiteral("three".to_string())
    );
    assert_eq!(tokens[12].lexeme, "\"three\"");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}

/// `CONST arr[char] x = ['a', 'b', 'c']` — checks every token in order
#[test]
fn const_arr_char_declaration() {
    let tokens = common::lex("CONST arr[char] x = ['a', 'b', 'c']");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
    assert_eq!(tokens[1].token, TokenType::Array);
    assert_eq!(tokens[1].lexeme, "arr");
    assert_eq!(tokens[2].token, TokenType::LeftBracket);
    assert_eq!(tokens[2].lexeme, "[");
    assert_eq!(tokens[3].token, TokenType::Char);
    assert_eq!(tokens[3].lexeme, "char");
    assert_eq!(tokens[4].token, TokenType::RightBracket);
    assert_eq!(tokens[4].lexeme, "]");
    assert_eq!(tokens[5].token, TokenType::Identifier("x".to_string()));
    assert_eq!(tokens[5].lexeme, "x");
    assert_eq!(tokens[6].token, TokenType::Assign);
    assert_eq!(tokens[6].lexeme, "=");
    assert_eq!(tokens[7].token, TokenType::LeftBracket);
    assert_eq!(tokens[7].lexeme, "[");
    assert_eq!(tokens[8].token, TokenType::CharacterLiteral('a'));
    assert_eq!(tokens[8].lexeme, "'a'");
    assert_eq!(tokens[9].token, TokenType::Comma);
    assert_eq!(tokens[9].lexeme, ",");
    assert_eq!(tokens[10].token, TokenType::CharacterLiteral('b'));
    assert_eq!(tokens[10].lexeme, "'b'");
    assert_eq!(tokens[11].token, TokenType::Comma);
    assert_eq!(tokens[11].lexeme, ",");
    assert_eq!(tokens[12].token, TokenType::CharacterLiteral('c'));
    assert_eq!(tokens[12].lexeme, "'c'");
    assert_eq!(tokens[13].token, TokenType::RightBracket);
    assert_eq!(tokens[13].lexeme, "]");
}
