use crate::{
    ast::nodes::{Expression, ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses multiplicative expressions: `*` and `/`.
    ///
    /// Left-associative: `a * b / c` is `(a * b) / c`.
    pub fn parse_factor(&mut self) -> Result<Expression, Error> {
        let mut left = self.parse_unary()?;
        while self.match_type(&[TokenType::Star, TokenType::Slash]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_unary()?;
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
