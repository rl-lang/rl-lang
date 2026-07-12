//! Tag (enum) declaration parser (`tag`).
//!
//! Handles simple unit-variant enum declarations:
//!
//! ```text
//! tag Name {
//!     VariantA,
//!     VariantB,
//! }
//! ```
//!
//! A trailing comma after the last field is allowed.

use crate::{
    ast::statements::{Statement, StatementKind},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses a `tag` declaration.
    ///
    /// Called after `tag` has been consumed. Reads the tag name, then a
    /// `{`-delimited, comma-separated list of variant names.
    ///
    /// Produces [`StatementKind::TagDeclaration`].
    ///
    /// # Errors
    /// Returns an error if the tag name, a variant name, or the closing
    /// `}` are missing or malformed.
    pub fn parse_tag_declaration(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected tag name", self.peek_span())),
        };

        self.tag_names.insert(name.clone());

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected `{` after tag name", self.peek_span()));
        }

        let mut variants = Vec::new();
        loop {
            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::RightBrace {
                break;
            }
            let variant_name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected variant name", self.peek_span())),
            };
            variants.push(variant_name);

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::RightBrace]) {
            return Err(self.err("expected `}` after tag variants", self.peek_span()));
        }

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::TagDeclaration { name, variants },
            span,
        ))
    }
}
