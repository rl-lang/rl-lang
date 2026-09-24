//! Program-level attribute parser.
//!
//! Parses `#![...]` attributes attached to the whole program - the `#![..]`
//! form (as opposed to `!#[..]` for functions and `#[..]` for statements).
//! Currently only one attribute is defined:
//!
//! ```text
//! #![convert(kg=1000(g))]
//! ```
//!
//! which declares that `1 kg = 1000 g`. The checker uses these declarations to
//! treat the two symbols as convertible, letting values carry either unit
//! interchangeably. Attributes are compile-time only and are discarded before
//! execution.

use rl_ast::statements::ProgramAttribute;
use rl_lexer::tokentypes::TokenType;
use rl_utils::errors::Error;

use crate::parser_logic::Parser;

/// Built-in attribute names no `#![define(...)]` may claim. Extend this
/// list (not the call sites) when new built-ins land.
pub(crate) const RESERVED_ATTRIBUTE_NAMES: &[&str] = &[
    "entry", "init", "final", "test", "allow", "deprecated", "convert", "define",
];

impl Parser {
    /// Parses a `#![...]` program attribute and returns it.
    ///
    /// The `#` and `!` tokens must already have been consumed by the top-level
    /// parse loop. Expects `[`, the attribute name, its arguments, then `]`.
    ///
    /// # Errors
    /// Returns an error if the brackets, attribute name, or arguments are
    /// malformed.
    pub fn parse_program_attribute(&mut self) -> Result<ProgramAttribute, Error> {
        if !self.match_type(&[TokenType::LeftBracket]) {
            return Err(self.err("expected `[` after `#!`", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}

        let attribute = match self.peek() {
            TokenType::Identifier(name) if name == "convert" => {
                self.advance();
                self.parse_convert_attribute()?
            }
            TokenType::Identifier(name) if name == "define" => {
                self.advance();
                self.parse_define_attribute()?
            }
            _ => return Err(self.err("expected a valid program attribute", self.peek_span())),
        };

        while self.match_type(&[TokenType::Newline]) {}

        if !self.match_type(&[TokenType::RightBracket]) {
            return Err(self.err("expected `]` to close program attribute", self.peek_span()));
        }

        Ok(attribute)
    }

    /// Parses a `define` attribute: `define(name)`.
    ///
    /// Declares a custom item marker usable as `!#[name]` or
    /// `!#[name("arg", ...)]`. Names colliding with built-in attributes
    /// or duplicate definitions are errors.
    ///
    /// # Errors
    /// Returns an error if the parentheses, name, or closing are malformed.
    fn parse_define_attribute(&mut self) -> Result<ProgramAttribute, Error> {
        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err("expected `(` after `define`", self.peek_span()));
        }
        while self.match_type(&[TokenType::Newline]) {}
        let name = match self.peek() {
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();
                name
            }
            _ => return Err(self.err("expected an attribute name", self.peek_span())),
        };
        if RESERVED_ATTRIBUTE_NAMES.contains(&name.as_str()) {
            return Err(self.err(
                format!("`{name}` is a built-in attribute and cannot be redefined"),
                self.peek_span(),
            ));
        }
        for attr in &self.ast_arena.program_attributes {
            if matches!(attr, ProgramAttribute::Define { name: prev } if prev == &name) {
                return Err(self.err(
                    format!("custom attribute `{name}` is already defined"),
                    self.peek_span(),
                ));
            }
        }
        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err("expected `)` to close define", self.peek_span()));
        }
        Ok(ProgramAttribute::Define { name })
    }
    ///
    /// Expects the `convert` identifier to have already been consumed. Reads
    /// `(`, a unit symbol, `=`, a numeric factor, `(`, a base unit symbol,
    /// `)`, then `)`.
    ///
    /// # Errors
    /// Returns an error if any part of the `convert(symbol=factor(base))`
    /// structure is missing or malformed.
    fn parse_convert_attribute(&mut self) -> Result<ProgramAttribute, Error> {
        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err("expected `(` after `convert`", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}

        let symbol =
            self.parse_unit_symbol_name("expected a unit symbol before `=` in `convert`")?;

        if !self.match_type(&[TokenType::Assign]) {
            return Err(self.err("expected `=` in `convert` attribute", self.peek_span()));
        }

        let factor = match self.peek() {
            TokenType::NumberLiteral(value) => {
                let factor = value as f64;
                self.advance();
                factor
            }
            TokenType::SignedLiteral(value) => {
                let factor = value as f64;
                self.advance();
                factor
            }
            TokenType::FloatLiteral(value) => {
                let factor = value;
                self.advance();
                factor
            }
            _ => {
                return Err(self.err(
                    "expected a numeric factor in `convert` attribute",
                    self.peek_span(),
                ));
            }
        };

        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err(
                "expected `(` around the base unit symbol in `convert` attribute",
                self.peek_span(),
            ));
        }

        while self.match_type(&[TokenType::Newline]) {}

        let base_symbol =
            self.parse_unit_symbol_name("expected a base unit symbol in `convert` attribute")?;

        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err(
                "expected `)` after the base unit symbol in `convert` attribute",
                self.peek_span(),
            ));
        }

        if !self.match_type(&[TokenType::RightParen]) {
            return Err(self.err(
                "expected `)` to close `convert` attribute",
                self.peek_span(),
            ));
        }

        Ok(ProgramAttribute::Convert {
            symbol,
            factor,
            base_symbol,
        })
    }

    /// Parses a single unit symbol name used inside a program attribute.
    ///
    /// Unit symbols are regular identifiers such as `m`, `s`, `kg`, or `N`.
    ///
    /// # Errors
    /// Returns an error if the current token is not an identifier.
    fn parse_unit_symbol_name(&mut self, message: &str) -> Result<String, Error> {
        match self.peek() {
            TokenType::Identifier(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(self.err(message, self.peek_span())),
        }
    }
}
