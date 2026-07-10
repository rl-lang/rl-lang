//! Low-level character cursor primitives used across all sub-scanners.
//!
//! Provides `is_at_end`, `peek`, `advance`, and similar helpers that operate
//! directly on the [`Tokenizer`]'s byte position.
use super::super::tokenizer::Tokenizer;
use super::super::tokentypes::{Token, TokenType};

impl Tokenizer {
    /// Returns `true` if all characters have been consumed.
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Returns the current character without consuming it.
    ///
    /// Returns `'\0'` if at end of source.
    pub fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Returns the character after the current one without consuming it.
    ///
    /// Returns `'\0'` if at end of source.
    pub fn peek_next(&mut self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    /// Consumes and returns the current character, advancing the cursor.
    ///
    /// Returns `'\0'` if at end of source.
    pub fn advance(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let character = self.source[self.current];
        self.current += 1;
        character
    }

    /// Constructs a [`Token`] from `start..current` and pushes it onto the token list.
    pub fn add_token(&mut self, tokentype: TokenType) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        let span = self.current_span();
        let mut tok = Token::new(tokentype, lexeme, self.line, span);
        tok.leading_trivia = std::mem::take(&mut self.pending_trivia);
        self.tokens.push(tok);
    }
}
