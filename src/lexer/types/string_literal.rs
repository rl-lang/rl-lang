use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// scans a double quoted literal
    ///
    /// supports multi-line strings by incrementing the line counter
    /// when hitting \n
    /// returns TokenType::StringLiteral
    pub fn string_literal(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            crate::utils::errors::Error::init(
                "Unterminated String".to_string(),
                Some(self.line),
                Some(crate::utils::errors::ErrorReason::init(
                    crate::utils::errors::Reason::Lexer,
                    None,
                )),
            )
            .print_error();
            return;
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::StringLiteral(value));
    }
}
