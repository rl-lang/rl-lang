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
            let span = left.span.join(right.span);

            match operator {
                TokenType::PlusEqual
                | TokenType::MinusEqual
                | TokenType::StarEqual
                | TokenType::SlashEqual => {
                    if let ExpressionKind::Identifier(name) = &left.kind {
                        let name = name.clone();
                        let lhs_span = left.span;
                        let operator = match operator {
                            TokenType::PlusEqual => TokenType::Plus,
                            TokenType::MinusEqual => TokenType::Minus,
                            TokenType::StarEqual => TokenType::Star,
                            TokenType::SlashEqual => TokenType::Slash,
                            _ => unreachable!(),
                        };
                        let binary = Expression::new(
                            ExpressionKind::Binary {
                                left: Box::new(Expression::new(
                                    ExpressionKind::Identifier(name.clone()),
                                    lhs_span,
                                )),
                                operator,
                                right: Box::new(right),
                            },
                            span,
                        );
                        left = Expression::new(
                            ExpressionKind::Assign {
                                name,
                                value: Box::new(binary),
                            },
                            span,
                        );
                    } else {
                        left = Expression::new(
                            ExpressionKind::Binary {
                                left: Box::new(left),
                                operator,
                                right: Box::new(right),
                            },
                            span,
                        );
                    }
                }
                _ => {
                    left = Expression::new(
                        ExpressionKind::Binary {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        },
                        span,
                    );
                }
            }
        }
        Ok(left)
    }
}
