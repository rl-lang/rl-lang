//! Unit annotation parser.
//!
//! Parses unit expressions that appear after `:` in numeric declarations.
//!
//! ```text
//! dec float distance: m = 10.0
//! dec float speed: m/s = 12.5
//! dec float force: kg*m/s = 20.0
//! ```
//!
//! Unit annotations are represented as [`UnitAnnotation`] syntax trees.
//! Their mathematical normalization and validation are handled later by the
//! checker.

use crate::parser_logic::Parser;
use rl_ast::statements::UnitAnnotation;
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

impl Parser {
    /// Parses a complete unit annotation expression.
    ///
    /// The `:` token must already have been consumed by the declaration
    /// parser. Parsing begins at the first unit symbol and stops before a token
    /// that is not part of a unit expression, normally `=`.
    ///
    /// Supported syntax:
    ///
    /// ```text
    /// m
    /// m/s
    /// kg*m
    /// kg*m/s
    /// ```
    ///
    /// Multiplication and division currently have equal precedence and are
    /// parsed from left to right.
    ///
    /// For example:
    ///
    /// ```text
    /// kg*m/s
    /// ```
    ///
    /// produces approximately:
    ///
    /// ```text
    /// Divide(
    ///     Multiply(Symbol("kg"), Symbol("m")),
    ///     Symbol("s"),
    /// )
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error when:
    ///
    /// - the annotation does not begin with a unit symbol;
    /// - `*` or `/` is not followed by another unit symbol.
    pub fn parse_unit_annotation(&mut self) -> Result<UnitAnnotation, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let mut unit = self.parse_unit_symbol()?;

        loop {
            while self.match_type(&[TokenType::Newline]) {}

            let operator = match self.peek() {
                TokenType::Star => TokenType::Star,
                TokenType::Slash => TokenType::Slash,
                _ => break,
            };

            self.advance();

            while self.match_type(&[TokenType::Newline]) {}

            let right = self.parse_unit_symbol()?;

            unit = match operator {
                TokenType::Star => UnitAnnotation::Multiply(Box::new(unit), Box::new(right)),
                TokenType::Slash => UnitAnnotation::Divide(Box::new(unit), Box::new(right)),
                _ => unreachable!("unit parser only accepts `*` and `/` operators"),
            };
        }

        Ok(unit)
    }

    /// Parses a single unit symbol.
    ///
    /// Unit symbols are regular identifiers such as `m`, `s`, `kg`, or `pi`.
    ///
    /// # Errors
    ///
    /// Returns an error if the current token is not an identifier.
    fn parse_unit_symbol(&mut self) -> Result<UnitAnnotation, Error> {
        match self.peek() {
            TokenType::Identifier(name) => {
                self.advance();
                Ok(UnitAnnotation::Symbol(name))
            }
            _ => Err(self.err("expected a unit symbol", self.peek_span())),
        }
    }
}
