//! Record (struct) declaration parser (`record`).
//!
//! Handles record type declarations in the form:
//!
//! ```text
//! record Name {
//!     int field_a,
//!     string field_b,
//! }
//! ```
//!
//! Field types are parsed via [`Parser::parse_param_type`], so any type
//! usable in a function parameter (including other record names) may be
//! used as a field type. A trailing comma after the last field is allowed.

use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses a `record` declaration.
    ///
    /// Called after `record` has been consumed. Reads the record name,
    /// then a `{`-delimited, comma-separated list of `T name` field pairs.
    ///
    /// Produces [`StatementKind::RecordDeclaration`].
    ///
    /// # Errors
    /// Returns an error if the record name, a field type/name, or the
    /// closing `}` are missing or malformed.
    pub fn parse_record_declaration(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected record name", self.peek_span())),
        };

        self.record_names.insert(name.clone());

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected `{` after record name", self.peek_span()));
        }

        let mut fields = Vec::new();
        loop {
            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::RightBrace {
                break;
            }
            let field_type = self.parse_param_type()?;
            while self.match_type(&[TokenType::Newline]) {}
            let field_name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected field name", self.peek_span())),
            };
            fields.push((field_name, field_type));

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::RightBrace]) {
            return Err(self.err("expected `}` after record fields", self.peek_span()));
        }

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::RecordDeclaration { name, fields },
            span,
        ))
    }
}
