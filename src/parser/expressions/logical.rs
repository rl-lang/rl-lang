use crate::{
    ast::nodes::{Expression, ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_logical(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_equality()?;
        while self.match_type(&[TokenType::And, TokenType::Or]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_equality()?;
            let span = left.span.join(right.span);
            left = Expression::new(
                ExpressionKind::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                },
                span,
            );
        }

        Ok(left)
    }
}
