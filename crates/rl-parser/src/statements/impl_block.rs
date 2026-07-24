//! `impl` block parser - attaches methods to a `record` type.
//!
//! ```text
//! impl Point {
//!     fn magnitude(self) -> float {
//!         return sqrt((self.x * self.x + self.y * self.y) as float)
//!     }
//!
//!     fn new(x: int, y: int) -> Point {
//!         return Point { x: x, y: y }
//!     }
//! }
//! ```
//!
//! A method whose first parameter is a bare `self` (no type prefix) is an
//! instance method, called via `value.method(args)` - `self` is implicitly
//! typed as the enclosing record. A method without `self` is an associated
//! function, called via `RecordName::method(args)`.
//!
//! Everything else about a method (remaining params, return type, body) is
//! parsed exactly like a normal [`Parser::parse_function`] declaration.

use crate::parser_logic::Parser;
use rl_ast::statements::{FunctionAttribute, Param, Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::TokenType;
use rl_utils::{errors::Error, span::Span};

impl Parser {
    /// Parses an `impl` block. Called after `impl` has been consumed.
    ///
    /// # Errors
    /// Returns an error if the record name, `{`, a `fn` keyword, or any
    /// malformed method declaration is encountered.
    pub fn parse_impl_block(&mut self, start: Span) -> Result<Statement, Error> {
        while self.match_type(&[TokenType::Newline]) {}

        let record = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected record name after `impl`", self.peek_span())),
        };

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::LeftBrace]) {
            return Err(self.err("expected `{` after `impl` name", self.peek_span()));
        }

        let mut methods = Vec::new();
        loop {
            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::RightBrace || self.is_at_end() {
                break;
            }
            if !self.match_type(&[TokenType::Fn]) {
                return Err(self.err(
                    "expected `fn` for a method inside `impl` block",
                    self.peek_span(),
                ));
            }
            let fn_span = self.previous_span();
            methods.push(self.parse_impl_method(record.clone(), fn_span)?);
        }

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::RightBrace]) {
            return Err(self.err("expected `}` after impl body", self.peek_span()));
        }

        let span = start.join(self.previous_span());
        Ok(Statement::new(
            StatementKind::ImplBlock { record, methods },
            span,
        ))
    }

    /// Parses a single method inside an `impl` block. Called after `fn` has
    /// been consumed. Identical to [`Parser::parse_function`] except the
    /// first parameter may be a bare `self`, implicitly typed as `record`.
    fn parse_impl_method(&mut self, record: String, start: Span) -> Result<Statement, Error> {
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected method name", self.peek_span())),
        };

        if !self.match_type(&[TokenType::LeftParen]) {
            return Err(self.err("expected `(` after method name", self.peek_span()));
        }

        let mut params: Vec<Param> = Vec::new();

        // bare `self` as the first parameter - implicitly typed as the
        // enclosing record.
        if let TokenType::Identifier(p) = self.peek()
            && p == "self"
        {
            self.advance();
            params.push(Param {
                param_name: "self".to_string(),
                param_type: TypeAnnotation::Record(record.clone()),
            });
            self.match_type(&[TokenType::Comma]);
        }

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
                attribute: None as Option<FunctionAttribute>,
            },
            span,
        ))
    }
}
