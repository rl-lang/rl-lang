//! Constant declaration parser (`CONST`).
//!
//! Handles the `CONST` keyword, which introduces an immutable binding. Mirrors
//! the variable declaration syntax but produces `C*` type annotation variants
//! and [`StatementKind::ConstantDeclaration`] / [`StatementKind::ConstantArray`].
//!
//! ```text
//! CONST int X = 42
//! CONST array[int] XS = [1, 2, 3]
//! ```

use crate::{
    ast::statements::{Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses a `CONST` constant declaration.
    ///
    /// Called after `CONST` has been consumed. Handles two forms:
    ///
    /// - **Constant array** - `CONST array[T] NAME = [items]` or
    ///   `CONST array[T] NAME = expr` - detected by the `array` keyword followed
    ///   by `[`. Produces [`StatementKind::ConstantArray`] for inline literals or
    ///   [`StatementKind::ConstantDeclaration`] with `TypeAnnotation::CArray` for
    ///   expression initialisers.
    ///
    /// - **Scalar constant** - `const T NAME = expr` - produces
    ///   [`StatementKind::ConstantDeclaration`] with the corresponding `C*`
    ///   annotation via [`parse_type`]`(false)`.
    ///
    /// # Errors
    /// Returns an error if the type, name, `=`, or initialiser expression is
    /// missing or malformed.
    ///
    /// [`parse_type`]: Parser::parse_type
    pub fn parse_const_declartion(&mut self, start: Span) -> Result<Statement, Error> {
        #[cfg(feature = "debug")]
        log::debug!("{:?}", self.peek());
        #[cfg(feature = "debug")]
        log::debug!("parsing type");
        if self.match_type(&[TokenType::Array]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            let annoation_type = match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::Int
                }
                TokenType::Byte => {
                    self.advance();
                    TypeAnnotation::Byte
                }
                TokenType::Float => {
                    self.advance();
                    TypeAnnotation::Float
                }
                TokenType::Bool => {
                    self.advance();
                    TypeAnnotation::Bool
                }
                TokenType::String => {
                    self.advance();
                    TypeAnnotation::String
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::Char
                }
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(false)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Array(Box::new(inner))
                }
                _ => return Err(self.err("expected type after `CONST`", self.peek_span())),
            };
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after type", self.peek_span()));
            }

            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected name after array type", self.peek_span())),
            };

            if !self.match_type(&[TokenType::Assign]) {
                return Err(self.err("expected `=` after name", self.peek_span()));
            }

            if self.peek() == TokenType::LeftBracket {
                self.advance();
                let mut items = Vec::new();
                while self.match_type(&[TokenType::Newline]) {}

                while self.peek() != TokenType::RightBracket {
                    let value = self.parse_expression()?;
                    items.push(value);
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightBracket {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected `,` between list items", self.peek_span()));
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                }
                self.match_type(&[TokenType::RightBracket]);
                let span = start.join(self.previous_span());
                return Ok(Statement::new(
                    StatementKind::ConstantArray {
                        name,
                        type_annotation: annoation_type,
                        value: items,
                    },
                    span,
                ));
            } else {
                let value = self.parse_expression()?;
                let span = start.join(value.span);
                return Ok(Statement::new(
                    StatementKind::ConstantDeclaration {
                        name,
                        type_annotation: TypeAnnotation::CArray(Box::new(annoation_type)),
                        value,
                    },
                    span,
                ));
            }
        }

        let const_type = self.parse_type(false)?;
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected name after type", self.peek_span())),
        };

        if !self.match_type(&[TokenType::Assign]) {
            return Err(self.err("expected `=` after name", self.peek_span()));
        }

        let value = self.parse_expression()?;
        let span = start.join(value.span);

        Ok(Statement::new(
            StatementKind::ConstantDeclaration {
                name,
                type_annotation: const_type,
                value,
            },
            span,
        ))
    }
}
