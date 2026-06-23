//! Function declaration parser (`fn`).
//!
//! Handles named function declarations in the form:
//!
//! ```text
//! fn name(T param, T param) -> ReturnType {
//!     body
//! }
//! ```
//!
//! Return type annotations are optional; omitting `-> T` defaults to
//! [`TypeAnnotation::Null`]. The `is_entry` flag is set when the function is
//! preceded by a `!#[entry]` attribute, marking it as the program entry point.

use crate::{
    ast::statements::{Param, Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses a named function declaration.
    ///
    /// Called after `fn` has been consumed (either by [`parse_statement_to_ast`]
    /// or [`parse_entry_attribute`]). Reads:
    ///
    /// 1. The function name (identifier).
    /// 2. A `(`-delimited, comma-separated parameter list of `T name` pairs.
    /// 3. An optional `-> T` return type annotation (defaults to
    ///    [`TypeAnnotation::Null`] when absent).
    /// 4. The function body via [`parse_block`].
    ///
    /// Produces [`StatementKind::FunctionDeclaration`].
    ///
    /// # Parameters
    /// - `start` - the span of the `fn` token, used as the start of the overall span.
    /// - `is_entry` - `true` when the function was marked with `!#[entry]`.
    ///
    /// # Errors
    /// Returns an error if the function name, a parameter name, or the body
    /// block are missing or malformed.
    ///
    /// [`parse_statement_to_ast`]: Parser::parse_statement_to_ast
    /// [`parse_entry_attribute`]: Parser::parse_entry_attribute
    /// [`parse_block`]: Parser::parse_block
    pub fn parse_function(&mut self, start: Span, is_entry: bool) -> Result<Statement, Error> {
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected function name", self.peek_span())),
        };

        self.match_type(&[TokenType::LeftParen]);

        let mut params: Vec<Param> = Vec::new();
        while !self.match_type(&[TokenType::RightParen]) {
            let param_type = self.parse_param_type()?;
            match self.peek() {
                TokenType::Identifier(p) => {
                    self.advance();
                    params.push(Param {
                        param_name: p,
                        param_type,
                    });
                }
                _ => return Err(self.err("expected parameter name", self.peek_span())),
            }
            if !self.match_type(&[TokenType::Comma]) {
                break;
            }
        }
        self.match_type(&[TokenType::RightParen]);

        // optional return type annotation; defaults to Null when omitted
        let return_type = if self.match_type(&[TokenType::Arrow]) {
            match self.parse_param_type() {
                Ok(a) => a,
                Err(_) => TypeAnnotation::Null,
            }
        } else {
            TypeAnnotation::Null
        };

        let body = self.parse_block()?;

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                is_entry,
            },
            span,
        ))
    }
}
