//! Record (struct) literal parser: `Name { field: value, ... }`.
//!
//! Called from [`crate::parser::expressions::primary`] once a plain
//! identifier is recognized as a previously-declared record name and is
//! immediately followed by `{`.

use crate::{
    ast::{ExprId, nodes::ExpressionKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses the body of a struct literal after `Name` has already been
    /// consumed. `start` is the span of the beginning of the `Name` token.
    pub fn parse_struct_literal(&mut self, name: String, start: Span) -> Result<ExprId, Error> {
        // consume `{`
        self.advance();
        while self.match_type(&[TokenType::Newline]) {}

        let mut fields = Vec::new();
        while self.peek() != TokenType::RightBrace {
            let field_name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected field name", self.peek_span())),
            };
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Colon]) {
                return Err(self.err("expected `:` after field name", self.peek_span()));
            }
            while self.match_type(&[TokenType::Newline]) {}
            let value = self.parse_expression()?;
            fields.push((field_name, value));

            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::RightBrace {
                break;
            }
            if !self.match_type(&[TokenType::Comma]) {
                return Err(self.err(
                    "expected `,` between struct literal fields",
                    self.peek_span(),
                ));
            }
            while self.match_type(&[TokenType::Newline]) {}
        }

        if !self.match_type(&[TokenType::RightBrace]) {
            return Err(self.err("expected `}` after struct literal", self.peek_span()));
        }

        let span = start.join(self.previous_span());
        let expr = self
            .ast_arena
            .alloc_expr(ExpressionKind::StructLiteral { name, fields }, span);
        self.parse_postfix(expr, start)
    }
}
