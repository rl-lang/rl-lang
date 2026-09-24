use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::{Error, Reason};
use rl_utils::span::Span;

pub fn token_to_c_op(token: &TokenType) -> Result<&'static str, Error> {
    match token {
        TokenType::Plus => Ok("+"),
        TokenType::Minus => Ok("-"),
        TokenType::Star => Ok("*"),
        TokenType::Slash => Ok("/"),
        TokenType::Compare => Ok("=="),
        TokenType::BangEqual => Ok("!="),
        TokenType::Less => Ok("<"),
        TokenType::LessEqual => Ok("<="),
        TokenType::Greater => Ok(">"),
        TokenType::GreaterEqual => Ok(">="),
        TokenType::And => Ok("&&"),
        TokenType::Or => Ok("||"),
        TokenType::Assign => Ok("="),
        TokenType::PlusEqual => Ok("+="),
        TokenType::MinusEqual => Ok("-="),
        TokenType::StarEqual => Ok("*="),
        TokenType::SlashEqual => Ok("/="),
        _ => Err(Error::at(
            Reason::Compile,
            format!("unsupported binary operator {:?}", token),
            Span::dummy(),
        )),
    }
}
