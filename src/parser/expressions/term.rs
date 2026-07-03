use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses additive expressions: `+` and `-`.
    ///
    /// Left-associative: `a + b - c` is `(a + b) - c`.
    pub fn parse_term(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_factor()?;
        while self.match_type(&[TokenType::Plus, TokenType::Minus]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_factor()?;
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
