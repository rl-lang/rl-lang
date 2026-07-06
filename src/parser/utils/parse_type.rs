use std::rc::Rc;

use crate::{
    ast::statements::TypeAnnotation, lexer::tokentypes::TokenType, parser::parser_logic::Parser,
    utils::errors::Error,
};

impl Parser {
    /// Parses a type keyword into a [`TypeAnnotation`].
    ///
    /// The `is_mut` flag controls which annotation variant is produced:
    ///
    /// | `is_mut` | produced variant |
    /// |---|---|
    /// | `true` | `Int`, `Float`, `Bool`, `String`, `Byte`, `Char`, `Fn`, `Array(T)` |
    /// | `false` | `CInt`, `CFloat`, `CBool`, `CString`, `CByte`, `CChar`, `Fn`, `CArray(T)` |
    ///
    /// The `C*` variants represent constant (immutable) bindings and are used by
    /// the `const` declaration path.
    ///
    /// For `array[T]` the inner element type is parsed recursively.
    ///
    /// # Errors
    /// Returns an error if the current token is not a recognised type keyword.
    pub fn parse_type(&mut self, is_mut: bool) -> Result<TypeAnnotation, Error> {
        let span = self.peek_span();
        Ok(match is_mut {
            true => match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::Int
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
                TokenType::Byte => {
                    self.advance();
                    TypeAnnotation::Byte
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::Char
                }
                TokenType::Fn => {
                    self.advance();
                    TypeAnnotation::Fn
                }
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(true)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Array(Box::new(inner))
                }
                TokenType::LeftParen => {
                    self.advance();
                    let mut inner = vec![];
                    while !self.match_type(&[TokenType::RightParen]) {
                        let item_type = self.parse_type(true)?;
                        inner.push(item_type);
                        if !self.match_type(&[TokenType::Comma]) {
                            return Err(
                                self.err("expected `,` between tuple types", self.peek_span())
                            );
                        }
                    }
                    TypeAnnotation::Tuple(Rc::new(inner))
                }
                TokenType::Result => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(true)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::Result(Box::new(inner))
                }
                TokenType::Error => {
                    self.advance();
                    TypeAnnotation::Error
                }
                TokenType::Identifier(name) => {
                    self.advance();
                    if self.tag_names.contains(&name) {
                        TypeAnnotation::Enum(name)
                    } else {
                        TypeAnnotation::Record(name)
                    }
                }
                _ => return Err(self.err("expected a type", span)),
            },
            false => match self.peek() {
                TokenType::Int => {
                    self.advance();
                    TypeAnnotation::CInt
                }
                TokenType::Float => {
                    self.advance();
                    TypeAnnotation::CFloat
                }
                TokenType::Bool => {
                    self.advance();
                    TypeAnnotation::CBool
                }
                TokenType::String => {
                    self.advance();
                    TypeAnnotation::CString
                }
                TokenType::Byte => {
                    self.advance();
                    TypeAnnotation::CByte
                }
                TokenType::Char => {
                    self.advance();
                    TypeAnnotation::CChar
                }
                TokenType::Fn => {
                    self.advance();
                    TypeAnnotation::Fn
                }
                TokenType::Array => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(false)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::CArray(Box::new(inner))
                }
                TokenType::LeftParen => {
                    self.advance();
                    let mut inner = vec![];
                    while !self.match_type(&[TokenType::RightParen]) {
                        let item_type = self.parse_type(false)?;
                        inner.push(item_type);
                        if !self.match_type(&[TokenType::Comma]) {
                            return Err(
                                self.err("expected `,` between tuple types", self.peek_span())
                            );
                        }
                    }
                    TypeAnnotation::CTuple(Rc::new(inner))
                }
                TokenType::Result => {
                    self.advance();
                    self.match_type(&[TokenType::LeftBracket]);
                    let inner = self.parse_type(false)?;
                    self.match_type(&[TokenType::RightBracket]);
                    TypeAnnotation::CResult(Box::new(inner))
                }
                TokenType::Error => {
                    self.advance();
                    TypeAnnotation::CError
                }
                TokenType::Identifier(name) => {
                    self.advance();
                    if self.tag_names.contains(&name) {
                        TypeAnnotation::CEnum(name)
                    } else {
                        TypeAnnotation::CRecord(name)
                    }
                }
                _ => return Err(self.err("expected a type", span)),
            },
        })
    }
}
