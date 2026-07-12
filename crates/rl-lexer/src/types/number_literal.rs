//! Integer and float literal scanner.
//!
//! Consumes a run of digits, checks for a `.` to decide between
//! [`TokenType::NumberLiteral`] and [`TokenType::FloatLiteral`], and handles
//! byte literals (`0b` prefix -> [`TokenType::ByteLiteral`]).
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// Scans an integer or float literal starting at the current position.
    ///
    /// A `.` followed by a digit switches to float parsing.
    /// Integers in the range `0..=255` are emitted as [`TokenType::ByteLiteral`],
    /// larger integers as [`TokenType::NumberLiteral`], and decimals as [`TokenType::FloatLiteral`].
    ///
    /// | Input   | Emitted token               |
    /// |---------|-----------------------------|
    /// | `1`     | `ByteLiteral(1)`            |
    /// | `1000`  | `NumberLiteral(1000)`       |
    /// | `3.14`  | `FloatLiteral(3.14)`        |
    pub fn number_literal(&mut self) {
        let mut is_float = false;
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        if is_float {
            let parsed_value: f64 = value.parse().unwrap();
            self.add_token(TokenType::FloatLiteral(parsed_value));
        } else {
            let parsed_value: i64 = value.parse().unwrap();
            self.add_token(TokenType::NumberLiteral(parsed_value));
        }
    }
}
