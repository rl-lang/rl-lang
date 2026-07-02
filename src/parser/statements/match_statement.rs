//! match statement parser.

use crate::{
    ast::{
        StmtId,
        statements::{MatchPattern, StatementKind},
    },
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    pub fn parse_match(&mut self, start: Span) -> Result<StmtId, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let value = self.parse_expression()?;

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected { after match value", self.peek_span()));
        }

        let mut arms: Vec<(MatchPattern, Vec<StmtId>)> = Vec::new();

        while !self.match_type(&[TokenType::RightBrace, TokenType::Eof]) {
            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::RightBrace || self.peek() == TokenType::Eof {
                break;
            }

            let pattern = if self.match_type(&[TokenType::Wildcard]) {
                MatchPattern::Wildcard
            } else {
                MatchPattern::Literal(self.parse_expression()?)
            };

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::FatArrow]) {
                return Err(self.err("expected => after match pattern", self.peek_span()));
            }

            let body = self.parse_block()?;
            while self.match_type(&[TokenType::Newline]) {}
            arms.push((pattern, body));
        }

        let span = start.join(self.previous_span());
        Ok(self
            .ast
            .alloc_stmt(StatementKind::Match { value, arms }, span))
    }
}
