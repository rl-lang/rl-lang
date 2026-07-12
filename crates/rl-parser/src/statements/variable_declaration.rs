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

use std::rc::Rc;

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
                let value_id = self.ast_arena.exprs.get(value);
                let span = start.join(value_id.span);
                return Ok(Statement::new(
                    StatementKind::VariableDeclaration {
                        name,
                        type_annotation: TypeAnnotation::Tuple(Rc::new(types)),
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
                | TokenType::Array
                | TokenType::Map
                | TokenType::Set
                | TokenType::Result
                | TokenType::Identifier(_)
        ) {
            let saved = self.current;
            while self.match_type(&[TokenType::Newline]) {}
            let first_type = self.parse_type(true)?;
            while self.match_type(&[TokenType::Newline]) {}
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
            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::Comma {
                let mut bindings = vec![(first_type, first_name)];
                while self.match_type(&[TokenType::Comma]) {
                    while self.match_type(&[TokenType::Newline]) {}
                    let t = self.parse_type(true)?;
                    while self.match_type(&[TokenType::Newline]) {}
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
                while self.match_type(&[TokenType::Newline]) {}
                if !self.match_type(&[TokenType::Assign]) {
                    return Err(
                        self.err("expected `=` after destructure bindings", self.peek_span())
                    );
                }
                while self.match_type(&[TokenType::Newline]) {}
                let value = self.parse_expression()?;
                let value_id = self.ast_arena.exprs.get(value);
                let span = start.join(value_id.span);
                return Ok(Statement::new(
                    StatementKind::DestructureDeclaration { bindings, value },
                    span,
                ));
            } else {
                self.current = saved;
                return self.parse_variable_declartion_single(start);
            }
        }

        self.parse_variable_declartion_single(start)
    }

    /// Parses a single (non-destructure) `dec` declaration: map, set, array,
    /// or scalar. Assumes the destructure/tuple-type possibilities have
    /// already been ruled out (or the parser position has been rewound past
    /// a failed destructure attempt) by the caller.
    fn parse_variable_declartion_single(&mut self, start: Span) -> Result<Statement, Error> {
        // -- map: dec map[K,V] name = {...} --
        if self.match_type(&[TokenType::Map]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            while self.match_type(&[TokenType::Newline]) {}
            let key_type = self.parse_param_type()?;

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Comma]) {
                return Err(self.err(
                    "expected `,` between map key and value types",
                    self.peek_span(),
                ));
            }

            while self.match_type(&[TokenType::Newline]) {}
            let value_type = self.parse_param_type()?;

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after type", self.peek_span()));
            }

            while self.match_type(&[TokenType::Newline]) {}
            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected name after map type", self.peek_span())),
            };

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Assign]) {
                return Err(self.err("expected `=` after name", self.peek_span()));
            }

            let annoation_type = TypeAnnotation::Map(Box::new(key_type), Box::new(value_type));

            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::LeftBrace {
                self.advance();
                let mut entries = Vec::new();
                while self.match_type(&[TokenType::Newline]) {}

                while self.peek() != TokenType::RightBrace {
                    let key = self.parse_expression()?;
                    while self.match_type(&[TokenType::Newline]) {}
                    if !self.match_type(&[TokenType::Colon]) {
                        return Err(self.err("expected `:` after map key", self.peek_span()));
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                    let value = self.parse_expression()?;
                    entries.push((key, value));
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightBrace {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected ',' between map entries", self.peek_span()));
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                }
                self.match_type(&[TokenType::RightBrace]);
                let span = start.join(self.previous_span());
                return Ok(Statement::new(
                    StatementKind::Map {
                        name,
                        type_annotation: annoation_type,
                        entries,
                    },
                    span,
                ));
            } else {
                let value = self.parse_expression()?;
                let value_id = self.ast_arena.exprs.get(value);
                let span = start.join(value_id.span);
                return Ok(Statement::new(
                    StatementKind::VariableDeclaration {
                        name,
                        type_annotation: annoation_type,
                        value,
                    },
                    span,
                ));
            }
        }

        // -- set: dec set[T] name = {...} --
        if self.match_type(&[TokenType::Set]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            while self.match_type(&[TokenType::Newline]) {}
            let annoation_type = self.parse_param_type()?;
            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after type", self.peek_span()));
            }

            while self.match_type(&[TokenType::Newline]) {}
            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected name after set type", self.peek_span())),
            };

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Assign]) {
                return Err(self.err("expected `=` after name", self.peek_span()));
            }

            while self.match_type(&[TokenType::Newline]) {}
            if self.peek() == TokenType::LeftBrace {
                self.advance();
                let mut items = Vec::new();
                while self.match_type(&[TokenType::Newline]) {}

                while self.peek() != TokenType::RightBrace {
                    let value = self.parse_expression()?;
                    items.push(value);
                    while self.match_type(&[TokenType::Newline]) {}
                    if self.peek() == TokenType::RightBrace {
                        break;
                    }
                    if !self.match_type(&[TokenType::Comma]) {
                        return Err(self.err("expected `,` between set items", self.peek_span()));
                    }
                    while self.match_type(&[TokenType::Newline]) {}
                }
                self.match_type(&[TokenType::RightBrace]);
                let span = start.join(self.previous_span());
                return Ok(Statement::new(
                    StatementKind::Set {
                        name,
                        type_annotation: annoation_type,
                        items,
                    },
                    span,
                ));
            } else {
                while self.match_type(&[TokenType::Newline]) {}
                let value = self.parse_expression()?;
                let value_id = self.ast_arena.exprs.get(value);
                let span = start.join(value_id.span);
                return Ok(Statement::new(
                    StatementKind::ConstantDeclaration {
                        name,
                        type_annotation: TypeAnnotation::Set(Box::new(annoation_type)),
                        value,
                    },
                    span,
                ));
            }
        }

        // -- array: dec array[T] name = [...] --
        if self.match_type(&[TokenType::Array]) && self.peek() == TokenType::LeftBracket {
            self.advance();
            while self.match_type(&[TokenType::Newline]) {}
            let annoation_type = self.parse_param_type()?;

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::RightBracket]) {
                return Err(self.err("expected `]` after type", self.peek_span()));
            }

            while self.match_type(&[TokenType::Newline]) {}
            let name = match self.peek() {
                TokenType::Identifier(n) => {
                    self.advance();
                    n
                }
                _ => return Err(self.err("expected name after array type", self.peek_span())),
            };

            while self.match_type(&[TokenType::Newline]) {}
            if !self.match_type(&[TokenType::Assign]) {
                return Err(self.err("expected `=` after name", self.peek_span()));
            }

            while self.match_type(&[TokenType::Newline]) {}
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
                while self.match_type(&[TokenType::Newline]) {}
                let value = self.parse_expression()?;
                let value_id = self.ast_arena.exprs.get(value);
                let span = start.join(value_id.span);
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
        let value_id = self.ast_arena.exprs.get(value);
        let span = start.join(value_id.span);

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
