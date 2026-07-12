use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses multiplicative expressions: `*` and `/`.
    ///
    /// Left-associative: `a * b / c` is `(a * b) / c`.
    pub fn parse_factor(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_unary()?;
        while self.match_type(&[TokenType::Star, TokenType::Slash]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_unary()?;
            let left_id = self.ast_arena.exprs.get(left);
            let right_id = self.ast_arena.exprs.get(right);

            let span = left_id.span.join(right_id.span);
            left = self.ast_arena.alloc_expr(
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
