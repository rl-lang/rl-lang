//! Variable declaration parser (`dec`).
//!
//! Handles the `dec` keyword, which introduces a mutable binding.
//!
//! ```text
//! dec int x = 42
//! dec array[int] xs = [1, 2, 3]
//! dec (int, string) p = (1, "hi")
//! dec int x, string y = (1, "hi")
//! ```

use crate::{
    ast::statements::{Statement, StatementKind, TypeAnnotation},
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Parses a `dec` variable declaration.
    ///
    /// Called after `dec` has been consumed. Handles two forms:
    ///
    /// - **Array declaration** - `dec array[T] name = [items]` or
    ///   `dec array[T] name = expr` - detected by the `array` keyword followed
    ///   by `[`. Produces [`StatementKind::Array`] for inline literals or
    ///   [`StatementKind::VariableDeclaration`] with `TypeAnnotation::Array` for
    ///   expression initialisers.
    ///
    /// - **Scalar declaration** - `dec T name = expr` - produces
    ///   [`StatementKind::VariableDeclaration`].
    ///
    /// # Errors
    /// Returns an error if the type, name, `=`, or initialiser expression is
    /// missing or malformed.
    pub fn parse_variable_declartion(&mut self, start: Span) -> Result<Statement, Error> {
        #[cfg(feature = "debug")]
        log::debug!("{:?}", self.peek());
        #[cfg(feature = "debug")]
        log::debug!("parsing type");

        // -- tuple: dec (T, T, ...) name = (...) --
        if self.peek() == TokenType::LeftParen {
            self.advance();
            while self.match_type(&[TokenType::Newline]) {}
            let first_type = self.parse_type(true)?;

            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::Comma || self.peek() == TokenType::RightParen {
                let mut types = vec![first_type];
                while self.match_type(&[TokenType::Comma]) {
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightParen {
                        break;
                    }
                    types.push(self.parse_type(true)?);
                }
                if !self.match_type(&[TokenType::RightParen]) {
                    return Err(self.err("expected `)` after tuple types", self.peek_span()));
                }
                while self.match_type(&[TokenType::Newline]) {}
                let name = match self.peek() {
                    TokenType::Identifier(n) => {
                        self.advance();
                        n
                    }
                    _ => return Err(self.err("expected name after tuple type", self.peek_span())),
                };
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::Assign]) {
                    return Err(self.err("expected `=` after name", self.peek_span()));
                }
                let value = self.parse_expression()?;
                let span = start.join(value.span);
                return Ok(Statement::new(
                    StatementKind::VariableDeclaration {
                        name,
                        type_annotation: TypeAnnotation::Tuple(types),
                        value,
                    },
                    span,
                ));
            } else {
                return Err(self.err("invalid tuple type syntax", self.peek_span()));
            }
        }

        // -- destructure: dec T x, T y = (...) --
        if matches!(
            self.peek(),
            TokenType::Int
                | TokenType::Float
                | TokenType::Bool
                | TokenType::String
                | TokenType::Byte
                | TokenType::Char
                | TokenType::Fn
                | TokenType::Error
        ) {
            let saved = self.current;
            let first_type = self.parse_type(true)?;
            let first_name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => {
                    self.current = saved;
                    return self.parse_variable_declartion_scalar(start);
                }
            };
            if self.peek() == TokenType::Comma {
                let mut bindings = vec![(first_type, first_name)];
                while self.match_type(&[TokenType::Comma]) {
                    let t = self.parse_type(true)?;
                    let n = match self.peek() {
                        TokenType::Identifier(n) => {
                            self.advance();
                            n
                        }
                        _ => {
                            return Err(
                                self.err("expected name in destructure binding", self.peek_span())
                            );
                        }
                    };
                    bindings.push((t, n));
                }
                if !self.match_type(&[TokenType::Assign]) {
                    return Err(
                        self.err("expected `=` after destructure bindings", self.peek_span())
                    );
                }
                let value = self.parse_expression()?;
                let span = start.join(value.span);
                return Ok(Statement::new(
                    StatementKind::DestructureDeclaration { bindings, value },
                    span,
                ));
            } else {
                if !self.match_type(&[TokenType::Assign]) {
                    return Err(self.err("expected `=` after name", self.peek_span()));
                }
                let value = self.parse_expression()?;
                let span = start.join(value.span);
                return Ok(Statement::new(
                    StatementKind::VariableDeclaration {
                        name: first_name,
                        type_annotation: first_type,
                        value,
                    },
                    span,
                ));
            }
        }

        // -- array: dec array[T] name = [...] --
        if self.match_type(&[TokenType::Array]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            let annoation_type = self.parse_param_type()?;

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
                        return Err(self.err("expected ',' between list items", self.peek_span()));
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                }
                self.match_type(&[TokenType::RightBracket]);
                let span = start.join(self.previous_span());
                return Ok(Statement::new(
                    StatementKind::Array {
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
                    StatementKind::VariableDeclaration {
                        name,
                        type_annotation: TypeAnnotation::Array(Box::new(annoation_type)),
                        value,
                    },
                    span,
                ));
            }
        }

        self.parse_variable_declartion_scalar(start)
    }

    fn parse_variable_declartion_scalar(&mut self, start: Span) -> Result<Statement, Error> {
        let var_type = self.parse_type(true)?;
        while self.match_type(&[TokenType::Newline]) {}
        let name = match self.peek() {
            TokenType::Identifier(n) => {
                self.advance();
                n
            }
            _ => return Err(self.err("expected name after type", self.peek_span())),
        };

        while self.match_type(&[TokenType::Newline]) {}
        if !self.match_type(&[TokenType::Assign]) {
            return Err(self.err("expected `=` after name", self.peek_span()));
        }

        while self.match_type(&[TokenType::Newline]) {}
        let value = self.parse_expression()?;
        let span = start.join(value.span);

        Ok(Statement::new(
            StatementKind::VariableDeclaration {
                name,
                type_annotation: var_type,
                value,
            },
            span,
        ))
    }
}
