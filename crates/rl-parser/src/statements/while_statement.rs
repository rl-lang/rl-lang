//! `while` loop parser.
//!
//! Handles the single `while` form:
//!
//! ```text
//! while (condition) { body }
//! ```

use crate::parser_logic::Parser;
use rl_ast::statements::{Statement, StatementKind};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses a `while` loop.
    ///
    /// Called after `while` has been consumed. Reads the loop condition as an
    /// expression, then the loop body via [`parse_block`], and produces
    /// [`StatementKind::While`].
    ///
    /// [`parse_block`]: Parser::parse_block
    pub fn parse_while(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let condition = self.parse_expression()?;
        while self.match_type(&[TokenType::Newline]) {}
        let body = self.parse_block()?;
        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::While { condition, body },
            span,
        ))
    }

    /// Parses a `loop` loop.
    ///
    /// Called after `loop` has been consumed. Reads the loop body via [`parse_block`],
    /// and produces [`StatementKind::Loop`].
    ///
    /// [`parse_block`]: Parser::parse_block
    pub fn parse_loop(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}
        let body = self.parse_block()?;
        let span = start.join(self.previous_span());
        Ok(Statement::new(StatementKind::Loop(body), span))
    }
}
