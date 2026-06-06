use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use crate::utils::errors::Error;

impl Tokenizer {
    /// scans a double quoted literal
    ///
    /// supports multi-line strings by incrementing the line counter
    /// when hitting \n
    /// returns TokenType::StringLiteral
    pub fn string_literal(&mut self) -> Result<(), Error> {
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(self.err("unterminated string", self.current_span()));
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenType::StringLiteral(value));
        Ok(())
    }
}
