use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind, statements::TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses `==` and `!=` binary expressions (lowest precedence).
    ///
    /// Left-associative: `a == b != c` is `(a == b) != c`.
    ///
    /// The `is` type test (`x is int`) binds at this level too, but its
    /// right side is a type, not an expression. Union targets are
    /// rejected (`x is any[...]` narrows to nothing - test members).
    pub fn parse_equality(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_comparsion()?;
        while self.match_type(&[TokenType::BangEqual, TokenType::Compare, TokenType::Is]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            if operator == TokenType::Is {
                let left_id = self.ast_arena.exprs.get(left);
                let span_start = left_id.span;
                let target_type = self.parse_type(true)?;
                match &target_type {
                    TypeAnnotation::Any(_) | TypeAnnotation::CAny(_) => {
                        return Err(self.err(
                            "`is` needs one concrete type - test union members one by one",
                            self.peek_span(),
                        ));
                    }
                    _ => {}
                }
                let end = self.previous_span();
                left = self.ast_arena.alloc_expr(
                    ExpressionKind::Is {
                        value: left,
                        target_type,
                    },
                    span_start.join(end),
                );
                continue;
            }
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
