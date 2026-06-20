mod binary;
mod index_assign;
mod unary;

use crate::lexer::tokentypes::TokenType;

// returns the operand as str
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
