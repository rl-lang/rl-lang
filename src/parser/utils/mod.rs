mod parse_type;
use std::rc::Rc;

use crate::{
    ast::statements::TypeAnnotation,
    lexer::tokentypes::TokenType,
    parser::parser_logic::Parser,
    utils::{errors::Error, span::Span},
};

impl Parser {
    /// Returns the [`Span`] of the token at the current read head without consuming it.
    pub fn peek_span(&self) -> Span {
        self.tokens[self.current].span
    }

    /// Returns the [`Span`] of the most recently consumed token.
    ///
    /// Returns the span of `tokens[0]` when called before any [`advance`].
    ///
    /// [`advance`]: Parser::advance
    pub fn previous_span(&self) -> Span {
        if self.current == 0 {
            self.tokens[0].span
        } else {
            self.tokens[self.current - 1].span
        }
    }

    /// Returns `true` when the read head sits on [`TokenType::Eof`], indicating
    /// the end of the token stream.
    pub fn is_at_end(&self) -> bool {
        #[cfg(feature = "debug")]
        if matches!(self.peek(), TokenType::Eof) {
            log::debug!("countered token [TokenType::Eof] indicating end of tokens for the file");
        }
        matches!(self.peek(), TokenType::Eof)
    }

    /// Advances the read head by one token, unless already at [`TokenType::Eof`].
    pub fn advance(&mut self) {
        if !self.is_at_end() {
            self.current += 1;
            #[cfg(feature = "debug")]
            log::debug!("advancing the parser current token: {}", self.current);
        }
    }

    /// Returns the [`TokenType`] at the current read head without consuming it.
    pub fn peek(&self) -> TokenType {
        #[cfg(feature = "debug")]
        log::debug!(
            "returning current token: [{:?}]",
            &self.tokens[self.current].token
        );
        self.tokens[self.current].token.clone()
    }

    /// Returns the [`TokenType`] of the most recently consumed token.
    ///
    /// # Panics
    /// Panics if called before any token has been consumed (`current == 0`).
    pub fn previous(&self) -> TokenType {
        #[cfg(feature = "debug")]
        log::debug!(
            "returning previous token: [{:?}]",
            &self.tokens[self.current - 1].token
        );
        self.tokens[self.current - 1].token.clone()
    }

    /// Returns `true` if the token at the read head matches `token_type`.
    ///
    /// Literal-carrying variants (`NumberLiteral`, `StringLiteral`, etc.) are
    /// matched by variant tag only - the inner value is ignored. Does **not**
    /// consume the token.
    pub fn check(&self, token_type: &TokenType) -> bool {
        let current = self.peek();
        match (token_type, &current) {
            (TokenType::NumberLiteral(_), TokenType::NumberLiteral(_)) => true,
            (TokenType::ByteLiteral(_), TokenType::ByteLiteral(_)) => true,
            (TokenType::StringLiteral(_), TokenType::StringLiteral(_)) => true,
            (TokenType::FloatLiteral(_), TokenType::FloatLiteral(_)) => true,
            (TokenType::BoolLiteral(_), TokenType::BoolLiteral(_)) => true,
            (TokenType::CharacterLiteral(_), TokenType::CharacterLiteral(_)) => true,
            (TokenType::Identifier(_), TokenType::Identifier(_)) => true,
            _ => token_type == &current,
        }
    }

