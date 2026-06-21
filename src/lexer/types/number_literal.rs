use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// scans a integer or float literal starting at the current position
    ///
    /// '.' enables float parsing
    /// returns either TokenType::NumberLiteral or TokenType::FloatLiteral
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
            if parsed_value >= 0 && parsed_value <= 255 {
                self.add_token(TokenType::ByteLiteral(parsed_value as u8));
                return;
            }
            self.add_token(TokenType::NumberLiteral(parsed_value));
        }
    }
}
