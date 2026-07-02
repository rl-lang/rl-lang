use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

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
