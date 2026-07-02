use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

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
            let span = self.expr_span(left).join(self.expr_span(right));

            match operator {
                TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual => {
                    if let ExpressionKind::Identifier(name) = &self.expr_kind(left) {
                        let name = name.clone();
                        let lhs_span = self.expr_span(left);
                        let operator = match operator {
                            TokenType::PlusEqual => TokenType::Plus,
                            TokenType::MinusEqual => TokenType::Minus,
                            TokenType::StarEqual => TokenType::Star,
                            TokenType::SlashEqual => TokenType::Slash,
                            _ => unreachable!(),
                        };
                        let l = self
                            .ast
                            .alloc_expr(ExpressionKind::Identifier(name.clone()), lhs_span);

                        let binary = self.ast.alloc_expr(
                            ExpressionKind::Binary {
                                left: l,
                                operator,
                                right,
                            },
                            span,
                        );
                        left = self.ast.alloc_expr(
                            ExpressionKind::Assign {
                                name,
                                value: binary,
                            },
                            span,
                        );
                    } else {
                        left = self.ast.alloc_expr(
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
                    left = self.ast.alloc_expr(
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
