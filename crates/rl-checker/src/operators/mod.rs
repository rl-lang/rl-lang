//! Operator type-checking - binary, unary, and index-assign.

mod binary;
mod index_assign;
mod unary;

use crate::lexer::tokentypes::TokenType;

/// Returns the display string for a binary operator token, used in error messages.
pub fn op_str(op: &TokenType) -> &'static str {
    match op {
        TokenType::Plus => "+",
        TokenType::Minus => "-",
        TokenType::Star => "*",
        TokenType::Slash => "/",
        TokenType::Less => "<",
        TokenType::Greater => ">",
        TokenType::LessEqual => "<=",
        TokenType::GreaterEqual => ">=",
        TokenType::Compare => "==",
        TokenType::BangEqual => "!=",
        _ => "?",
    }
}
