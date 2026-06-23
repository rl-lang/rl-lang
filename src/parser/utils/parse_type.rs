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
                _ => return Err(self.err("expected a type", span)),
            },
        })
    }
}
