use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses comparison and compound-assignment operators:
    /// `<`, `<=`, `>`, `>=`, `+=`, `-=`, `*=`, `/=`.
    ///
    /// Compound-assignment operators (`+=` etc.) are desugared in-place: when
    /// the left-hand side is a plain [`ExpressionKind::Identifier`], they expand
    /// to an [`ExpressionKind::Assign`] whose value is the corresponding
    /// [`ExpressionKind::Binary`]. If the LHS is not a simple identifier the
    /// operator is treated as a plain binary expression (the checker will reject
    /// it later if needed).
    pub fn parse_comparsion(&mut self) -> Result<ExprId, Error> {
        let mut left = self.parse_term()?;
        while self.match_type(&[
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::StarEqual,
            TokenType::SlashEqual,
        ]) {
            while self.match_type(&[TokenType::Newline]) {}
            let operator = self.previous();
            while self.match_type(&[TokenType::Newline]) {}
            let right = self.parse_term()?;
            let left_id = self.ast_arena.exprs.get(left);
            let right_id = self.ast_arena.exprs.get(right);

            let span = left_id.span.join(right_id.span);

            match operator {
                TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual => {
                    if let ExpressionKind::Identifier(name) = &left_id.kind {
                        let name = name.clone();
                        let lhs_span = left_id.span;
                        let operator = match operator {
                            TokenType::PlusEqual => TokenType::Plus,
                            TokenType::MinusEqual => TokenType::Minus,
                            TokenType::StarEqual => TokenType::Star,
                            TokenType::SlashEqual => TokenType::Slash,
                            _ => unreachable!(),
                        };
                        let left_expr = self
                            .ast_arena
                            .alloc_expr(ExpressionKind::Identifier(name.clone()), lhs_span);

                        let binary = self.ast_arena.alloc_expr(
                            ExpressionKind::Binary {
                                left: left_expr,
                                operator,
                                right,
                            },
                            span,
                        );

                        left = self.ast_arena.alloc_expr(
                            ExpressionKind::Assign {
                                name,
                                value: binary,
                            },
                            span,
                        );
                    } else {
                        left = self.ast_arena.alloc_expr(
                            ExpressionKind::Binary {
                                left,
                                operator,
                                right,
                            },
                            span,
                        );
                    }
                }
                _ => {
                    left = self.ast_arena.alloc_expr(
                        ExpressionKind::Binary {
                            left,
                            operator,
                            right,
                        },
                        span,
                    );
                }
            }
        }
        Ok(left)
    }
}
