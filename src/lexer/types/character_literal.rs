use crate::lexer::{tokenizer::Tokenizer, tokentypes::TokenType};
use crate::utils::errors::Error;

impl Tokenizer {
    /// scans a single quoted literal
    ///
    /// only single character are allowed
    /// returns TokenType::CharacterLiteral
    pub fn character_literal(&mut self) -> Result<(), Error> {
        self.advance();

        if self.is_at_end() {
            return Err(self.err("unterminated character literal", self.current_span()));
        }

        let character = self.source[self.current - 1];
        let value: char = if character == '\\' {
            // escape sequence
            if self.is_at_end() {
                return Err(self.err("unterminated character literal", self.current_span()));
            }
            let escaped = self.source[self.current];
            self.advance();
            match escaped {
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                '\\' => '\\',
                '\'' => '\'',
                '0' => '\0',
                _ => {
                    return Err(self.err(
                        format!("unknown escape sequence `\\{}`", escaped),
                        self.current_span(),
                    ));
                }
            }
        } else {
            character
        };

        if self.peek() != '\'' {
            return Err(self.err("unterminated character literal", self.current_span()));
        }

        self.advance();

        self.add_token(TokenType::CharacterLiteral(value));
        Ok(())
    }
}
