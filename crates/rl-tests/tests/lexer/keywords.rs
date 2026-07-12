//! Tests for lexer keyword recognition.
//! Covers control flow type and other keywords

use super::super::common;
use rl_lexer::tokentypes::TokenType;

/// `fn` keyword produces `Fn` with correct lexeme
#[test]
fn fn_keyword() {
    let tokens = common::lex("fn");
    assert_eq!(tokens[0].token, TokenType::Fn);
    assert_eq!(tokens[0].lexeme, "fn");
}

/// `in` keyword produces `In` with correct lexeme
#[test]
fn in_keyword() {
    let tokens = common::lex("in");
    assert_eq!(tokens[0].token, TokenType::In);
    assert_eq!(tokens[0].lexeme, "in");
}

/// `for` keyword produces `For` with correct lexeme
#[test]
fn for_keyword() {
    let tokens = common::lex("for");
    assert_eq!(tokens[0].token, TokenType::For);
    assert_eq!(tokens[0].lexeme, "for");
}

/// `while` keyword produces `While` with correct lexeme
#[test]
fn while_keyword() {
    let tokens = common::lex("while");
    assert_eq!(tokens[0].token, TokenType::While);
    assert_eq!(tokens[0].lexeme, "while");
}

/// `return` keyword produces `Return` with correct lexeme
#[test]
fn return_keyword() {
    let tokens = common::lex("return");
    assert_eq!(tokens[0].token, TokenType::Return);
    assert_eq!(tokens[0].lexeme, "return");
}

/// `break` keyword produces `Break` with correct lexeme
#[test]
fn break_keyword() {
    let tokens = common::lex("break");
    assert_eq!(tokens[0].token, TokenType::Break);
    assert_eq!(tokens[0].lexeme, "break");
}

/// `continue` keyword produces `Continue` with correct lexeme
#[test]
fn continue_keyword() {
    let tokens = common::lex("continue");
    assert_eq!(tokens[0].token, TokenType::Continue);
    assert_eq!(tokens[0].lexeme, "continue");
}

/// `get` keyword produces `Get` with correct lexeme
#[test]
fn get_keyword() {
    let tokens = common::lex("get");
    assert_eq!(tokens[0].token, TokenType::Get);
    assert_eq!(tokens[0].lexeme, "get");
}

/// `from` keyword produces `From` with correct lexeme
#[test]
fn from_keyword() {
    let tokens = common::lex("from");
    assert_eq!(tokens[0].token, TokenType::From);
    assert_eq!(tokens[0].lexeme, "from");
}

/// `or` keyword produces `Or` with correct lexeme
#[test]
fn or_keyword() {
    let tokens = common::lex("or");
    assert_eq!(tokens[0].token, TokenType::Or);
    assert_eq!(tokens[0].lexeme, "or");
}

/// `and` keyword produces `And` with correct lexeme
#[test]
fn and_keyword() {
    let tokens = common::lex("and");
    assert_eq!(tokens[0].token, TokenType::And);
    assert_eq!(tokens[0].lexeme, "and");
}

/// `if` keyword produces `If` with correct lexeme
#[test]
fn if_keyword() {
    let tokens = common::lex("if");
    assert_eq!(tokens[0].token, TokenType::If);
    assert_eq!(tokens[0].lexeme, "if");
}

/// `else` keyword produces `Else` with correct lexeme
#[test]
fn else_keyword() {
    let tokens = common::lex("else");
    assert_eq!(tokens[0].token, TokenType::Else);
    assert_eq!(tokens[0].lexeme, "else");
}

/// `const` keyword produces `Const` with correct lexeme
#[test]
fn const_keyword() {
    let tokens = common::lex("CONST");
    assert_eq!(tokens[0].token, TokenType::Const);
    assert_eq!(tokens[0].lexeme, "CONST");
}

/// `dec` keyword produces `Dec` with correct lexeme
#[test]
fn dec_keyword() {
    let tokens = common::lex("dec");
    assert_eq!(tokens[0].token, TokenType::Dec);
    assert_eq!(tokens[0].lexeme, "dec");
}

/// `int` type keyword produces `Int` with correct lexeme
#[test]
fn int_type_keyword() {
    let tokens = common::lex("int");
    assert_eq!(tokens[0].token, TokenType::Int);
    assert_eq!(tokens[0].lexeme, "int");
}

/// `float` type keyword produces `Float` with correct lexeme
#[test]
fn float_type_keyword() {
    let tokens = common::lex("float");
    assert_eq!(tokens[0].token, TokenType::Float);
    assert_eq!(tokens[0].lexeme, "float");
}

/// `bool` type keyword produces `Bool` with correct lexeme
#[test]
fn bool_type_keyword() {
    let tokens = common::lex("bool");
    assert_eq!(tokens[0].token, TokenType::Bool);
    assert_eq!(tokens[0].lexeme, "bool");
}

/// `string` type keyword produces `String` with correct lexeme
#[test]
fn string_type_keyword() {
    let tokens = common::lex("string");
    assert_eq!(tokens[0].token, TokenType::String);
    assert_eq!(tokens[0].lexeme, "string");
}

/// `char` type keyword produces `Char` with correct lexeme
#[test]
fn char_type_keyword() {
    let tokens = common::lex("char");
    assert_eq!(tokens[0].token, TokenType::Char);
    assert_eq!(tokens[0].lexeme, "char");
}

/// `array` type keyword produces `Array` with correct lexeme
#[test]
fn array_type_keyword() {
    let tokens = common::lex("arr");
    assert_eq!(tokens[0].token, TokenType::Array);
    assert_eq!(tokens[0].lexeme, "arr");
}
