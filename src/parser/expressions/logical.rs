use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    pub fn parse_logical(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_equality()?;
        while self.match_type(&[TokenType::And, TokenType::Or]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_equality()?;
            let span = self.expr_span(left).join(self.expr_span(right));
            left = self.ast.alloc_expr(
                ExpressionKind::Binary {
                    left,
                    operator,
                    right,
                },
                span,
            );
        }

        Ok(left)
    }
}
