use crate::parser_logic::Parser;
use rl_ast::{ExprId, nodes::ExpressionKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses unary prefix expressions: `!` (logical not) and `-` (negation).
    ///
    /// Right-associative by recursion: `--x` parses as `-(-(x))`.
    /// Falls through to [`parse_primary`] when no prefix operator is present.
    ///
    /// [`parse_primary`]: Parser::parse_primary
    pub fn parse_unary(&mut self) -> Result<ExprId, Error> {
        let start = self.peek_span();
        if self.match_type(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let operand = self.parse_unary()?;
            let operand_id = self.ast_arena.exprs.get(operand);
            let span = start.join(operand_id.span);
            return Ok(self
                .ast_arena
                .alloc_expr(ExpressionKind::Unary { operator, operand }, span));
        }
        self.parse_primary()
    }
}