    /// Tries each variant in `types` against the current token via [`check`].
    ///
    /// On the first match the token is consumed via [`advance`] and `true` is
    /// returned. Returns `false` without advancing if nothing matches.
    ///
    /// [`check`]: Parser::check
    /// [`advance`]: Parser::advance
    pub fn match_type(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                #[cfg(feature = "debug")]
                log::debug!("Token {:?} matched one in [{:?}]", self.peek(), types);
                self.advance();
                return true;
            }
        }
        #[cfg(feature = "debug")]
        log::debug!("Token {:?} did not match any in [{:?}]", self.peek(), types);
        false
    }

    /// Parses a single type keyword (or `array[T]`) as a function/lambda parameter
    /// type annotation.
    ///
    /// Called when a type is expected in a parameter list position. For the
    /// `array` keyword the inner element type is parsed recursively via
    /// [`nested_array_type`].
    ///
    /// # Errors
    /// Returns an error if the current token is not a recognised type keyword.
    ///
    /// [`nested_array_type`]: Parser::nested_array_type
    pub fn parse_param_type(&mut self) -> Result<TypeAnnotation, Error> {
        if matches!(self.peek(), TokenType::Array) {
            self.advance();
            return Ok(*self.nested_array_type()?);
        }
        if matches!(self.peek(), TokenType::Map) {
            self.advance();
            return Ok(*self.nested_map_type()?);
        }
        if matches!(self.peek(), TokenType::Set) {
            self.advance();
            return Ok(*self.set_type()?);
        }
        match self.peek() {
            TokenType::Int => {
                self.advance();
                Ok(TypeAnnotation::Int)
            }
            TokenType::Byte => {
                self.advance();
                Ok(TypeAnnotation::Byte)
            }
            TokenType::Float => {
                self.advance();
                Ok(TypeAnnotation::Float)
            }
            TokenType::Bool => {
                self.advance();
                Ok(TypeAnnotation::Bool)
            }
            TokenType::String => {
                self.advance();
                Ok(TypeAnnotation::String)
            }
            TokenType::Char => {
                self.advance();
                Ok(TypeAnnotation::Char)
            }
            TokenType::Fn => {
                self.advance();
                Ok(TypeAnnotation::Fn)
            }
            TokenType::LeftParen => {
                self.advance();
                let mut inner = vec![];
                loop {
                    inner.push(self.parse_param_type()?);
                    if !self.match_type(&[TokenType::Comma]) {
                        break;
                    }
                }
                if !self.match_type(&[TokenType::RightParen]) {
                    return Err(self.err("expected `)` after tuple types", self.peek_span()));
                }
                Ok(TypeAnnotation::Tuple(Rc::new(inner)))
            }
            TokenType::Error => {
                self.advance();
                Ok(TypeAnnotation::Error)
            }
            TokenType::Result => {
                self.advance();
                if !self.match_type(&[TokenType::LeftBracket]) {
                    return Err(self.err("expected `[` after `result`", self.peek_span()));
                }
                let inner = self.parse_param_type()?;
                if !self.match_type(&[TokenType::RightBracket]) {
                    return Err(self.err("expected `]` after result inner type", self.peek_span()));
                }
                Ok(TypeAnnotation::Result(Box::new(inner)))
            }
            TokenType::Null => {
                self.advance();
                Ok(TypeAnnotation::Null)
            }

            TokenType::Identifier(name) => {
                self.advance();
                if self.tag_names.contains(&name) {
                    Ok(TypeAnnotation::Enum(name))
                } else {
                    Ok(TypeAnnotation::Record(name))
                }
            }

            _ => Err(self.err("expected type", self.peek_span())),
        }
    }

    /// Parses the `[T]` suffix of an `array[T]` type annotation, returning a
    /// boxed [`TypeAnnotation::Array`].
    ///
    /// Expects the `array` keyword to have already been consumed. Reads
    /// `[`, the inner element type (via [`parse_param_type`]), then `]`.
    ///
    /// # Errors
    /// Returns an error if `[` or `]` are missing, or if the inner type is invalid.
    ///
    /// [`parse_param_type`]: Parser::parse_param_type
    pub fn nested_array_type(&mut self) -> Result<Box<TypeAnnotation>, Error> {
        match self.peek() {
            TokenType::LeftBracket => {
                self.advance();
                let a = self.parse_param_type()?;
                match self.peek() {
                    TokenType::RightBracket => {
                        self.advance();
                        Ok(Box::new(TypeAnnotation::Array(Box::new(a))))
                    }
                    _ => Err(self.err("expected ']'", self.peek_span())),
                }
            }

            _ => Err(self.err("expected '['", self.peek_span())),
        }
    }

    pub fn nested_map_type(&mut self) -> Result<Box<TypeAnnotation>, Error> {
        match self.peek() {
            TokenType::LeftBracket => {
                self.advance();
                let key = self.parse_param_type()?;
                if !self.match_type(&[TokenType::Comma]) {
                    return Err(self.err(
                        "expected `,` between map key and value types",
                        self.peek_span(),
                    ));
                }
                let value = self.parse_param_type()?;
                match self.peek() {
                    TokenType::RightBracket => {
                        self.advance();
                        Ok(Box::new(TypeAnnotation::Map(
                            Box::new(key),
                            Box::new(value),
                        )))
                    }
                    _ => Err(self.err("expected ']'", self.peek_span())),
                }
            }

            _ => Err(self.err("expected '['", self.peek_span())),
        }
    }

    pub fn set_type(&mut self) -> Result<Box<TypeAnnotation>, Error> {
        match self.peek() {
            TokenType::LeftBracket => {
                self.advance();
                let a = self.parse_param_type()?;
                match self.peek() {
                    TokenType::RightBracket => {
                        self.advance();
                        Ok(Box::new(TypeAnnotation::Set(Box::new(a))))
                    }
                    _ => Err(self.err("expected ']'", self.peek_span())),
                }
            }

            _ => Err(self.err("expected '['", self.peek_span())),
        }
    }
}
