use crate::{
    ast::nodes::{Expression, ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses unary prefix expressions: `!` (logical not) and `-` (negation).
    ///
    /// Right-associative by recursion: `--x` parses as `-(-(x))`.
    /// Falls through to [`parse_primary`] when no prefix operator is present.
    ///
    /// [`parse_primary`]: Parser::parse_primary
    pub fn parse_unary(&mut self) -> Result<Expression, Error> {
        let start = self.peek_span();
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let operand = self.parse_unary()?;
            let span = start.join(operand.span);
            return Ok(Expression::new(
                ExpressionKind::Unary {
                    operator,
                    operand: Box::new(operand),
                },
                span,
            ));
        }
        self.parse_primary()
    }
}
