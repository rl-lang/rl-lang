use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses `==` and `!=` binary expressions (lowest precedence).
    ///
    /// Left-associative: `a == b != c` is `(a == b) != c`.
    pub fn parse_equality(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_comparsion()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::Compare]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_comparsion()?;
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
